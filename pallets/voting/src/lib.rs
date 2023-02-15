#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

use frame_support::traits::Currency;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::{ *},
		sp_runtime::traits::{IntegerSquareRoot, Zero},
		traits::{Currency, LockableCurrency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use frame_system::weights::WeightInfo;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type to access the Balances Pallet.
		type Currency: Currency<Self::AccountId>
			+ ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId>;

		#[pallet::constant]
		type RegisterFee: Get<BalanceOf<Self>>;

		#[pallet::constant]
		type MaxVecLen: Get<u32>;

		#[pallet::constant]
		type MaxProposalDuration: Get<Self::BlockNumber>;

		type WeightInfo: weights::WeightInfo;
	}


	#[derive( Clone, Encode, Decode, TypeInfo,MaxEncodedLen, Debug, Copy, Eq, PartialEq)]
	#[scale_info(skip_type_params(T))]
	pub struct Options<Hash>{
		pub id: u8,
		pub votes: u128,
		pub text: Hash
	}

	#[derive( Clone, Encode, Decode, TypeInfo,MaxEncodedLen, Debug, Copy, Eq, PartialEq)]
	pub struct VoteStruct {
		pub id: u8,
		pub votes: u128,
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Debug, Eq, PartialEq)]
	#[scale_info(skip_type_params(T))]
	pub struct FinishedProposal<T: Config> {
		pub id: u32,
		pub text: T::Hash,
		pub end_block: T::BlockNumber,
		pub status: ProposalStatus,
		pub options_votes: BoundedVec<Options<T::Hash>, T::MaxVecLen>,
		pub winner_index: u8,
	}

	#[derive( Encode, Decode, TypeInfo, MaxEncodedLen, Eq, PartialEq, Clone, Debug)]
	#[scale_info(skip_type_params(T))]
	pub struct CurrentProposal <T: Config>{
		pub id: u32,
		pub end_block: T::BlockNumber,
		pub status: ProposalStatus,
		pub text: T::Hash,
		pub options: BoundedVec<Options<T::Hash>, T::MaxVecLen>,
	} 
	

	#[derive(Debug, Clone, Encode, Decode, TypeInfo, MaxEncodedLen, Eq, PartialEq, Copy)]
	pub enum ProposalStatus {
		InProgress,
		Finished,
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn is_voter)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	/// Storage item to store accounts that can vote.
	pub type Voters<T: Config> = StorageMap<_, Blake2_128, T::AccountId, u128>;
	
	 #[pallet::storage]
	pub type ActiveProposal<T: Config> = StorageValue<_, CurrentProposal<T>>;

	#[pallet::type_value]
	pub fn DefaultProposalCounter<T: Config>() -> u32 { 1u32 }

	#[pallet::storage]
	pub type ProposalCount<T: Config> = StorageValue<_, u32, ValueQuery, DefaultProposalCounter<T>>;
	
	#[pallet::storage]
	pub type FinishedProposals<T: Config> = StorageMap<_, Blake2_128, u32 ,FinishedProposal<T>>;

	#[pallet::storage]
	pub type VotedProposals<T: Config> = StorageMap<_, Blake2_128, T::AccountId, u32, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New voter added.
		NewVoter { who: T::AccountId },
		/// Votes Emited
		VotesEmited { who: T::AccountId, votes: u128 },
		/// Proposal Created
		ProposalCreated { id: u32 },
		/// Vote Casted
		VotesDeposited { who: T::AccountId, proposal_id: u32, votes: BoundedVec<VoteStruct, T::MaxVecLen> },
		/// Proposal Finished
		ProposalFinished { id: u32, winner_index: u8, winner_votes: u128 },
		/// Votes Withdrawn
		VotesWithdrawn { who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Account is already registered as a voter.
		AlreadyVoter,
		/// Account is not registered as a voter.
		NotAVoter,
		/// Invalid amount of reserved tokens
		InvalidTokenAmount,
		/// A proposal is already active
		ProposalAlreadyActive,
		/// There is no active proposal
		NoActiveProposal,
		/// Not enough votes
		NotEnoughVotes,
		/// The account has already voted
		AlreadyVoted,
		/// The proposal is not finished
		ProposalNotFinished,
		/// Invalid option to vote
		InvalidOptionId,
		/// Proposal finished
		ProposalFinished,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T::WeightInfo as weights::WeightInfo>::add_voter())]
		/// Register a new voter. A fee is needed to register.
		pub fn add_voter(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(!<Voters<T>>::contains_key(who.clone()), Error::<T>::AlreadyVoter);
			let fee = T::RegisterFee::get();
			T::Currency::reserve(&who, fee)?;
			<Voters<T>>::insert(who.clone(), 0);
			Self::deposit_event(Event::NewVoter { who });
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T::WeightInfo as weights::WeightInfo>::get_votes())]
		/// Lock tokens to get votes
		pub fn get_votes(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			
			let who = ensure_signed(origin)?;

			ensure!(<Voters<T>>::contains_key(who.clone()), Error::<T>::NotAVoter);
			ensure!(amount > T::RegisterFee::get(), Error::<T>::InvalidTokenAmount);

			T::Currency::reserve(&who, amount)?;
			let reserves_to_compute = T::Currency::reserved_balance(&who) - T::RegisterFee::get();
			// qed
			let votes: u128 = reserves_to_compute.integer_sqrt().try_into().ok().unwrap();
			<Voters<T>>::mutate(who.clone(), | previous_votes| {
				*previous_votes = Some(votes);
			});
			Self::deposit_event(Event::VotesEmited { who, votes });
			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T::WeightInfo as weights::WeightInfo>::set_proposal())]
		/// Set a new active proposal
		pub fn set_proposal(origin: OriginFor<T>, text: T::Hash, vote_options: BoundedVec<Options<T::Hash>,T::MaxVecLen>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			ensure!(<ActiveProposal<T>>::get().is_none(), Error::<T>::ProposalAlreadyActive);

			let proposal_count = <ProposalCount<T>>::get();
			// create the proposal struct
			let new_proposal = CurrentProposal::<T> {
				id: proposal_count,
				end_block: <frame_system::Pallet<T>>::block_number() + T::MaxProposalDuration::get(),
				status: ProposalStatus::InProgress,
				text: text,
				options: vote_options,
			};

			// set the proposal as active
			<ActiveProposal<T>>::set(Some(new_proposal));

			Self::deposit_event(Event::ProposalCreated { id: proposal_count });

			// increase the proposals count
			<ProposalCount<T>>::mutate(|count| {
				*count += 1;
			});
			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(<T::WeightInfo as weights::WeightInfo>::vote())]
		/// Vote for a proposal
		pub fn vote(origin: OriginFor<T>, votes: BoundedVec<VoteStruct,T::MaxVecLen>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// check if the voter is registered
			ensure!(<Voters<T>>::contains_key(who.clone()), Error::<T>::NotAVoter);
			// check if the voter has mroe than 0 votes
			ensure!(!<Voters<T>>::get(who.clone()).unwrap().is_zero(), Error::<T>::NotEnoughVotes);
			// check if there is an active proposal
			ensure!(<ActiveProposal<T>>::get().is_some(), Error::<T>::NoActiveProposal);

			// check if the proposal is open qed
			if <ActiveProposal<T>>::get().unwrap().end_block < <frame_system::Pallet<T>>::block_number() {
				// return error
				return Err(Error::<T>::ProposalFinished.into());
			}

			// get the active proposal qed
			let mut active_proposal = <ActiveProposal<T>>::get().unwrap();

			// check if the voter has already voted
			if <VotedProposals<T>>::get(who.clone()) == active_proposal.id {
				// return error	
				return Err(Error::<T>::AlreadyVoted.into());
			}

			// set this proposal as voted
			<VotedProposals<T>>::insert(who.clone(), active_proposal.id);

			// counter to check the used votes
			let mut used_votes = 0u128;
			
			// get the available votes qed
			let available_votes = <Voters<T>>::get(who.clone()).unwrap();

			
			for i in &votes {
			 	let proposal_voted = i.id;
				if u32::from(proposal_voted) >= T::MaxVecLen::get() {
					return Err(Error::<T>::InvalidOptionId.into());
				}
				for j in 0..active_proposal.options.len() {
					if active_proposal.options[j].id == proposal_voted {
						active_proposal.options[j].votes += i.votes;
						used_votes += i.votes;
					}
				}
			}
			
			Self::deposit_event(Event::VotesDeposited { who: who.clone(),proposal_id: active_proposal.id, votes: votes.clone() });
			// verify that the user not vote more than he can
			ensure!(used_votes <= available_votes, Error::<T>::NotEnoughVotes);
			// update the active proposal
			<ActiveProposal<T>>::set(Some(active_proposal));

			Ok(().into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(<T::WeightInfo as weights::WeightInfo>::end_proposal())]
		/// End the active proposal
		pub fn end_proposal(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			ensure!(<ActiveProposal<T>>::get().is_some(), Error::<T>::NoActiveProposal);

			// qed
			let active_proposal = <ActiveProposal<T>>::get().unwrap();
			if active_proposal.end_block > <frame_system::Pallet<T>>::block_number() {
				return Err(Error::<T>::ProposalNotFinished.into());
			}

			// search the winner option
			let mut winner_index:u8 = 0;
			let mut winner_votes:u128 = 0; 
			for i in active_proposal.options.iter() {
				if i.votes > winner_votes {
					winner_index = i.id as u8;
					winner_votes = i.votes;
				}
			}
			
			let new_finished_proposal = FinishedProposal::<T> {
				id: active_proposal.id,
				text: active_proposal.text,
				end_block: active_proposal.end_block,
				status: ProposalStatus::Finished,
				options_votes: active_proposal.options,
				winner_index: winner_index,
			};

			// add the finished proposal to the finished proposals list
			<FinishedProposals<T>>::insert(active_proposal.id, new_finished_proposal);
			// kill the current active proposal
			<ActiveProposal<T>>::kill();

			Self::deposit_event(Event::ProposalFinished { id: active_proposal.id, winner_index: winner_index, winner_votes: winner_votes });

			Ok(().into())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(<T::WeightInfo as weights::WeightInfo>::withdraw())]
		/// Withdraw the votes
		pub fn withdraw(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// check if the voter is registered
			ensure!(<Voters<T>>::contains_key(who.clone()), Error::<T>::NotAVoter);
			// check if there is an active proposal
			ensure!(!<ActiveProposal<T>>::get().is_some(), Error::<T>::ProposalAlreadyActive);

			// kill the voters storage
			<Voters<T>>::remove(who.clone());

			// free the tokens
			T::Currency::unreserve(&who, T::Currency::reserved_balance(&who));

			Self::deposit_event(Event::VotesWithdrawn { who: who.clone() });

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get the account ID of the pallet.
		pub fn get_vote_amount(who: T::AccountId) -> Option<u128> {
			Voters::<T>::get(who)
		}

		pub fn get_active_proposal() -> Option<CurrentProposal<T>> {
			ActiveProposal::<T>::get()
		}

		pub fn get_closed_proposal(id: u32) -> Option<FinishedProposal<T>> {
			FinishedProposals::<T>::get(id)
		}
	}
}