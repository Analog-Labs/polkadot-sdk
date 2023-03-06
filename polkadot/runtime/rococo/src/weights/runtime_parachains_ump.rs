// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_parachains::ump`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::ump
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/runtime_parachains_ump.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::ump`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::ump::WeightInfo for WeightInfo<T> {
	/// The range of component `s` is `[0, 51200]`.
	fn process_upward_message(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_149 nanoseconds.
		Weight::from_parts(2_328_083, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 13
			.saturating_add(Weight::from_parts(1_853, 0).saturating_mul(s.into()))
	}
	/// Storage: Ump NeedsDispatch (r:1 w:1)
	/// Proof Skipped: Ump NeedsDispatch (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	/// Proof Skipped: Ump NextDispatchRoundStartWith (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Ump RelayDispatchQueues (r:0 w:1)
	/// Proof Skipped: Ump RelayDispatchQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump RelayDispatchQueueSize (r:0 w:1)
	/// Proof Skipped: Ump RelayDispatchQueueSize (max_values: None, max_size: None, mode: Measured)
	fn clean_ump_after_outgoing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `234`
		//  Estimated: `1926`
		// Minimum execution time: 8_704 nanoseconds.
		Weight::from_parts(8_997_000, 0)
			.saturating_add(Weight::from_parts(0, 1926))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Ump Overweight (r:1 w:1)
	/// Proof Skipped: Ump Overweight (max_values: None, max_size: None, mode: Measured)
	/// Storage: Ump CounterForOverweight (r:1 w:1)
	/// Proof: Ump CounterForOverweight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn service_overweight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3193`
		// Minimum execution time: 22_612 nanoseconds.
		Weight::from_parts(22_947_000, 0)
			.saturating_add(Weight::from_parts(0, 3193))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}