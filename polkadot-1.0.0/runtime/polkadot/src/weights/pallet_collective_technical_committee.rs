// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-e8ezs4ez-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalCommittee Members (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:100 w:100)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
		//  Estimated: `15800 + m * (1967 ±16) + p * (4332 ±16)`
		// Minimum execution time: 18_203_000 picoseconds.
		Weight::from_parts(18_473_000, 0)
			.saturating_add(Weight::from_parts(0, 15800))
			// Standard Error: 43_603
			.saturating_add(Weight::from_parts(4_734_955, 0).saturating_mul(m.into()))
			// Standard Error: 43_603
			.saturating_add(Weight::from_parts(8_291_611, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1967).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 4332).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141 + m * (32 ±0)`
		//  Estimated: `1627 + m * (32 ±0)`
		// Minimum execution time: 17_071_000 picoseconds.
		Weight::from_parts(16_315_595, 0)
			.saturating_add(Weight::from_parts(0, 1627))
			// Standard Error: 14
			.saturating_add(Weight::from_parts(1_706, 0).saturating_mul(b.into()))
			// Standard Error: 146
			.saturating_add(Weight::from_parts(13_626, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141 + m * (32 ±0)`
		//  Estimated: `3607 + m * (32 ±0)`
		// Minimum execution time: 19_983_000 picoseconds.
		Weight::from_parts(18_925_239, 0)
			.saturating_add(Weight::from_parts(0, 3607))
			// Standard Error: 17
			.saturating_add(Weight::from_parts(1_664, 0).saturating_mul(b.into()))
			// Standard Error: 176
			.saturating_add(Weight::from_parts(23_169, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `3823 + m * (33 ±0) + p * (36 ±0)`
		// Minimum execution time: 26_490_000 picoseconds.
		Weight::from_parts(25_739_853, 0)
			.saturating_add(Weight::from_parts(0, 3823))
			// Standard Error: 77
			.saturating_add(Weight::from_parts(3_479, 0).saturating_mul(b.into()))
			// Standard Error: 807
			.saturating_add(Weight::from_parts(28_438, 0).saturating_mul(m.into()))
			// Standard Error: 796
			.saturating_add(Weight::from_parts(199_864, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `880 + m * (64 ±0)`
		//  Estimated: `4344 + m * (64 ±0)`
		// Minimum execution time: 27_693_000 picoseconds.
		Weight::from_parts(28_461_881, 0)
			.saturating_add(Weight::from_parts(0, 4344))
			// Standard Error: 592
			.saturating_add(Weight::from_parts(55_442, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `469 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `3914 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 28_958_000 picoseconds.
		Weight::from_parts(28_772_598, 0)
			.saturating_add(Weight::from_parts(0, 3914))
			// Standard Error: 673
			.saturating_add(Weight::from_parts(36_736, 0).saturating_mul(m.into()))
			// Standard Error: 657
			.saturating_add(Weight::from_parts(191_282, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `771 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4088 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 40_599_000 picoseconds.
		Weight::from_parts(40_617_733, 0)
			.saturating_add(Weight::from_parts(0, 4088))
			// Standard Error: 122
			.saturating_add(Weight::from_parts(3_479, 0).saturating_mul(b.into()))
			// Standard Error: 1_296
			.saturating_add(Weight::from_parts(34_407, 0).saturating_mul(m.into()))
			// Standard Error: 1_263
			.saturating_add(Weight::from_parts(236_766, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `489 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `3934 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 32_265_000 picoseconds.
		Weight::from_parts(31_660_039, 0)
			.saturating_add(Weight::from_parts(0, 3934))
			// Standard Error: 689
			.saturating_add(Weight::from_parts(39_118, 0).saturating_mul(m.into()))
			// Standard Error: 672
			.saturating_add(Weight::from_parts(192_797, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4108 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 42_456_000 picoseconds.
		Weight::from_parts(43_760_828, 0)
			.saturating_add(Weight::from_parts(0, 4108))
			// Standard Error: 132
			.saturating_add(Weight::from_parts(3_531, 0).saturating_mul(b.into()))
			// Standard Error: 1_397
			.saturating_add(Weight::from_parts(28_101, 0).saturating_mul(m.into()))
			// Standard Error: 1_362
			.saturating_add(Weight::from_parts(248_244, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298 + p * (32 ±0)`
		//  Estimated: `1783 + p * (32 ±0)`
		// Minimum execution time: 16_506_000 picoseconds.
		Weight::from_parts(18_127_000, 0)
			.saturating_add(Weight::from_parts(0, 1783))
			// Standard Error: 616
			.saturating_add(Weight::from_parts(175_889, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}