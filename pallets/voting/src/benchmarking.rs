//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Voting;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::{
	ensure,
	pallet_prelude::DispatchResultWithPostInfo,
	sp_runtime::traits::{Bounded, Get, Hash},
	BoundedVec
};
use frame_benchmarking::Vec;


benchmarks! {

	add_voter {
		let caller : T::AccountId = whitelisted_caller();
		let balance = BalanceOf::<T>::max_value();
		T::Currency::make_free_balance_be(&caller, balance);
	}: _(RawOrigin::Signed(caller.clone()))
	verify {
		assert!(Voters::<T>::contains_key(caller));
	}

	get_votes {
		let caller : T::AccountId = whitelisted_caller();
		let balance = BalanceOf::<T>::max_value();
		T::Currency::make_free_balance_be(&caller, balance);
		Pallet::<T>::add_voter(RawOrigin::Signed(caller.clone()).into());
	}: _(RawOrigin::Signed(caller.clone()), balance - T::RegisterFee::get())
	verify {
		assert!(Voters::<T>::contains_key(caller.clone()));
		assert!(Voters::<T>::get(caller).unwrap() > 0);
	}

	set_proposal {
		let mut options_vec: Vec<Options<T::Hash>> = Vec::new();
		let text = T::Hashing::hash_of(&1);
		for i in 0..T::MaxVecLen::get() {
			options_vec.push(Options {id: i as u8, votes: 0, text: text});
		}

		let options_bounded: BoundedVec<_, _> = options_vec.try_into().unwrap();

	}: _(RawOrigin::Root, text, options_bounded)
	verify {
		assert!(ActiveProposal::<T>::get().is_some());
	}

	vote {
		// create voter and the option that he will vote for
		let caller : T::AccountId = whitelisted_caller();
		let balance = BalanceOf::<T>::max_value();
		T::Currency::make_free_balance_be(&caller, balance);
		Pallet::<T>::add_voter(RawOrigin::Signed(caller.clone()).into());
		Pallet::<T>::get_votes(RawOrigin::Signed(caller.clone()).into(), balance - T::RegisterFee::get())?;

		let mut votes_vec: Vec<VoteStruct> = Vec::new();

		for i in 0..T::MaxVecLen::get() {
			votes_vec.push(VoteStruct {id: i as u8, votes: 1000});
		}

		let votes_bouded: BoundedVec<_, _> = votes_vec.try_into().unwrap();

		//create proposal
		let mut options_vec: Vec<Options<T::Hash>> = Vec::new();
		let text = T::Hashing::hash_of(&1);
		for i in 0..T::MaxVecLen::get() {
			options_vec.push(Options {id: i as u8, votes: 0, text: text});
		}

		let options_bounded: BoundedVec<_, _> = options_vec.try_into().unwrap();
		Pallet::<T>::set_proposal(RawOrigin::Root.into(), text, options_bounded)?;
	}: _(RawOrigin::Signed(caller.clone()), votes_bouded)
	verify {
		assert!(VotedProposals::<T>::contains_key(caller.clone()));
	}

	end_proposal {
		let caller : T::AccountId = whitelisted_caller();

		//create proposal
		let mut options_vec: Vec<Options<T::Hash>> = Vec::new();
		let text = T::Hashing::hash_of(&1);
		for i in 0..T::MaxVecLen::get() {
			options_vec.push(Options {id: i as u8, votes: 0, text: text});
		}

		let options_bounded: BoundedVec<_, _> = options_vec.try_into().unwrap();
		Pallet::<T>::set_proposal(RawOrigin::Root.into(), text, options_bounded)?;

		frame_system::Pallet::<T>::set_block_number(15u32.into());
	}: _(RawOrigin::Signed(caller.clone()))
	verify {
		assert!(ActiveProposal::<T>::get().is_none());
	}

	withdraw {
		let caller : T::AccountId = whitelisted_caller();
		let balance = BalanceOf::<T>::max_value();
		T::Currency::make_free_balance_be(&caller, balance);
		Pallet::<T>::add_voter(RawOrigin::Signed(caller.clone()).into());
		Pallet::<T>::get_votes(RawOrigin::Signed(caller.clone()).into(), balance - T::RegisterFee::get())?;
	}: _(RawOrigin::Signed(caller.clone()))
	verify {
		assert!(!Voters::<T>::contains_key(caller.clone()));
	}


	impl_benchmark_test_suite!(Voting, crate::mock::new_test_ext(), crate::mock::Test);
}




/* Benchmarking setup for pallet-template

#[allow(unused)]
use super::{Pallet as Template, *};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{
	ensure,
	pallet_prelude::DispatchResultWithPostInfo,
	sp_runtime::traits::{Bounded, Get},
};
use frame_system::RawOrigin;
use codec::Encode;

const SEED: u32 = 0;

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

// Call `register_voter` n times.
// The `whitelisted_caller` is always used as the final voter.
fn set_voters<T: Config>(n: u32) -> DispatchResultWithPostInfo {
	if n > 0 {
		// Starting at 1 to leave room for the whitelisted caller.
		for i in 1..n {
			// Add random voters.
			let voter = account::<T::AccountId>("voter", i, SEED);
			Pallet::<T>::register_voter(RawOrigin::Root.into(), voter)?;
		}

		// Add the whitelisted caller at the end.
		let caller = whitelisted_caller();
		Pallet::<T>::register_voter(RawOrigin::Root.into(), caller)?;
	}

	Ok(().into())
}

// Set the votes of the voters in the pallet by number of ayes, nays, and abstains.
// If the total number of votes exceeds the number of voters, we will return an error.
fn set_votes<T: Config>(ayes: u32, nays: u32, abstain: u32) -> DispatchResultWithPostInfo {
	let voters = Voters::<T>::get();
	let total_votes = ayes + nays + abstain;
	ensure!(voters.len() as u32 >= total_votes, "Too many votes for voters.");

	// Distribute votes to voters. Order of votes should not matter.
	for (i, voter) in voters.into_iter().enumerate() {
		if (i as u32) < ayes {
			Pallet::<T>::make_vote(RawOrigin::Signed(voter).into(), Vote::Aye)?;
		} else if (i as u32) < ayes + nays {
			Pallet::<T>::make_vote(RawOrigin::Signed(voter).into(), Vote::Nay)?;
		} else if (i as u32) < ayes + nays + abstain {
			Pallet::<T>::make_vote(RawOrigin::Signed(voter).into(), Vote::Abstain)?;
		} else {
			break
		}
	}

	Ok(().into())
}

// - Complete the following benchmarks.
// - Ensure they test for the worst case scenario.
// - Where it makes sense, use components to get parameterized weight outputs.
// - Add a verification function to each benchmark to make sure the expected code executed.
// - Don't forget to use the `whitelisted_caller` for the `Origin`.
// - You can iteratively test your benchmarks with:
// 		`cargo test -p pallet-template --features runtime-benchmarks`
benchmarks! {
	// Write a benchmark for `i` hashes.
	hashing {
		let i in 0 .. 1_000;
	}: {
		(0..i).for_each(|x: u32| {
			// Just add some kind of hashing here!
			// Hint: Look at the pallet code for some copyable code!
			//T::Hashing::hash_of(&x);
			T::Hashing::hash(&x.encode()[..]);
		})
	}

	// Write a benchmark for the `counter` extrinsic.
	counter {
		let i in 0 .. 255;
		let caller: T::AccountId = whitelisted_caller();
		let amount: u8 = i.try_into().unwrap();
	}: _(RawOrigin::Signed(caller), amount)
	verify {
		/* verify final state */
		assert!(MyValue::<T>::get() == amount)
	}

	// Write a benchmark for the `claimer` extrinsic.
	claimer {
		let i in 0 .. 255;
		let caller: T::AccountId = whitelisted_caller();
		let amount: u8 = i.try_into().unwrap();
	}: claimer(RawOrigin::Signed(caller), amount)
	verify {
		/* verify final state */
		assert!(MyMap::<T>::contains_key(amount));
	}

	// Write a benchmark for the `transfer_all` extrinsic.
	// Hint: This is a valid line of code:
	// `T::Currency::make_free_balance_be(&caller, balance);`
	transfer_all {
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		let receiver: T::AccountId = account("bob", 0, 0);
		assert!(T::Currency::free_balance(&receiver) == 0u32.into());

	}: _(RawOrigin::Signed(caller), receiver.clone())
	verify {
		assert!(T::Currency::free_balance(&receiver) > 0u32.into());
	}

	// Write **both** benchmarks needed for the branching function.

	branch_true {
       let caller: T::AccountId = whitelisted_caller();
   }: branched_logic(RawOrigin::Signed(caller), true)
   verify {
		assert!(MyValue::<T>::get() != 69);
   }

	branch_false {
       let caller: T::AccountId = whitelisted_caller();
   }: branched_logic(RawOrigin::Signed(caller), false)
   verify {
		assert!(MyValue::<T>::get() == 69);
   }

	// For the next benchmarks, feel free to use the provided helper functions in this file.

	// Write benchmark for register_vote function.
	// How can you verify that things executed as expected?
	// Extra: Consider what would be needed to support Weight refunds.

	// Write benchmark for make_vote function in the worst case scenario.
	// How can you verify that things executed as expected?
	// Extra: Consider what would be needed to support Weight refunds.

	// Write a benchmark for the close_vote function in the worst case scenario.
	// How can you verify that things executed as expected?
	// Extra: Consider what would be needed to support Weight refunds.

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
*/