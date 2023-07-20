
//! Autogenerated weights for `orml_currencies_allowance_extension`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-19, STEPS: `100`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Hugos-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("foucoco"), DB CACHE: 1024

// Executed Command:
// ./target/release/pendulum-node
// benchmark
// pallet
// --chain=foucoco
// --pallet=orml-currencies-allowance-extension
// --extrinsic=*
// --steps=100
// --repeat=10
// --wasm-execution=compiled
// --output=pallets/orml-currencies-allowance-extension/src/default_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `orml_currencies_allowance_extension`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	/// Storage: TokenAllowance AllowedCurrencies (r:0 w:1)
	/// Proof Skipped: TokenAllowance AllowedCurrencies (max_values: None, max_size: None, mode: Measured)
	fn add_allowed_currencies() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TokenAllowance AllowedCurrencies (r:0 w:1)
	/// Proof Skipped: TokenAllowance AllowedCurrencies (max_values: None, max_size: None, mode: Measured)
	fn remove_allowed_currencies() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TokenAllowance AllowedCurrencies (r:1 w:0)
	/// Proof Skipped: TokenAllowance AllowedCurrencies (max_values: None, max_size: None, mode: Measured)
	/// Storage: TokenAllowance Approvals (r:0 w:1)
	/// Proof Skipped: TokenAllowance Approvals (max_values: None, max_size: None, mode: Measured)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `184`
		//  Estimated: `3833`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3833))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TokenAllowance AllowedCurrencies (r:1 w:0)
	/// Proof Skipped: TokenAllowance AllowedCurrencies (max_values: None, max_size: None, mode: Measured)
	/// Storage: TokenAllowance Approvals (r:1 w:1)
	/// Proof Skipped: TokenAllowance Approvals (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `561`
		//  Estimated: `14248`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(33_000_000, 0)
			.saturating_add(Weight::from_parts(0, 14248))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
