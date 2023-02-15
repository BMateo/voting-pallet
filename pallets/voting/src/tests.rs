use crate::{mock::*, Error, Event, Options, VoteStruct};
use frame_support::{assert_noop, assert_ok};
use frame_support::bounded_vec;
use frame_support::pallet_prelude::ConstU32;

use sp_runtime::BoundedVec;

type BalanceError = pallet_balances::Error::<Test>;

use sp_core::H256;

use sp_runtime::traits::IntegerSquareRoot;
 


#[test]
fn add_a_voter() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));

		assert!(Voting::is_voter(1).is_some());

		// Assert that the correct event was deposited
		System::assert_last_event(Event::NewVoter { who: 1}.into());
	});
}

#[test]
fn cannot_add_a_voter_twice() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));
		assert!(Voting::is_voter(1).is_some());

		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Voting::add_voter(RuntimeOrigin::signed(1)), Error::<Test>::AlreadyVoter);

		// Assert that the correct event was deposited
		System::assert_last_event(Event::NewVoter { who: 1}.into());
	});
}

#[test]
fn insufficient_balance_to_register() {
	new_test_ext().execute_with(|| {
		assert_noop!(Voting::add_voter(RuntimeOrigin::signed(10)), BalanceError::InsufficientBalance);		
	});
	
}

#[test]
fn get_votes() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// a not registered voter cannot get votes
		assert_noop!(Voting::get_votes(RuntimeOrigin::signed(1), 100), Error::<Test>::NotAVoter);
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));

		// cannot reserve 0 votes
		assert_noop!(Voting::get_votes(RuntimeOrigin::signed(1), 0), Error::<Test>::InvalidTokenAmount);
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));

		let votes = Voting::get_vote_amount(1);
		assert!(votes.unwrap() == 100u128.integer_sqrt());

		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));
		let new_votes = Voting::get_vote_amount(1);

		assert!(new_votes.unwrap() == 200u128.integer_sqrt());

		// Assert that the correct event was deposited
		System::assert_last_event(Event::VotesEmited { who: 1, votes: 200u128.integer_sqrt()}.into());

	});
}

#[test]
fn create_first_proposal() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// create vector of options 
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));

		assert!(Voting::get_active_proposal().is_some());

		// if i want to create a second proposal i need to wait for the first one to expire
		assert_noop!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options), Error::<Test>::ProposalAlreadyActive);

		// Assert that the correct event was deposited
		System::assert_last_event(Event::ProposalCreated { id: 1}.into());
	});
}

#[test]
fn vote_proposal() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));
		// add a voter and get votes
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));
		
		// vote successfully
		let vote_vec_valid: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:5}, VoteStruct {id:1, votes:2}, VoteStruct{id:2, votes:3}];
		assert_ok!(Voting::vote(RuntimeOrigin::signed(1), vote_vec_valid.clone()));

		let active_proposal = Voting::get_active_proposal().unwrap();

		// verify that the votes are correct
		assert!(active_proposal.options[0].votes == 5 && active_proposal.options[1].votes == 2 && active_proposal.options[2].votes == 3);

		// add another voter
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(2)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(2), 144));

		// vote successfully voter 2
		let vote_vec_valid_2: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:5}, VoteStruct {id:1, votes:5}, VoteStruct{id:2, votes:0}];
		assert_ok!(Voting::vote(RuntimeOrigin::signed(2), vote_vec_valid_2.clone()));

		// user cannot vote twice even if he not used all of his votes
		assert_noop!(Voting::vote(RuntimeOrigin::signed(2), vote_vec_valid_2), Error::<Test>::AlreadyVoted);

		let active_proposal_updated = Voting::get_active_proposal().unwrap();

		assert!(active_proposal_updated.options[0].votes == 10 && active_proposal_updated.options[1].votes == 7 && active_proposal_updated.options[2].votes == 3);
	});	
}

#[test]
fn cannot_vote_after_end_block() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));
		// add a voter and get votes
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));

		// advance time and check that the user cannot vote
		System::set_block_number(100);
		let vote_vec_valid: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:5}, VoteStruct {id:1, votes:2}, VoteStruct{id:2, votes:3}];
		assert_noop!(Voting::vote(RuntimeOrigin::signed(1), vote_vec_valid), Error::<Test>::ProposalFinished);
	});
}

#[test]
fn not_enough_votes() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));
		// add a voter and get votes
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));

		// invalid votes (invalid amount of votes)
		let vote_vec_invalid_votes: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:5}, VoteStruct {id:1, votes:5}, VoteStruct{id:2, votes:3}];
		assert_noop!(Voting::vote(RuntimeOrigin::signed(1), vote_vec_invalid_votes.clone()), Error::<Test>::NotEnoughVotes);
	});
}

#[test]
fn cannot_vote_twice() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));
		// add a voter and get votes
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));

		// vote successfully
		let vote_vec_valid: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:5}, VoteStruct {id:1, votes:2}, VoteStruct{id:2, votes:3}];
		assert_ok!(Voting::vote(RuntimeOrigin::signed(1), vote_vec_valid.clone()));
		// user cannot vote twice
		assert_noop!(Voting::vote(RuntimeOrigin::signed(1), vote_vec_valid.clone()), Error::<Test>::AlreadyVoted);
	});
}

#[test]
fn not_register_vote_cannot_vote() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));

		//vector of the options that the user wants to vote
		let vote_vec: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:5}, VoteStruct {id:1, votes:5}, VoteStruct{id:3, votes:3}];
		assert_noop!(Voting::vote(RuntimeOrigin::signed(1), vote_vec.clone()), Error::<Test>::NotAVoter);
	});
}

#[test]
fn vote_invalid_option_id() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));

		// add a voter and get votes
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));

		// invalid votes (invalid vote id)
		//vector of the options that the user wants to vote
		let vote_vec_invalid_id: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:3, votes:1}, VoteStruct {id:1, votes:2}, VoteStruct{id:2, votes:3}];
		assert_noop!(Voting::vote(RuntimeOrigin::signed(1),vote_vec_invalid_id.clone()), Error::<Test>::InvalidOptionId);
	});
}

#[test]
fn close_proposal() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];

		System::set_block_number( 1);
		// try to close a proposal without one active
		assert_noop!(Voting::end_proposal(RuntimeOrigin::signed(1)), Error::<Test>::NoActiveProposal);

		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));

		// try to close a proposal before the end of the voting period
		assert_noop!(Voting::end_proposal(RuntimeOrigin::signed(1)), Error::<Test>::ProposalNotFinished);

		// add voter
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(2)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(2), 144));

		// vote successfully voter 2
		let vote_vec_valid_2: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:6}, VoteStruct {id:1, votes:5}, VoteStruct{id:2, votes:0}];
		assert_ok!(Voting::vote(RuntimeOrigin::signed(2), vote_vec_valid_2.clone()));

		

		// End a proposal successfully
		System::set_block_number( 15);

		assert_ok!(Voting::end_proposal(RuntimeOrigin::signed(1)));
		// clean the active proposal
		assert!(Voting::get_active_proposal().is_none());

		// check the list of closed proposals
		assert!(Voting::get_closed_proposal(1).is_some());

		// check the winner option in the closed proposal
		let finished_proposal = Voting::get_closed_proposal(1).unwrap();

		assert!(finished_proposal.options_votes[0].votes == 6 && finished_proposal.winner_index == 0);
	});
}

#[test]
fn withdraw_votes() {
	new_test_ext().execute_with(|| {
		// create vector of options
		let options: BoundedVec<Options<H256>,ConstU32<3>> = bounded_vec![Options {id:0, votes:0, text:H256::random()}, Options {id:1, votes:0, text:H256::random()}, Options{id:2, votes:0, text:H256::random()}];
		// a not registered voter cannot withdraw votes
		assert_noop!(Voting::withdraw(RuntimeOrigin::signed(1)), Error::<Test>::NotAVoter);
		
		// create proposal
		assert_ok!(Voting::set_proposal(RuntimeOrigin::root(),H256::random(), options.clone()));
		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(1)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(1), 100));

		assert_ok!(Voting::add_voter(RuntimeOrigin::signed(2)));
		assert_ok!(Voting::get_votes(RuntimeOrigin::signed(2), 100));

		// users vote
		// vote successfully
		let vote_vec_valid: BoundedVec<VoteStruct,ConstU32<3>> = bounded_vec![VoteStruct {id:0, votes:5}, VoteStruct {id:1, votes:2}, VoteStruct{id:2, votes:3}];
		assert_ok!(Voting::vote(RuntimeOrigin::signed(1), vote_vec_valid.clone()));
		assert_ok!(Voting::vote(RuntimeOrigin::signed(2), vote_vec_valid.clone()));

		// cannot withdraw votes before the end of the voting period
		assert_noop!(Voting::withdraw(RuntimeOrigin::signed(1)), Error::<Test>::ProposalAlreadyActive);
		// Set the end of the voting period
		System::set_block_number( 15);
		// end proposal
		assert_ok!(Voting::end_proposal(RuntimeOrigin::signed(1)));
		// user one withdraw is votes
		assert_ok!(Voting::withdraw(RuntimeOrigin::signed(1)));
	});
}