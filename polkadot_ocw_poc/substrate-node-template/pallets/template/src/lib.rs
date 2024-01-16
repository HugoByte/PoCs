#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
	pallet_prelude::BlockNumberFor,
};

use sp_core::crypto::KeyTypeId;
use sp_runtime::offchain::storage::StorageValueRef;
use sp_std::vec::Vec;
use sp_core::crypto::AccountId32;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
pub const PENDING_AUTHORIZED_NODES_STORAGE: &[u8] = b"pallet_template::pending_authorized_nodes";

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct AuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for AuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

pub use pallet::*;

pub mod kurtosis;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
		CreateEnclaveRequest {},
		EnclaveCreateRequestAcknowledge {},
		EnclaveCreated { user: T::AccountId, operator: T::AccountId, enclave_port: u32 },
		IntializeDeployment {},
		DeploymentCompleted {},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Something::<T>::put(something);

			Self::deposit_event(Event::SomethingStored { something, who });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			match Pallet::<T>::something() {
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Something::<T>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
        pub fn create_enclave_request(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Emit the CreateEnclaveRequest event
            Self::deposit_event(Event::CreateEnclaveRequest {});

            Ok(())
        }

		#[pallet::weight(0)]
		#[pallet::call_index(3)]
		pub fn deploy(origin: OriginFor<T>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			Self::deposit_event(Event::IntializeDeployment {});

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		<T as frame_system::Config>::RuntimeEvent: From<pallet::Event<T>>,
		<T as frame_system::Config>::RuntimeEvent: TryInto<pallet::Event<T>>,
	{
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			let signers = Signer::<T, T::AuthorityId>::all_accounts();
			if !signers.can_sign() {
				log::info!("No local accounts available");
			}

			let storage_ref = StorageValueRef::persistent(PENDING_AUTHORIZED_NODES_STORAGE);

			if let Ok(Some(authorized_nodes)) = storage_ref.get::<Vec<AccountId32>>() {
				if authorized_nodes.len() > 0 {
					// Add to storage along with enclave info
				}
			}

			for (index, event) in frame_system::Pallet::<T>::read_events_no_consensus().enumerate()
			{
				match event.event.try_into() {
					Ok(Event::<T>::IntializeDeployment {}) => {
						log::info!("Initialize deployment called");
					},
					Ok(Event::<T>::CreateEnclaveRequest {}) => {
						log::info!("CreateEnclaveRequest event detected");
						
						// Create enclave and deploy conduit node and pass enclave info to conduit node
					},
					_ => {} // Ignore other events
				}
			}
		}
	}
}
