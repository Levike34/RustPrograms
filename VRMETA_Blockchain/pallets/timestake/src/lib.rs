#![cfg_attr(not(feature = "std"), no_std)]

/// Players earn based on the time they play.  More time -> greater mint.
/// Changes:  Earned rewards will depend on staked funds and personal play times + points.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*};
	use frame_system::pallet_prelude::*;
    use frame_support::traits::{UnixTime, Currency};


   pub type NegativeImbalanceOf<T> = <<T as Config>::Cclc as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;
   pub type BalanceOf<T> = <<T as Config>::Cclc as Currency<<T as frame_system::Config>::AccountId>>::Balance;

   impl<T: Config> Pallet<T> {
    pub fn switch(i: u32) -> BalanceOf<T> {
        i.into()
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
        /// Event emitted when connected. [who, timestamp]
        Connected(T::AccountId, u64),
        /// Event emitted when disconnected. [who, timestamp, reward]
        Disconnected(T::AccountId, u64, BalanceOf<T>),
    }

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		  /// Self.
		  AlreadyConnected,
		  /// self
		  NotConnected,
	
	}

	#[pallet::storage]
    /// Maps each player to the timestamp when they started.
    pub type Players<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        u64,
        OptionQuery,
    >;

  
    #[pallet::call]
    impl<T: Config> Pallet<T> {


        #[pallet::weight((10_000, DispatchClass::Normal, Pays::No))]
        pub fn connect(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not already been claimed.
            ensure!(!Players::<T>::contains_key(&sender), Error::<T>::AlreadyConnected);

            let current_time: u64 = T::TimeProvider::now().as_secs();
          
            Players::<T>::insert(&sender, current_time);

            Self::deposit_event(Event::Connected(sender, current_time));

            Ok(())
        }

        #[pallet::weight((10_000, DispatchClass::Normal, Pays::No))]
        pub fn disconnect(
            origin: OriginFor<T>,
        ) -> DispatchResult {
         
            let sender = ensure_signed(origin)?;
            // Verify that the specified proof has been claimed.
            ensure!(Players::<T>::contains_key(&sender), Error::<T>::NotConnected);
        
            let reward = Players::<T>::get(&sender).unwrap();
            let current_time: u64 = T::TimeProvider::now().as_secs();
            let time_played = current_time - reward;
         
            let multiplier: BalanceOf<T> = Self::switch(time_played as u32);
            let coin_per_hour: BalanceOf<T> = Self::switch(1_000_000_000);
            let divisor: BalanceOf<T> = Self::switch(3_600);

            let amount_to_give: BalanceOf<T> = (multiplier / divisor) * coin_per_hour;       
            let _tx = T::Cclc::deposit_into_existing(&sender, amount_to_give);


            Players::<T>::remove(&sender);
            Self::deposit_event(Event::Disconnected(sender, time_played, amount_to_give));
            Ok(())
        }
       
        
        }

        
   
}
