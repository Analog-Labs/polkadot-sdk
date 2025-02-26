// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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


//! Autogenerated weights for `pallet_election_provider_multi_block`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-02-25, STEPS: `5`, REPEAT: `5`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/substrate-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_election_provider_multi_block
// --extrinsic
// all
// --steps
// 5
// --repeat
// 5
// --template
// substrate/.maintain/frame-weight-template.hbs
// --heap-pages
// 65000
// --default-pov-mode
// measured
// --output
// ../measured

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_election_provider_multi_block`.
pub trait WeightInfo {
	fn on_initialize_nothing() -> Weight;
	fn on_initialize_into_snapshot_msp() -> Weight;
	fn on_initialize_into_snapshot_rest() -> Weight;
	fn on_initialize_into_signed() -> Weight;
	fn on_initialize_into_signed_validation() -> Weight;
	fn on_initialize_into_unsigned() -> Weight;
	fn export_non_terminal() -> Weight;
	fn export_terminal() -> Weight;
	fn manage() -> Weight;
}

/// Weights for `pallet_election_provider_multi_block` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	fn on_initialize_nothing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `156`
		//  Estimated: `1641`
		// Minimum execution time: 19_340_000 picoseconds.
		Weight::from_parts(19_590_000, 1641)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `Staking::ValidatorCount` (r:1 w:0)
	/// Proof: `Staking::ValidatorCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::CounterForValidators` (r:1 w:0)
	/// Proof: `Staking::CounterForValidators` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:1001 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::DesiredTargets` (r:0 w:1)
	/// Proof: `MultiBlock::DesiredTargets` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshotHash` (r:0 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshot` (r:0 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshot` (`max_values`: None, `max_size`: Some(32014), added: 34489, mode: `Measured`)
	fn on_initialize_into_snapshot_msp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48669`
		//  Estimated: `2527134`
		// Minimum execution time: 9_392_098_000 picoseconds.
		Weight::from_parts(9_449_778_000, 2527134)
			.saturating_add(T::DbWeight::get().reads(1005_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `Staking::VoterSnapshotStatus` (r:1 w:1)
	/// Proof: `Staking::VoterSnapshotStatus` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `Measured`)
	/// Storage: `VoterList::CounterForListNodes` (r:1 w:0)
	/// Proof: `VoterList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `VoterList::ListBags` (r:125 w:0)
	/// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `Measured`)
	/// Storage: `VoterList::ListNodes` (r:353 w:0)
	/// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `Measured`)
	/// Storage: `Staking::Bonded` (r:351 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `Measured`)
	/// Storage: `Staking::Ledger` (r:351 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1091), added: 3566, mode: `Measured`)
	/// Storage: `Staking::Nominators` (r:351 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:351 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshot` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: Some(194117), added: 196592, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshotHash` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `Staking::MinimumActiveStake` (r:0 w:1)
	/// Proof: `Staking::MinimumActiveStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `Measured`)
	/// Storage: `VoterList::Lock` (r:0 w:1)
	/// Proof: `VoterList::Lock` (`max_values`: Some(1), `max_size`: Some(0), added: 495, mode: `Measured`)
	fn on_initialize_into_snapshot_rest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `798019`
		//  Estimated: `1672684`
		// Minimum execution time: 26_405_552_000 picoseconds.
		Weight::from_parts(26_445_542_000, 1672684)
			.saturating_add(T::DbWeight::get().reads(1886_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `Staking::VoterSnapshotStatus` (r:1 w:1)
	/// Proof: `Staking::VoterSnapshotStatus` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `Measured`)
	/// Storage: `VoterList::CounterForListNodes` (r:1 w:0)
	/// Proof: `VoterList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `VoterList::ListNodes` (r:353 w:0)
	/// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `Measured`)
	/// Storage: `Staking::Bonded` (r:352 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `Measured`)
	/// Storage: `Staking::Ledger` (r:352 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1091), added: 3566, mode: `Measured`)
	/// Storage: `Staking::Nominators` (r:351 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `Measured`)
	/// Storage: `VoterList::ListBags` (r:1 w:0)
	/// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshot` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: Some(194117), added: 196592, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshotHash` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `Staking::MinimumActiveStake` (r:0 w:1)
	/// Proof: `Staking::MinimumActiveStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `Measured`)
	/// Storage: `VoterList::Lock` (r:0 w:1)
	/// Proof: `VoterList::Lock` (`max_values`: Some(1), `max_size`: Some(0), added: 495, mode: `Measured`)
	fn on_initialize_into_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `933895`
		//  Estimated: `1808560`
		// Minimum execution time: 26_398_971_000 picoseconds.
		Weight::from_parts(27_344_327_000, 1808560)
			.saturating_add(T::DbWeight::get().reads(1414_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::Round` (r:1 w:0)
	/// Proof: `MultiBlock::Round` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `MultiBlockSigned::SortedScores` (r:1 w:0)
	/// Proof: `MultiBlockSigned::SortedScores` (`max_values`: None, `max_size`: Some(653), added: 3128, mode: `Measured`)
	fn on_initialize_into_signed_validation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `344`
		//  Estimated: `3809`
		// Minimum execution time: 699_915_000 picoseconds.
		Weight::from_parts(1_586_881_000, 3809)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:1)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedValidVariant` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::QueuedValidVariant` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `Measured`)
	fn on_initialize_into_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `1825`
		// Minimum execution time: 266_012_000 picoseconds.
		Weight::from_parts(1_371_310_000, 1825)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedValidVariant` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::QueuedValidVariant` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedSolutionX` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::QueuedSolutionX` (`max_values`: None, `max_size`: Some(6194014), added: 6196489, mode: `Measured`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::ElectableStashes` (r:1 w:1)
	/// Proof: `Staking::ElectableStashes` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `Measured`)
	/// Storage: `Staking::ErasStakersOverview` (r:237 w:237)
	/// Proof: `Staking::ErasStakersOverview` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `Measured`)
	/// Storage: `Staking::ErasTotalStake` (r:1 w:1)
	/// Proof: `Staking::ErasTotalStake` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:237 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `Staking::ErasValidatorPrefs` (r:0 w:237)
	/// Proof: `Staking::ErasValidatorPrefs` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `Measured`)
	fn export_non_terminal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55478`
		//  Estimated: `643043`
		// Minimum execution time: 5_222_165_000 picoseconds.
		Weight::from_parts(5_247_846_000, 643043)
			.saturating_add(T::DbWeight::get().reads(480_u64))
			.saturating_add(T::DbWeight::get().writes(477_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedValidVariant` (r:1 w:1)
	/// Proof: `MultiBlockVerifier::QueuedValidVariant` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedSolutionX` (r:64 w:64)
	/// Proof: `MultiBlockVerifier::QueuedSolutionX` (`max_values`: None, `max_size`: Some(6194014), added: 6196489, mode: `Measured`)
	/// Storage: `MultiBlock::Round` (r:1 w:1)
	/// Proof: `MultiBlock::Round` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshot` (r:64 w:64)
	/// Proof: `MultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: Some(194117), added: 196592, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshotHash` (r:64 w:64)
	/// Proof: `MultiBlock::PagedVoterSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshot` (r:1 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshot` (`max_values`: None, `max_size`: Some(32014), added: 34489, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshotHash` (r:1 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::ElectableStashes` (r:1 w:1)
	/// Proof: `Staking::ElectableStashes` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `Measured`)
	/// Storage: `Staking::ErasStakersOverview` (r:414 w:414)
	/// Proof: `Staking::ErasStakersOverview` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `Measured`)
	/// Storage: `Staking::ErasStakersPaged` (r:414 w:414)
	/// Proof: `Staking::ErasStakersPaged` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ErasTotalStake` (r:1 w:1)
	/// Proof: `Staking::ErasTotalStake` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:414 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlock::DesiredTargets` (r:0 w:1)
	/// Proof: `MultiBlock::DesiredTargets` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::ErasValidatorPrefs` (r:0 w:414)
	/// Proof: `Staking::ErasValidatorPrefs` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:0 w:1)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedSolutionScore` (r:0 w:1)
	/// Proof: `MultiBlockVerifier::QueuedSolutionScore` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `Measured`)
	fn export_terminal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769472`
		//  Estimated: `1795112`
		// Minimum execution time: 24_273_552_000 picoseconds.
		Weight::from_parts(24_374_973_000, 1795112)
			.saturating_add(T::DbWeight::get().reads(1442_u64))
			.saturating_add(T::DbWeight::get().writes(1444_u64))
	}
	fn manage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 280_000 picoseconds.
		Weight::from_parts(350_000, 0)
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	fn on_initialize_nothing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `156`
		//  Estimated: `1641`
		// Minimum execution time: 19_340_000 picoseconds.
		Weight::from_parts(19_590_000, 1641)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `Staking::ValidatorCount` (r:1 w:0)
	/// Proof: `Staking::ValidatorCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::CounterForValidators` (r:1 w:0)
	/// Proof: `Staking::CounterForValidators` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:1001 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::DesiredTargets` (r:0 w:1)
	/// Proof: `MultiBlock::DesiredTargets` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshotHash` (r:0 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshot` (r:0 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshot` (`max_values`: None, `max_size`: Some(32014), added: 34489, mode: `Measured`)
	fn on_initialize_into_snapshot_msp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48669`
		//  Estimated: `2527134`
		// Minimum execution time: 9_392_098_000 picoseconds.
		Weight::from_parts(9_449_778_000, 2527134)
			.saturating_add(RocksDbWeight::get().reads(1005_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `Staking::VoterSnapshotStatus` (r:1 w:1)
	/// Proof: `Staking::VoterSnapshotStatus` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `Measured`)
	/// Storage: `VoterList::CounterForListNodes` (r:1 w:0)
	/// Proof: `VoterList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `VoterList::ListBags` (r:125 w:0)
	/// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `Measured`)
	/// Storage: `VoterList::ListNodes` (r:353 w:0)
	/// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `Measured`)
	/// Storage: `Staking::Bonded` (r:351 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `Measured`)
	/// Storage: `Staking::Ledger` (r:351 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1091), added: 3566, mode: `Measured`)
	/// Storage: `Staking::Nominators` (r:351 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:351 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshot` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: Some(194117), added: 196592, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshotHash` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `Staking::MinimumActiveStake` (r:0 w:1)
	/// Proof: `Staking::MinimumActiveStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `Measured`)
	/// Storage: `VoterList::Lock` (r:0 w:1)
	/// Proof: `VoterList::Lock` (`max_values`: Some(1), `max_size`: Some(0), added: 495, mode: `Measured`)
	fn on_initialize_into_snapshot_rest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `798019`
		//  Estimated: `1672684`
		// Minimum execution time: 26_405_552_000 picoseconds.
		Weight::from_parts(26_445_542_000, 1672684)
			.saturating_add(RocksDbWeight::get().reads(1886_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `Staking::VoterSnapshotStatus` (r:1 w:1)
	/// Proof: `Staking::VoterSnapshotStatus` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `Measured`)
	/// Storage: `VoterList::CounterForListNodes` (r:1 w:0)
	/// Proof: `VoterList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `VoterList::ListNodes` (r:353 w:0)
	/// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `Measured`)
	/// Storage: `Staking::Bonded` (r:352 w:0)
	/// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `Measured`)
	/// Storage: `Staking::Ledger` (r:352 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1091), added: 3566, mode: `Measured`)
	/// Storage: `Staking::Nominators` (r:351 w:0)
	/// Proof: `Staking::Nominators` (`max_values`: None, `max_size`: Some(558), added: 3033, mode: `Measured`)
	/// Storage: `VoterList::ListBags` (r:1 w:0)
	/// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:1 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshot` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: Some(194117), added: 196592, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshotHash` (r:0 w:1)
	/// Proof: `MultiBlock::PagedVoterSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `Staking::MinimumActiveStake` (r:0 w:1)
	/// Proof: `Staking::MinimumActiveStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `Measured`)
	/// Storage: `VoterList::Lock` (r:0 w:1)
	/// Proof: `VoterList::Lock` (`max_values`: Some(1), `max_size`: Some(0), added: 495, mode: `Measured`)
	fn on_initialize_into_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `933895`
		//  Estimated: `1808560`
		// Minimum execution time: 26_398_971_000 picoseconds.
		Weight::from_parts(27_344_327_000, 1808560)
			.saturating_add(RocksDbWeight::get().reads(1414_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlock::Round` (r:1 w:0)
	/// Proof: `MultiBlock::Round` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `MultiBlockSigned::SortedScores` (r:1 w:0)
	/// Proof: `MultiBlockSigned::SortedScores` (`max_values`: None, `max_size`: Some(653), added: 3128, mode: `Measured`)
	fn on_initialize_into_signed_validation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `344`
		//  Estimated: `3809`
		// Minimum execution time: 699_915_000 picoseconds.
		Weight::from_parts(1_586_881_000, 3809)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:1 w:1)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedValidVariant` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::QueuedValidVariant` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `Measured`)
	fn on_initialize_into_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `1825`
		// Minimum execution time: 266_012_000 picoseconds.
		Weight::from_parts(1_371_310_000, 1825)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedValidVariant` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::QueuedValidVariant` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedSolutionX` (r:1 w:0)
	/// Proof: `MultiBlockVerifier::QueuedSolutionX` (`max_values`: None, `max_size`: Some(6194014), added: 6196489, mode: `Measured`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::ElectableStashes` (r:1 w:1)
	/// Proof: `Staking::ElectableStashes` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `Measured`)
	/// Storage: `Staking::ErasStakersOverview` (r:237 w:237)
	/// Proof: `Staking::ErasStakersOverview` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `Measured`)
	/// Storage: `Staking::ErasTotalStake` (r:1 w:1)
	/// Proof: `Staking::ErasTotalStake` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:237 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `Staking::ErasValidatorPrefs` (r:0 w:237)
	/// Proof: `Staking::ErasValidatorPrefs` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `Measured`)
	fn export_non_terminal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `55478`
		//  Estimated: `643043`
		// Minimum execution time: 5_222_165_000 picoseconds.
		Weight::from_parts(5_247_846_000, 643043)
			.saturating_add(RocksDbWeight::get().reads(480_u64))
			.saturating_add(RocksDbWeight::get().writes(477_u64))
	}
	/// Storage: `MultiBlock::CurrentPhase` (r:1 w:1)
	/// Proof: `MultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedValidVariant` (r:1 w:1)
	/// Proof: `MultiBlockVerifier::QueuedValidVariant` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedSolutionX` (r:64 w:64)
	/// Proof: `MultiBlockVerifier::QueuedSolutionX` (`max_values`: None, `max_size`: Some(6194014), added: 6196489, mode: `Measured`)
	/// Storage: `MultiBlock::Round` (r:1 w:1)
	/// Proof: `MultiBlock::Round` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshot` (r:64 w:64)
	/// Proof: `MultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: Some(194117), added: 196592, mode: `Measured`)
	/// Storage: `MultiBlock::PagedVoterSnapshotHash` (r:64 w:64)
	/// Proof: `MultiBlock::PagedVoterSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshot` (r:1 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshot` (`max_values`: None, `max_size`: Some(32014), added: 34489, mode: `Measured`)
	/// Storage: `MultiBlock::PagedTargetSnapshotHash` (r:1 w:1)
	/// Proof: `MultiBlock::PagedTargetSnapshotHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `Measured`)
	/// Storage: `Staking::CurrentEra` (r:1 w:0)
	/// Proof: `Staking::CurrentEra` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::ElectableStashes` (r:1 w:1)
	/// Proof: `Staking::ElectableStashes` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `Measured`)
	/// Storage: `Staking::ErasStakersOverview` (r:414 w:414)
	/// Proof: `Staking::ErasStakersOverview` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `Measured`)
	/// Storage: `Staking::ErasStakersPaged` (r:414 w:414)
	/// Proof: `Staking::ErasStakersPaged` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ErasTotalStake` (r:1 w:1)
	/// Proof: `Staking::ErasTotalStake` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `Measured`)
	/// Storage: `Staking::Validators` (r:414 w:0)
	/// Proof: `Staking::Validators` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `Measured`)
	/// Storage: `MultiBlock::DesiredTargets` (r:0 w:1)
	/// Proof: `MultiBlock::DesiredTargets` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `Measured`)
	/// Storage: `Staking::ErasValidatorPrefs` (r:0 w:414)
	/// Proof: `Staking::ErasValidatorPrefs` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::StatusStorage` (r:0 w:1)
	/// Proof: `MultiBlockVerifier::StatusStorage` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `Measured`)
	/// Storage: `MultiBlockVerifier::QueuedSolutionScore` (r:0 w:1)
	/// Proof: `MultiBlockVerifier::QueuedSolutionScore` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `Measured`)
	fn export_terminal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769472`
		//  Estimated: `1795112`
		// Minimum execution time: 24_273_552_000 picoseconds.
		Weight::from_parts(24_374_973_000, 1795112)
			.saturating_add(RocksDbWeight::get().reads(1442_u64))
			.saturating_add(RocksDbWeight::get().writes(1444_u64))
	}
	fn manage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 280_000 picoseconds.
		Weight::from_parts(350_000, 0)
	}
}
