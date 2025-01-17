
//! Autogenerated weights for `pallet_voting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Mateo`, CPU: `Intel(R) Core(TM) i5-1035G4 CPU @ 1.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/node-template
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-voting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/voting/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_voter() -> Weight;
	fn get_votes() -> Weight;
	fn set_proposal() -> Weight;
	fn vote() -> Weight;
	fn end_proposal() -> Weight;
	fn withdraw() -> Weight;
}

/// Weight functions for `pallet_voting`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Voting Voters (r:1 w:1)
	fn add_voter() -> Weight {
		// Minimum execution time: 44_830 nanoseconds.
		Weight::from_ref_time(45_963_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Voting Voters (r:1 w:1)
	fn get_votes() -> Weight {
		// Minimum execution time: 49_383 nanoseconds.
		Weight::from_ref_time(51_324_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Voting ActiveProposal (r:1 w:1)
	// Storage: Voting ProposalCount (r:1 w:1)
	fn set_proposal() -> Weight {
		// Minimum execution time: 31_415 nanoseconds.
		Weight::from_ref_time(32_145_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Voting Voters (r:1 w:0)
	// Storage: Voting ActiveProposal (r:1 w:1)
	// Storage: Voting VotedProposals (r:1 w:1)
	fn vote() -> Weight {
		// Minimum execution time: 50_141 nanoseconds.
		Weight::from_ref_time(53_487_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Voting ActiveProposal (r:1 w:1)
	// Storage: Voting FinishedProposals (r:0 w:1)
	fn end_proposal() -> Weight {
		// Minimum execution time: 35_472 nanoseconds.
		Weight::from_ref_time(37_081_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Voting Voters (r:1 w:1)
	// Storage: Voting ActiveProposal (r:1 w:0)
	fn withdraw() -> Weight {
		// Minimum execution time: 50_598 nanoseconds.
		Weight::from_ref_time(51_795_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
