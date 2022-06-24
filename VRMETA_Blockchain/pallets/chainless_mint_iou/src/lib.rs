#![cfg_attr(not(feature = "std"), no_std)]

///This is a pallet for ChainBridge Mini.  It is used for generating claims
/// cross-chain in order to receive VRMETA from other chains; VRMETA from the host
/// chain will be burned in an equivalent amount and minted accordingly on our blockchain.
/// Hashing from inputs is the key to this process and claims are expired after 5 minutes.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*};
    use frame_support::inherent::Vec;
	use frame_system::pallet_prelude::*;
    use frame_support::traits::{UnixTime, Currency};
    use sp_io::hashing::*;


   pub type NegativeImbalanceOf<T> = <<T as Config>::Cclc as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;
   pub type Balance<T> = <<T as Config>::Cclc as Currency<<T as frame_system::Config>::AccountId>>::Balance;

   impl<T: Config> Pallet<T> {
    pub fn switch(i: u32) -> Balance<T> {
        i.into()
    }


   pub fn hash_out(phrase: Vec<u8>) -> [u8; 32] {
    let phrase_bytes: &[u8] = &phrase;
    let data = keccak_256(phrase_bytes); 
    data
    }
}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // The type used for timestamp math operations.
        type TimeProvider: UnixTime;
        // Balances
        type Cclc: Currency<Self::AccountId>;   
         
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when connected. [Hash, timestamp]
        ClaimReceived([u8; 32], [Balance<T>; 2]),
        /// Event emitted when disconnected. [Hash, who, timestamp, reward]
        ClaimFiled([u8; 32], T::AccountId, u32, Balance<T>),
    
    }

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		  /// Self.
		  NoClaim,
		  /// self
		  ClaimExpired,
          /// self
          PasswordIncorrect,
          /// self
          ClaimExists,
	
	}

	#[pallet::storage]
    /// Maps each claim to a timestamp when it will expire.
    pub type Claims<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 32],
        [Balance<T>; 2],
        OptionQuery,
    >;

  
    #[pallet::call]
    impl<T: Config> Pallet<T> {


        #[pallet::weight((10_000, DispatchClass::Normal, Pays::No))]
        pub fn validate_hash(
            origin: OriginFor<T>,
            pw:  Vec<u8>,
        ) -> DispatchResult {
            
            let sender = ensure_signed(origin)?;
            let hash: [u8; 32] = Self::hash_out(pw);
        
            ensure!(Claims::<T>::contains_key(&hash), Error::<T>::NoClaim);
            let current_time: u64 = T::TimeProvider::now().as_secs();
            let time_expire: [Balance<T>; 2] = Claims::<T>::get(&hash).unwrap();
            // Verify that the specified proof has not already been claimed.
            
            ensure!(time_expire[1] <= Self::switch(current_time as u32), Error::<T>::ClaimExpired);

            let amount_to_give = time_expire[0];
            T::Cclc::deposit_into_existing(&sender, amount_to_give);

            Self::deposit_event(Event::ClaimFiled(hash, sender, current_time as u32, amount_to_give));
            Claims::<T>::remove(&hash);

            Ok(())
        }

        #[pallet::weight((1_000, DispatchClass::Normal, Pays::No))]
        pub fn hash_received(
            origin: OriginFor<T>,
            hash: [u8; 32],
            amount: Balance<T>
        ) -> DispatchResult {
         
            // Verify that the specified proof has been claimed.
            ensure!(!Claims::<T>::contains_key(&hash), Error::<T>::ClaimExists);
        
            let current_time: u64 = T::TimeProvider::now().as_secs();
            let expiration_time = Self::switch(current_time as u32 + 300u32);

            let amount_to_give: Balance<T> = amount;       
            Claims::<T>::insert(&hash, [amount_to_give, expiration_time]);


            Self::deposit_event(Event::ClaimReceived(hash, [amount_to_give, expiration_time]));
            Ok(())
        }
       
        
    }
}


        
