// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

use crate::ethereum_client::{self, EthereumConnectionParams};
use crate::ethereum_types::{EthereumHeaderId, EthereumHeadersSyncPipeline, Header, QueuedEthereumHeader, Receipt};
use crate::substrate_client::{self, SubstrateConnectionParams, SubstrateSigningParams};
use crate::sync::{HeadersSyncParams, TargetTransactionMode};
use crate::sync_loop::{SourceClient, TargetClient};
use futures::future::FutureExt;
use std::{future::Future, pin::Pin};
use web3::types::H256;

/// Interval (in ms) at which we check new Ethereum headers when we are synced/almost synced.
const ETHEREUM_TICK_INTERVAL_MS: u64 = 10_000;
/// Interval (in ms) at which we check new Substrate blocks.
const SUBSTRATE_TICK_INTERVAL_MS: u64 = 5_000;
/// Max number of headers in single submit transaction.
const MAX_HEADERS_IN_SINGLE_SUBMIT: usize = 32;
/// Max total size of headers in single submit transaction. This only affects signed
/// submissions, when several headers are submitted at once. 4096 is the maximal **expected**
/// size of the Ethereum header + transactions receipts (if they're required).
const MAX_HEADERS_SIZE_IN_SINGLE_SUBMIT: usize = MAX_HEADERS_IN_SINGLE_SUBMIT * 4096;
/// Max Ethereum headers we want to have in all 'before-submitted' states.
const MAX_FUTURE_HEADERS_TO_DOWNLOAD: usize = 128;
/// Max Ethereum headers count we want to have in 'submitted' state.
const MAX_SUBMITTED_HEADERS: usize = 128;
/// Max depth of in-memory headers in all states. Past this depth they will be forgotten (pruned).
const PRUNE_DEPTH: u32 = 4096;

/// Ethereum synchronization parameters.
pub struct EthereumSyncParams {
	/// Ethereum connection params.
	pub eth: EthereumConnectionParams,
	/// Substrate connection params.
	pub sub: SubstrateConnectionParams,
	/// Substrate signing params.
	pub sub_sign: SubstrateSigningParams,
	/// Synchronization parameters.
	pub sync_params: HeadersSyncParams,
}

impl Default for EthereumSyncParams {
	fn default() -> Self {
		EthereumSyncParams {
			eth: Default::default(),
			sub: Default::default(),
			sub_sign: Default::default(),
			sync_params: HeadersSyncParams {
				max_future_headers_to_download: MAX_FUTURE_HEADERS_TO_DOWNLOAD,
				max_headers_in_submitted_status: MAX_SUBMITTED_HEADERS,
				max_headers_in_single_submit: MAX_HEADERS_IN_SINGLE_SUBMIT,
				max_headers_size_in_single_submit: MAX_HEADERS_SIZE_IN_SINGLE_SUBMIT,
				prune_depth: PRUNE_DEPTH,
				target_tx_mode: TargetTransactionMode::Signed,
			},
		}
	}
}

/// Ethereum client as headers source.
struct EthereumHeadersSource {
	/// Ethereum node client.
	client: ethereum_client::Client,
}

impl SourceClient<EthereumHeadersSyncPipeline> for EthereumHeadersSource {
	type Error = ethereum_client::Error;
	type BestBlockNumberFuture = Pin<Box<dyn Future<Output = (Self, Result<u64, Self::Error>)>>>;
	type HeaderByHashFuture = Pin<Box<dyn Future<Output = (Self, Result<Header, Self::Error>)>>>;
	type HeaderByNumberFuture = Pin<Box<dyn Future<Output = (Self, Result<Header, Self::Error>)>>>;
	type HeaderExtraFuture =
		Pin<Box<dyn Future<Output = (Self, Result<(EthereumHeaderId, Vec<Receipt>), Self::Error>)>>>;

	fn best_block_number(self) -> Self::BestBlockNumberFuture {
		ethereum_client::best_block_number(self.client)
			.map(|(client, result)| (EthereumHeadersSource { client }, result))
			.boxed()
	}

	fn header_by_hash(self, hash: H256) -> Self::HeaderByHashFuture {
		ethereum_client::header_by_hash(self.client, hash)
			.map(|(client, result)| (EthereumHeadersSource { client }, result))
			.boxed()
	}

	fn header_by_number(self, number: u64) -> Self::HeaderByNumberFuture {
		ethereum_client::header_by_number(self.client, number)
			.map(|(client, result)| (EthereumHeadersSource { client }, result))
			.boxed()
	}

	fn header_extra(self, id: EthereumHeaderId, header: &Header) -> Self::HeaderExtraFuture {
		ethereum_client::transactions_receipts(self.client, id, header.transactions.clone())
			.map(|(client, result)| (EthereumHeadersSource { client }, result))
			.boxed()
	}
}

/// Substrate client as Ethereum headers target.
struct SubstrateHeadersTarget {
	/// Substrate node client.
	client: substrate_client::Client,
	/// Whether we want to submit signed (true), or unsigned (false) transactions.
	sign_transactions: bool,
	/// Substrate signing params.
	sign_params: SubstrateSigningParams,
}

impl TargetClient<EthereumHeadersSyncPipeline> for SubstrateHeadersTarget {
	type Error = substrate_client::Error;
	type BestHeaderIdFuture = Pin<Box<dyn Future<Output = (Self, Result<EthereumHeaderId, Self::Error>)>>>;
	type IsKnownHeaderFuture = Pin<Box<dyn Future<Output = (Self, Result<(EthereumHeaderId, bool), Self::Error>)>>>;
	type RequiresExtraFuture = Pin<Box<dyn Future<Output = (Self, Result<(EthereumHeaderId, bool), Self::Error>)>>>;
	type SubmitHeadersFuture = Pin<Box<dyn Future<Output = (Self, Result<Vec<EthereumHeaderId>, Self::Error>)>>>;

	fn best_header_id(self) -> Self::BestHeaderIdFuture {
		let (sign_transactions, sign_params) = (self.sign_transactions, self.sign_params);
		substrate_client::best_ethereum_block(self.client)
			.map(move |(client, result)| {
				(
					SubstrateHeadersTarget {
						client,
						sign_transactions,
						sign_params,
					},
					result,
				)
			})
			.boxed()
	}

	fn is_known_header(self, id: EthereumHeaderId) -> Self::IsKnownHeaderFuture {
		let (sign_transactions, sign_params) = (self.sign_transactions, self.sign_params);
		substrate_client::ethereum_header_known(self.client, id)
			.map(move |(client, result)| {
				(
					SubstrateHeadersTarget {
						client,
						sign_transactions,
						sign_params,
					},
					result,
				)
			})
			.boxed()
	}

	fn requires_extra(self, header: &QueuedEthereumHeader) -> Self::RequiresExtraFuture {
		// we can minimize number of receipts_check calls by checking header
		// logs bloom here, but it may give us false positives (when authorities
		// source is contract, we never need any logs)
		let (sign_transactions, sign_params) = (self.sign_transactions, self.sign_params);
		substrate_client::ethereum_receipts_required(self.client, header.clone())
			.map(move |(client, result)| {
				(
					SubstrateHeadersTarget {
						client,
						sign_transactions,
						sign_params,
					},
					result,
				)
			})
			.boxed()
	}

	fn submit_headers(self, headers: Vec<QueuedEthereumHeader>) -> Self::SubmitHeadersFuture {
		let (sign_transactions, sign_params) = (self.sign_transactions, self.sign_params);
		substrate_client::submit_ethereum_headers(self.client, sign_params.clone(), headers, sign_transactions)
			.map(move |(client, result)| {
				(
					SubstrateHeadersTarget {
						client,
						sign_transactions,
						sign_params,
					},
					result,
				)
			})
			.boxed()
	}
}

/// Run Ethereum headers synchronization.
pub fn run(params: EthereumSyncParams) {
	let eth_client = ethereum_client::client(params.eth);
	let sub_client = substrate_client::client(params.sub);

	let sign_sub_transactions = match params.sync_params.target_tx_mode {
		TargetTransactionMode::Signed | TargetTransactionMode::Backup => true,
		TargetTransactionMode::Unsigned => false,
	};

	crate::sync_loop::run(
		EthereumHeadersSource { client: eth_client },
		ETHEREUM_TICK_INTERVAL_MS,
		SubstrateHeadersTarget {
			client: sub_client,
			sign_transactions: sign_sub_transactions,
			sign_params: params.sub_sign,
		},
		SUBSTRATE_TICK_INTERVAL_MS,
		params.sync_params,
	);
}