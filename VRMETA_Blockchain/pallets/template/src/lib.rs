#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, storage::bounded_vec::BoundedVec};
	use frame_system::pallet_prelude::*;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		 /// For constraining the maximum bytes of a hash used for any proof
		type MaxBytesInHash: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a proof has been claimed. [who, claim]
        StartPlan(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
        EndPlan(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
        /// Event emitted when a proof has been claimed. [who, claim]
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
    }

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		  /// The proof has already been claimed.
		  ClientExists,
		  /// The proof does not exist, so it cannot be revoked.
		  NoSuchProof,
		  /// The proof is claimed by another account, so caller can't revoke it.
		  NotProofOwner,
	
	}

	#[pallet::storage]
    /// Maps each proof to its owner and block number when the proof was made
    /// Name in Hash + Identification Number => ID + Blocknumber
    pub(super) type Clients<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxBytesInHash>,
        (T::IdentificationNumber, T::BlockNumber),
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn get)]
    pub type IdentificationNumber<T: Config> = StorageValue<_, u128>;

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub fn add_to_plan(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxBytesInHash>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Clients::<T>::contains_key(&name), Error::<T>::ClientExists);
         
            let current_block = <frame_system::Pallet<T>>::block_number();
            let id = IdentificationNumber::<T>::try_get().is_ok();
            if id == false {
                IdentificationNumber::<T>::put(1);
            } 
            let new_id = id += 1;

            // Store the proof with the sender and block number.
            Clients::<T>::insert(&name, (&sender, current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(sender, proof));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            proof: BoundedVec<u8, T::MaxBytesInHash>,
        ) -> DispatchResult {
        
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get owner of the claim.
            // Panic condition: there is no way to set a `None` owner, so this must always unwrap.
            let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage.
            Proofs::<T>::remove(&proof);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, proof));
            Ok(())
        }
    }
}
