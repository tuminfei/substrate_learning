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
//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_referenda
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: Referenda ReferendumCount (r:1 w:1)
	/// Proof: Referenda ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:0 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185`
		//  Estimated: `43917`
		// Minimum execution time: 35_637_000 picoseconds.
		Weight::from_parts(36_388_000, 0)
			.saturating_add(Weight::from_parts(0, 43917))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438`
		//  Estimated: `88267`
		// Minimum execution time: 49_103_000 picoseconds.
		Weight::from_parts(50_573_000, 0)
			.saturating_add(Weight::from_parts(0, 88267))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3034`
		//  Estimated: `13357`
		// Minimum execution time: 44_797_000 picoseconds.
		Weight::from_parts(45_606_000, 0)
			.saturating_add(Weight::from_parts(0, 13357))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3054`
		//  Estimated: `13357`
		// Minimum execution time: 44_300_000 picoseconds.
		Weight::from_parts(45_215_000, 0)
			.saturating_add(Weight::from_parts(0, 13357))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438`
		//  Estimated: `93247`
		// Minimum execution time: 64_058_000 picoseconds.
		Weight::from_parts(66_457_000, 0)
			.saturating_add(Weight::from_parts(0, 93247))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `9381`
		// Minimum execution time: 39_214_000 picoseconds.
		Weight::from_parts(40_199_000, 0)
			.saturating_add(Weight::from_parts(0, 9381))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4401`
		// Minimum execution time: 25_633_000 picoseconds.
		Weight::from_parts(26_200_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `4401`
		// Minimum execution time: 25_887_000 picoseconds.
		Weight::from_parts(26_428_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `88267`
		// Minimum execution time: 37_627_000 picoseconds.
		Weight::from_parts(38_185_000, 0)
			.saturating_add(Weight::from_parts(0, 88267))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:1 w:0)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `587`
		//  Estimated: `91784`
		// Minimum execution time: 92_363_000 picoseconds.
		Weight::from_parts(93_288_000, 0)
			.saturating_add(Weight::from_parts(0, 91784))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda TrackQueue (r:1 w:0)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `8956`
		// Minimum execution time: 9_171_000 picoseconds.
		Weight::from_parts(9_545_000, 0)
			.saturating_add(Weight::from_parts(0, 8956))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3804`
		//  Estimated: `95245`
		// Minimum execution time: 107_076_000 picoseconds.
		Weight::from_parts(107_863_000, 0)
			.saturating_add(Weight::from_parts(0, 95245))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3804`
		//  Estimated: `95245`
		// Minimum execution time: 108_656_000 picoseconds.
		Weight::from_parts(109_588_000, 0)
			.saturating_add(Weight::from_parts(0, 95245))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3822`
		//  Estimated: `52306`
		// Minimum execution time: 56_686_000 picoseconds.
		Weight::from_parts(57_215_000, 0)
			.saturating_add(Weight::from_parts(0, 52306))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3822`
		//  Estimated: `52306`
		// Minimum execution time: 56_533_000 picoseconds.
		Weight::from_parts(57_130_000, 0)
			.saturating_add(Weight::from_parts(0, 52306))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3796`
		//  Estimated: `55785`
		// Minimum execution time: 58_599_000 picoseconds.
		Weight::from_parts(59_180_000, 0)
			.saturating_add(Weight::from_parts(0, 55785))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3816`
		//  Estimated: `55785`
		// Minimum execution time: 58_664_000 picoseconds.
		Weight::from_parts(59_215_000, 0)
			.saturating_add(Weight::from_parts(0, 55785))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `46829`
		// Minimum execution time: 25_040_000 picoseconds.
		Weight::from_parts(25_512_000, 0)
			.saturating_add(Weight::from_parts(0, 46829))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `46829`
		// Minimum execution time: 25_143_000 picoseconds.
		Weight::from_parts(26_027_000, 0)
			.saturating_add(Weight::from_parts(0, 46829))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `205`
		//  Estimated: `4401`
		// Minimum execution time: 16_580_000 picoseconds.
		Weight::from_parts(17_305_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `51809`
		// Minimum execution time: 35_384_000 picoseconds.
		Weight::from_parts(35_850_000, 0)
			.saturating_add(Weight::from_parts(0, 51809))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `51809`
		// Minimum execution time: 37_799_000 picoseconds.
		Weight::from_parts(38_672_000, 0)
			.saturating_add(Weight::from_parts(0, 51809))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399`
		//  Estimated: `48330`
		// Minimum execution time: 31_282_000 picoseconds.
		Weight::from_parts(31_812_000, 0)
			.saturating_add(Weight::from_parts(0, 48330))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `382`
		//  Estimated: `48330`
		// Minimum execution time: 32_692_000 picoseconds.
		Weight::from_parts(33_118_000, 0)
			.saturating_add(Weight::from_parts(0, 48330))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399`
		//  Estimated: `48330`
		// Minimum execution time: 29_825_000 picoseconds.
		Weight::from_parts(30_210_000, 0)
			.saturating_add(Weight::from_parts(0, 48330))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `403`
		//  Estimated: `48330`
		// Minimum execution time: 28_156_000 picoseconds.
		Weight::from_parts(28_686_000, 0)
			.saturating_add(Weight::from_parts(0, 48330))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `403`
		//  Estimated: `93281`
		// Minimum execution time: 44_274_000 picoseconds.
		Weight::from_parts(44_748_000, 0)
			.saturating_add(Weight::from_parts(0, 93281))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399`
		//  Estimated: `48330`
		// Minimum execution time: 31_654_000 picoseconds.
		Weight::from_parts(32_198_000, 0)
			.saturating_add(Weight::from_parts(0, 48330))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:0 w:1)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `349`
		//  Estimated: `7957`
		// Minimum execution time: 20_050_000 picoseconds.
		Weight::from_parts(20_684_000, 0)
			.saturating_add(Weight::from_parts(0, 7957))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:1 w:1)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `7918`
		// Minimum execution time: 17_427_000 picoseconds.
		Weight::from_parts(17_920_000, 0)
			.saturating_add(Weight::from_parts(0, 7918))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}