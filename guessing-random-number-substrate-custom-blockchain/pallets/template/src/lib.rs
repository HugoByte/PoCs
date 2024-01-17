#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// pub use frame_support::traits::Randomness;

	#[pallet::pallet]
	//  #[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	pub(super) type TargetNumber<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub(super) type Guesses<T: Config> = StorageMap<_, Blake2_128Concat, u32, (T::AccountId, u32)>;

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TargetNotSet,

		ExactNumber { who: T::AccountId, number: u32 },

		TargetNumberChanged,

		TargetNumberRemoved,

		GuessedNumber { who: T::AccountId, number: u32 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		TargetNotSet,

		PredecessorNumber,

		SuccessorNumber,

		TooManyTurns,
	}

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		#[pallet::call_index(1)]
		pub fn set_target(origin: OriginFor<T>, number: u32) -> DispatchResult {
			// Check that the extrinsic was root.
			// This function will return an error if the extrinsic is not signed.
			ensure_root(origin)?;
			Self::deposit_event(Event::TargetNumberChanged);
			TargetNumber::<T>::put(number);

			Ok(())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(2)]
		pub fn check_with(origin: OriginFor<T>, number: u32) -> DispatchResult {
			if !TargetNumber::<T>::exists() {
				return Err(Error::<T>::TargetNotSet.into())
			}

			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			let target = TargetNumber::<T>::get();

			Self::deposit_event(Event::GuessedNumber { who: sender.clone(), number });

			if number == target {
				Self::deposit_event(Event::ExactNumber { who: sender, number });
				TargetNumber::<T>::kill();
				Self::deposit_event(Event::TargetNumberRemoved);
			} else if number < target {
				return Err(Error::<T>::PredecessorNumber.into())
			} else {
				return Err(Error::<T>::SuccessorNumber.into())
			}

			return Ok(());
		}

		#[pallet::weight(0)]
		#[pallet::call_index(3)]
		pub fn remove_target(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was root.
			// This function will return an error if the extrinsic is not signed.
			ensure_root(origin)?;
			ensure!(TargetNumber::<T>::exists(), Error::<T>::TargetNotSet);

			TargetNumber::<T>::kill();

			Self::deposit_event(Event::TargetNumberRemoved);
			Ok(())
		}
	}
}
