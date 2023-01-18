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

//! Autogenerated weights for pallet_referenda
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-27, STEPS: `20`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --chain=dev
// --steps=20
// --repeat=1
// --pallet=pallet-referenda
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/referenda/src/._weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_referenda.
pub trait WeightInfo {
	fn submit() -> Weight;
	fn place_decision_deposit_preparing() -> Weight;
	fn place_decision_deposit_queued() -> Weight;
	fn place_decision_deposit_not_queued() -> Weight;
	fn place_decision_deposit_passing() -> Weight;
	fn place_decision_deposit_failing() -> Weight;
	fn refund_decision_deposit() -> Weight;
	fn refund_submission_deposit() -> Weight;
	fn cancel() -> Weight;
	fn kill() -> Weight;
	fn one_fewer_deciding_queue_empty() -> Weight;
	fn one_fewer_deciding_failing() -> Weight;
	fn one_fewer_deciding_passing() -> Weight;
	fn nudge_referendum_requeued_insertion() -> Weight;
	fn nudge_referendum_requeued_slide() -> Weight;
	fn nudge_referendum_queued() -> Weight;
	fn nudge_referendum_not_queued() -> Weight;
	fn nudge_referendum_no_deposit() -> Weight;
	fn nudge_referendum_preparing() -> Weight;
	fn nudge_referendum_timed_out() -> Weight;
	fn nudge_referendum_begin_deciding_failing() -> Weight;
	fn nudge_referendum_begin_deciding_passing() -> Weight;
	fn nudge_referendum_begin_confirming() -> Weight;
	fn nudge_referendum_end_confirming() -> Weight;
	fn nudge_referendum_continue_not_confirming() -> Weight;
	fn nudge_referendum_continue_confirming() -> Weight;
	fn nudge_referendum_approved() -> Weight;
	fn nudge_referendum_rejected() -> Weight;
}

/// Weights for pallet_referenda using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Referenda ReferendumCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:0 w:1)
	fn submit() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_preparing() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(35_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_queued() -> Weight {
		// Minimum execution time: 40_000 nanoseconds.
		Weight::from_ref_time(40_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_not_queued() -> Weight {
		// Minimum execution time: 39_000 nanoseconds.
		Weight::from_ref_time(39_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_passing() -> Weight {
		// Minimum execution time: 43_000 nanoseconds.
		Weight::from_ref_time(43_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_failing() -> Weight {
		// Minimum execution time: 84_000 nanoseconds.
		Weight::from_ref_time(84_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn refund_decision_deposit() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn refund_submission_deposit() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn cancel() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(26_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn kill() -> Weight {
		// Minimum execution time: 47_000 nanoseconds.
		Weight::from_ref_time(47_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda TrackQueue (r:1 w:0)
	// Storage: Referenda DecidingCount (r:1 w:1)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_failing() -> Weight {
		// Minimum execution time: 88_000 nanoseconds.
		Weight::from_ref_time(88_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_passing() -> Weight {
		// Minimum execution time: 75_000 nanoseconds.
		Weight::from_ref_time(75_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Minimum execution time: 72_000 nanoseconds.
		Weight::from_ref_time(72_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Minimum execution time: 56_000 nanoseconds.
		Weight::from_ref_time(56_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_queued() -> Weight {
		// Minimum execution time: 55_000 nanoseconds.
		Weight::from_ref_time(55_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_not_queued() -> Weight {
		// Minimum execution time: 60_000 nanoseconds.
		Weight::from_ref_time(60_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_no_deposit() -> Weight {
		// Minimum execution time: 22_000 nanoseconds.
		Weight::from_ref_time(22_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_preparing() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(21_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn nudge_referendum_timed_out() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(17_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Minimum execution time: 39_000 nanoseconds.
		Weight::from_ref_time(39_000_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Minimum execution time: 31_000 nanoseconds.
		Weight::from_ref_time(31_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_end_confirming() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Scheduler Lookup (r:1 w:1)
	fn nudge_referendum_approved() -> Weight {
		// Minimum execution time: 45_000 nanoseconds.
		Weight::from_ref_time(45_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_rejected() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Referenda ReferendumCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:0 w:1)
	fn submit() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_preparing() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(35_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_queued() -> Weight {
		// Minimum execution time: 40_000 nanoseconds.
		Weight::from_ref_time(40_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_not_queued() -> Weight {
		// Minimum execution time: 39_000 nanoseconds.
		Weight::from_ref_time(39_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_passing() -> Weight {
		// Minimum execution time: 43_000 nanoseconds.
		Weight::from_ref_time(43_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_failing() -> Weight {
		// Minimum execution time: 84_000 nanoseconds.
		Weight::from_ref_time(84_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn refund_decision_deposit() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn refund_submission_deposit() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn cancel() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(26_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn kill() -> Weight {
		// Minimum execution time: 47_000 nanoseconds.
		Weight::from_ref_time(47_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda TrackQueue (r:1 w:0)
	// Storage: Referenda DecidingCount (r:1 w:1)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_failing() -> Weight {
		// Minimum execution time: 88_000 nanoseconds.
		Weight::from_ref_time(88_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_passing() -> Weight {
		// Minimum execution time: 75_000 nanoseconds.
		Weight::from_ref_time(75_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Minimum execution time: 72_000 nanoseconds.
		Weight::from_ref_time(72_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Minimum execution time: 56_000 nanoseconds.
		Weight::from_ref_time(56_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_queued() -> Weight {
		// Minimum execution time: 55_000 nanoseconds.
		Weight::from_ref_time(55_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_not_queued() -> Weight {
		// Minimum execution time: 60_000 nanoseconds.
		Weight::from_ref_time(60_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_no_deposit() -> Weight {
		// Minimum execution time: 22_000 nanoseconds.
		Weight::from_ref_time(22_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_preparing() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(21_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn nudge_referendum_timed_out() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(17_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Minimum execution time: 39_000 nanoseconds.
		Weight::from_ref_time(39_000_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Minimum execution time: 31_000 nanoseconds.
		Weight::from_ref_time(31_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_end_confirming() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Scheduler Lookup (r:1 w:1)
	fn nudge_referendum_approved() -> Weight {
		// Minimum execution time: 45_000 nanoseconds.
		Weight::from_ref_time(45_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_rejected() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(30_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}