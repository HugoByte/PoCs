#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::{CompactAs, Decode, Encode, MaxEncodedLen};
	use frame_support::pallet_prelude::{IsType, *};
	use frame_system::{ensure_root, pallet_prelude::OriginFor};
	use scale_info::{
		prelude::fmt,
		TypeInfo,
	};
	use serde::{Deserialize, Serialize};
	use sp_core::{bytes, RuntimeDebug, TypeId};
    use polkadot_core_primitives::{Hash, OutboundHrmpMessage};
    use codec::alloc::string::ToString;
    use sp_std::vec::Vec;


    pub use sp_runtime::traits::{BlakeTwo256, Hash as HashT};
	use frame_system::ensure_signed;


    
	#[pallet::pallet]
    #[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created (T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		ErrorOccurs,
        ParaAlreadyExists
	}

	#[derive(
		PartialEq,
		Eq,
		Clone,
		Encode,
		Decode,
		RuntimeDebug,
		derive_more::From,
		TypeInfo,
		Serialize,
		Deserialize,
	)]
	#[cfg_attr(feature = "std", derive(Hash))]
	pub struct ValidationCode(#[serde(with = "bytes")] pub Vec<u8>);

	impl ValidationCode {
		/// Get the blake2-256 hash of the validation code bytes.
		pub fn hash(&self) -> ValidationCodeHash {
			ValidationCodeHash(sp_runtime::traits::BlakeTwo256::hash(&self.0[..]))
		}
	}

	#[derive(Clone, Encode, Decode, Hash, Eq, PartialEq, PartialOrd, Ord, TypeInfo)]
	pub struct ValidationCodeHash(Hash);

	impl sp_std::fmt::Display for ValidationCodeHash {
		fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
			self.0.fmt(f)
		}
	}

	impl sp_std::fmt::Debug for ValidationCodeHash {
		fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
			write!(f, "{:?}", self.0)
		}
	}

	impl AsRef<[u8]> for ValidationCodeHash {
		fn as_ref(&self) -> &[u8] {
			self.0.as_ref()
		}
	}

	impl From<Hash> for ValidationCodeHash {
		fn from(hash:Hash) -> ValidationCodeHash {
			ValidationCodeHash(hash)
		}
	}

	impl From<[u8; 32]> for ValidationCodeHash {
		fn from(hash: [u8; 32]) -> ValidationCodeHash {
			ValidationCodeHash(hash.into())
		}
	}

	impl sp_std::fmt::LowerHex for ValidationCodeHash {
		fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
			sp_std::fmt::LowerHex::fmt(&self.0, f)
		}
	}


	#[derive(
		PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Serialize, Deserialize,
	)]
	pub struct ParaGenesisArgs {
		pub validation_code: ValidationCode,
	}

	#[pallet::storage]
	pub(super) type UpcomingParasGenesis<T: Config> =
		StorageValue<_, ParaGenesisArgs>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_schedule_para_initialize(
			origin: OriginFor<T>,
			genesis: ParaGenesisArgs,
			
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

            UpcomingParasGenesis::<T>::put( genesis.clone());

            Self::deposit_event(Event::Created (who));
        

			Ok(())
		}
	}
}
