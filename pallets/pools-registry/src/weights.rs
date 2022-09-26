//! Autogenerated weights for pallet_pools
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development
// --steps=50
// --repeat=20
// --pallet=pallet-pools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/pools/src/weights.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_pools.
pub trait WeightInfo {
	fn create(n: u32) -> Weight;
	fn update_no_execution(n: u32) -> Weight;
	fn set_metadata(n: u32) -> Weight;
}

/// Weights for pallet_pools using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create(n: u32) -> Weight {
		(74_485_000 as Weight) // Standard Error: 41_000
			.saturating_add((299_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}

	fn update_no_execution(n: u32) -> Weight {
		(28_660_000 as Weight) // Standard Error: 17_000
			.saturating_add((285_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn set_metadata(n: u32) -> Weight {
		(35_072_000 as Weight) // Standard Error: 0
			.saturating_add((13_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create(n: u32) -> Weight {
		(74_485_000 as Weight) // Standard Error: 41_000
			.saturating_add((299_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}

	fn update_no_execution(n: u32) -> Weight {
		(28_660_000 as Weight) // Standard Error: 17_000
			.saturating_add((285_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}

	fn set_metadata(n: u32) -> Weight {
		(35_072_000 as Weight) // Standard Error: 0
			.saturating_add((13_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
