#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

const TOKEN_VALUE: u32 = 2;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Currency;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::error]
    pub enum Error<T> {
        NoSufficientBalance,
        NoneValue,
    }

    #[pallet::storage]
    type BalanceStore<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    // #[pallet::storage]
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Balance {
            amount: u32,
        },
        TokenTransferred {
            from: T::AccountId,
            to: T::AccountId,
            amount: u32,
        },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn exchange_native_to_custom_token(
            origin: OriginFor<T>,
            amount: u32,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let total_fee: u32 = amount * TOKEN_VALUE;

            if T::Currency::total_balance(&owner) < total_fee.into() {
                return Err(Error::<T>::NoSufficientBalance.into());
            }

            T::Currency::burn(total_fee.into());
            <BalanceStore<T>>::insert(&owner, amount);

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

            match <BalanceStore<T>>::get(&owner) {
                Some(balance) => {
                    if balance < amount {
                        return Err(Error::<T>::NoSufficientBalance.into());
                    }
                }
                None => {
                    <BalanceStore<T>>::insert(owner.clone(), 0);
                    return Err(Error::<T>::NoSufficientBalance.into());
                }
            }

            <BalanceStore<T>>::mutate(&owner, |balance| {
                *balance = Some(balance.unwrap() - amount);
            });

            if !<BalanceStore<T>>::contains_key(dest.clone()) {
                <BalanceStore<T>>::insert(dest.clone(), amount);
            } else {
                <BalanceStore<T>>::mutate(&dest, |balance| {
                    *balance = Some(balance.unwrap() + amount);
                });
            }

            Self::deposit_event(Event::TokenTransferred {
                from: owner,
                to: dest,
                amount,
            });

            Ok(())
        }

        #[pallet::weight(0)]
        #[pallet::call_index(3)]
        pub fn get_balance(origin: OriginFor<T>) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            let balance = <BalanceStore<T>>::get(&owner);

            match balance {
                Some(balance) => Self::deposit_event(Event::Balance { amount: balance }),
                None => {
                    Self::deposit_event(Event::Balance { amount: 0 });
                    <BalanceStore<T>>::insert(owner, 0);
                }
            }
            Ok(())
        }
    }
}
