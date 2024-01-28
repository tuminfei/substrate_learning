
//! Autogenerated weights for pallet_poe
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-28, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `TerryTudeMBP`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --wasm-execution
// compiled
// --pallet
// pallet_poe
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// ./pallets/poe/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_poe.
pub trait WeightInfo {
	fn create_claim(d: u32, ) -> Weight;
	fn revoke_claim(d: u32, ) -> Weight;
	fn transfer_claim(d: u32, ) -> Weight;
}

/// Weights for pallet_poe using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 32]`.
	fn create_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3550`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(25_432_334, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 32]`.
	fn revoke_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3550`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(25_845_568, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 32]`.
	fn transfer_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3550`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_853_088, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 32]`.
	fn create_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3550`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(25_432_334, 3550)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 32]`.
	fn revoke_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3550`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(25_845_568, 3550)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 32]`.
	fn transfer_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3550`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_853_088, 3550)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}