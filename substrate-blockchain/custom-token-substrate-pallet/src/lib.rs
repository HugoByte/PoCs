#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    type BalanceStore<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Balance { dest: T::AccountId, amount: u32 },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        #[pallet::call_index(1)]
        pub fn add_amount(origin: OriginFor<T>, dest: T::AccountId, amount: u32) -> DispatchResult {
            ensure_root(origin)?;

            <BalanceStore<T>>::insert(&dest, amount);

            Ok(())
        }

        #[pallet::weight(0)]
        #[pallet::call_index(2)]
        pub fn my_transfer(
            origin: OriginFor<T>,
            dest: T::AccountId,
            amount: u32,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            <BalanceStore<T>>::mutate(&owner, |balance| {
                *balance = Some(balance.unwrap() - amount);
            });

            <BalanceStore<T>>::mutate(&dest, |balance| {
                *balance = Some(balance.unwrap() + amount);
            });

            Ok(())
        }

        #[pallet::weight(0)]
        #[pallet::call_index(3)]
        pub fn get_balance(origin: OriginFor<T>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let balance = <BalanceStore<T>>::get(&owner);

            match balance {
                Some(balance) => Self::deposit_event(Event::Balance {
                    dest: owner,
                    amount: balance,
                }),
                None => Self::deposit_event(Event::Balance {
                    dest: owner,
                    amount: 0,
                }),
            }

            Ok(())
        }
    }
}
