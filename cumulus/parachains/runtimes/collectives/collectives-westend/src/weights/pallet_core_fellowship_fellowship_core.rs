// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_core_fellowship`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `623e9e4b814e`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/production/wbuild/collectives-westend-runtime/collectives_westend_runtime.wasm
// --pallet=pallet_core_fellowship
// --header=/__w/polkadot-sdk/polkadot-sdk/cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/collectives/collectives-westend/src/weights
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096
// --no-storage-info
// --no-min-squares
// --no-median-slopes

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_core_fellowship`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_core_fellowship::WeightInfo for WeightInfo<T> {
	/// Storage: `FellowshipCore::Params` (r:0 w:1)
	/// Proof: `FellowshipCore::Params` (`max_values`: Some(1), `max_size`: Some(368), added: 863, mode: `MaxEncodedLen`)
	fn set_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_115_000 picoseconds.
		Weight::from_parts(9_523_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipCore::Params` (r:1 w:1)
	/// Proof: `FellowshipCore::Params` (`max_values`: Some(1), `max_size`: Some(368), added: 863, mode: `MaxEncodedLen`)
	fn set_partial_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `504`
		//  Estimated: `1853`
		// Minimum execution time: 18_294_000 picoseconds.
		Weight::from_parts(18_942_000, 0)
			.saturating_add(Weight::from_parts(0, 1853))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::Members` (r:1 w:1)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Params` (r:1 w:0)
	/// Proof: `FellowshipCore::Params` (`max_values`: Some(1), `max_size`: Some(368), added: 863, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:1)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IdToIndex` (r:1 w:1)
	/// Proof: `FellowshipCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::MemberEvidence` (r:1 w:1)
	/// Proof: `FellowshipCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IndexToId` (r:0 w:1)
	/// Proof: `FellowshipCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn bump_offboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66535`
		//  Estimated: `69046`
		// Minimum execution time: 152_823_000 picoseconds.
		Weight::from_parts(158_737_000, 0)
			.saturating_add(Weight::from_parts(0, 69046))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::Members` (r:1 w:1)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Params` (r:1 w:0)
	/// Proof: `FellowshipCore::Params` (`max_values`: Some(1), `max_size`: Some(368), added: 863, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:1)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IdToIndex` (r:1 w:1)
	/// Proof: `FellowshipCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::MemberEvidence` (r:1 w:1)
	/// Proof: `FellowshipCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IndexToId` (r:0 w:1)
	/// Proof: `FellowshipCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn bump_demote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66645`
		//  Estimated: `69046`
		// Minimum execution time: 157_605_000 picoseconds.
		Weight::from_parts(162_341_000, 0)
			.saturating_add(Weight::from_parts(0, 69046))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `FellowshipCollective::Members` (r:1 w:0)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn set_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493`
		//  Estimated: `3514`
		// Minimum execution time: 25_194_000 picoseconds.
		Weight::from_parts(26_262_000, 0)
			.saturating_add(Weight::from_parts(0, 3514))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::Members` (r:1 w:1)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:1)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IndexToId` (r:0 w:1)
	/// Proof: `FellowshipCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IdToIndex` (r:0 w:1)
	/// Proof: `FellowshipCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn induct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `251`
		//  Estimated: `3514`
		// Minimum execution time: 35_479_000 picoseconds.
		Weight::from_parts(36_360_000, 0)
			.saturating_add(Weight::from_parts(0, 3514))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `FellowshipCollective::Members` (r:1 w:1)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Params` (r:1 w:0)
	/// Proof: `FellowshipCore::Params` (`max_values`: Some(1), `max_size`: Some(368), added: 863, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:1)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::MemberEvidence` (r:1 w:1)
	/// Proof: `FellowshipCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IndexToId` (r:0 w:1)
	/// Proof: `FellowshipCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IdToIndex` (r:0 w:1)
	/// Proof: `FellowshipCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn promote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66188`
		//  Estimated: `69046`
		// Minimum execution time: 147_993_000 picoseconds.
		Weight::from_parts(153_943_000, 0)
			.saturating_add(Weight::from_parts(0, 69046))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `FellowshipCollective::Members` (r:1 w:1)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:9 w:9)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::MemberEvidence` (r:1 w:1)
	/// Proof: `FellowshipCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IndexToId` (r:0 w:9)
	/// Proof: `FellowshipCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::IdToIndex` (r:0 w:9)
	/// Proof: `FellowshipCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 9]`.
	/// The range of component `r` is `[1, 9]`.
	fn promote_fast(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66101`
		//  Estimated: `69046 + r * (2489 ±0)`
		// Minimum execution time: 138_444_000 picoseconds.
		Weight::from_parts(125_440_035, 0)
			.saturating_add(Weight::from_parts(0, 69046))
			// Standard Error: 55_452
			.saturating_add(Weight::from_parts(19_946_954, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2489).saturating_mul(r.into()))
	}
	/// Storage: `FellowshipCollective::Members` (r:1 w:0)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::MemberEvidence` (r:0 w:1)
	/// Proof: `FellowshipCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	fn offboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398`
		//  Estimated: `3514`
		// Minimum execution time: 27_392_000 picoseconds.
		Weight::from_parts(28_134_000, 0)
			.saturating_add(Weight::from_parts(0, 3514))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::Members` (r:1 w:0)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	fn import() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `418`
		//  Estimated: `3514`
		// Minimum execution time: 23_523_000 picoseconds.
		Weight::from_parts(24_046_000, 0)
			.saturating_add(Weight::from_parts(0, 3514))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::Members` (r:1 w:0)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	fn import_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `418`
		//  Estimated: `3514`
		// Minimum execution time: 23_369_000 picoseconds.
		Weight::from_parts(24_088_000, 0)
			.saturating_add(Weight::from_parts(0, 3514))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipCollective::Members` (r:1 w:0)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::Member` (r:1 w:1)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::MemberEvidence` (r:1 w:1)
	/// Proof: `FellowshipCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66100`
		//  Estimated: `69046`
		// Minimum execution time: 127_137_000 picoseconds.
		Weight::from_parts(131_638_000, 0)
			.saturating_add(Weight::from_parts(0, 69046))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipCore::Member` (r:1 w:0)
	/// Proof: `FellowshipCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCore::MemberEvidence` (r:1 w:1)
	/// Proof: `FellowshipCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	fn submit_evidence() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `184`
		//  Estimated: `69046`
		// Minimum execution time: 103_212_000 picoseconds.
		Weight::from_parts(105_488_000, 0)
			.saturating_add(Weight::from_parts(0, 69046))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
