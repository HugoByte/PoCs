#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
	pallet_prelude::BlockNumberFor,
};

use frame_system::offchain::{Account, ForAll};
use sp_core::crypto::{AccountId32, KeyTypeId};
use sp_runtime::offchain::storage::StorageValueRef;
use sp_std::{collections::btree_map::BTreeMap, prelude::ToOwned, vec, vec::Vec};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
pub const PENDING_AUTHORIZED_CONDUIT_NODES_STORAGE: &[u8] =
	b"pallet_template::pending_authorized_conduit_nodes";

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

	pub type RequestId = u64;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	pub enum EnclaveAction {
		CreateEnclave {/* Enclave Specification */}, // Provider
		SetupEnclave {/* Setup Configurations */},   // Conduit
		ExecuteInEnclave {/* Commands */},           // Conduit
		StopEnclave,                                 // Provider
		StartEnclave,                                // Provider
		RemoveEnclave,                               // Provider
	}

	// Simple Outcome for now
	#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, Eq, MaxEncodedLen, TypeInfo)]
	pub enum Outcome<T> {
		EnclaveCreated { handle: T },
	}

	#[derive(
		Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo,
	)]
	pub struct Provider {
		// details such as capacity, status, etc
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	pub struct EnclaveRequest<T> {
		user: T,
		handler: Option<T>,
		action: EnclaveAction,
		// other details such as type of environment
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	pub struct AcknowledgedRequest<T, U> {
		request: EnclaveRequest<T>,
		start_block: U,
		handler: T,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxEnclaveCount: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn request_counter)]
	pub type RequestCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn providers)]
	pub(super) type Providers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Provider, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn enclave_requests)]
	pub type EnclaveRequests<T: Config> =
		StorageMap<_, Blake2_128Concat, RequestId, EnclaveRequest<T::AccountId>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn acknowledged_requests)]
	pub type AcknowledgedRequests<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RequestId,
		AcknowledgedRequest<T::AccountId, BlockNumberFor<T>>,
		OptionQuery,
	>;

	#[derive(
		Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo,
	)]
	pub struct EnclaveInfo<T> {
		provider: T,
		user: T,
	}

	#[pallet::storage]
	#[pallet::getter(fn enclaves)]
	pub type Enclaves<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, EnclaveInfo<T::AccountId>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn provider_enclaves)]
	pub type ProviderEnclaves<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		WeakBoundedVec<T::AccountId, T::MaxEnclaveCount>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProviderEnrolled(T::AccountId),
		EnclaveRequestCreated(u64),
		EnclaveRequestAcknowledged(u64),
		EnclaveRequestProcessed {
			request_id: u64,
			handle: T::AccountId,
			outcome: Outcome<T::AccountId>,
		},
		EnclaveCreated {
			user: T::AccountId,
			handler: T::AccountId,
			enclave_port: u32,
		},
		IntializeDeployment {},
		DeploymentCompleted {},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		NotAProvider,
		AlreadyAProvider,
		NotAuthorizedHandler,
		RequestNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn enroll_as_provider(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Providers::<T>::contains_key(&who), Error::<T>::AlreadyAProvider);
			Providers::<T>::insert(&who, Provider {});

			Self::deposit_event(Event::ProviderEnrolled(who.clone()));

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn create_enclave_request(
			origin: OriginFor<T>,
			handler: Option<T::AccountId>,
			action: EnclaveAction,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let request_id = Self::next_request_id();
			let request = EnclaveRequest { user: who, handler, action };

			EnclaveRequests::<T>::insert(request_id, request);

			Self::deposit_event(Event::EnclaveRequestCreated(request_id));

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn acknowledge_enclave_request(
			origin: OriginFor<T>,
			request_id: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			EnclaveRequests::<T>::mutate_exists(request_id, |r| {
				let request = r.as_mut().ok_or(Error::<T>::RequestNotFound)?;

				ensure!(
					request.handler.is_none() || *request.handler.as_ref().unwrap() == who,
					Error::<T>::NotAuthorizedHandler
				);

				let acknowledged_request = AcknowledgedRequest {
					request: request.to_owned(),
					start_block: <frame_system::Pallet<T>>::block_number(),
					handler: who,
				};

				AcknowledgedRequests::<T>::insert(request_id, acknowledged_request);
				*r = None;

				Self::deposit_event(Event::EnclaveRequestAcknowledged(request_id));

				Ok(())
			})
		}

		#[pallet::weight(0)]
		#[pallet::call_index(3)]
		pub fn process_enclave_request(
			origin: OriginFor<T>,
			request_id: u64,
			outcome: Outcome<T::AccountId>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			AcknowledgedRequests::<T>::mutate_exists(request_id, |a| {
				let acknowledged_request = a.as_mut().ok_or(Error::<T>::RequestNotFound)?;

				ensure!(acknowledged_request.handler == who, Error::<T>::NotAuthorizedHandler);

				let dispatch = match acknowledged_request.request.action {
					EnclaveAction::CreateEnclave {} => {
						ensure!(Providers::<T>::contains_key(&who), Error::<T>::NotAProvider);

						match outcome {
							Outcome::EnclaveCreated { ref handle } => {
								let enclave_info = EnclaveInfo {
									provider: acknowledged_request.handler.clone(),
									user: acknowledged_request.request.user.clone(),
								};

								Enclaves::<T>::insert(&handle, enclave_info);

								Self::create_enclave_request(
									OriginFor::<T>::from(Some(who.clone()).into()),
									Some(handle.to_owned()),
									EnclaveAction::SetupEnclave {},
								)
							},
						}
					},
					_ => Ok(()),
				};

				*a = None;

				Self::deposit_event(Event::EnclaveRequestProcessed {
					request_id,
					handle: who,
					outcome,
				});

				dispatch
			})
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		<T as frame_system::Config>::RuntimeEvent: From<pallet::Event<T>>,
		<T as frame_system::Config>::RuntimeEvent: TryInto<pallet::Event<T>>,
	{
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			let storage_ref = StorageValueRef::persistent(PENDING_AUTHORIZED_CONDUIT_NODES_STORAGE);

			// Potential Race condition should optimize
			if let Ok(Some(mut authorized_nodes)) = storage_ref.get::<BTreeMap<u64, T::AccountId>>()
			{
				let mut processed_requests = Vec::new();

				for (request_id, account_id) in authorized_nodes.clone().into_iter() {
					Self::process_acknowledged_request_with_authorization(
						request_id,
						|request, signer| match request.action {
							EnclaveAction::CreateEnclave {} => {
								let tx_results = signer.send_signed_transaction(|_| {
									Call::process_enclave_request {
										request_id,
										outcome: Outcome::EnclaveCreated {
											handle: account_id.clone(),
										},
									}
								});
							},
							_ => {},
						},
					);

					processed_requests.push(request_id);
				}

				for request_id in processed_requests {
					authorized_nodes.remove(&request_id);
				}

				let serialized = authorized_nodes.encode();
				storage_ref.set(&serialized);
			}

			for (index, event) in frame_system::Pallet::<T>::read_events_no_consensus().enumerate()
			{
				match event.event.try_into() {
					Ok(Event::<T>::EnclaveRequestCreated(id)) => {
						Self::request_with_authorization(id, |request, signer| {
							let tx_results = signer.send_signed_transaction(|_| {
								Call::acknowledge_enclave_request { request_id: id }
							});
						});
					},
					Ok(Event::<T>::EnclaveRequestAcknowledged(id)) => {
						Self::process_acknowledged_request_with_authorization(
							id,
							|request, signer| match request.action {
								EnclaveAction::CreateEnclave {} =>
									kurtosis::kurtosis::create_enclave(),
								EnclaveAction::SetupEnclave {} => {},
								_ => {},
							},
						);
					},
					_ => {}, // Ignore other events
				}
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn next_request_id() -> u64 {
			let next_request = RequestCounter::<T>::get().wrapping_add(1);
			RequestCounter::<T>::put(next_request);

			next_request
		}

		pub fn request_with_authorization<F>(request_id: u64, f: F)
		where
			F: FnOnce(EnclaveRequest<T::AccountId>, Signer<T, <T as Config>::AuthorityId, ForAll>),
		{
			if let Some(request) = EnclaveRequests::<T>::get(request_id) {
				match request.action {
					EnclaveAction::CreateEnclave {} => {
						let signer = Signer::<T, T::AuthorityId>::all_accounts();

						if let Some(ref account) = signer
							.accounts_from_keys()
							.find(|account| Providers::<T>::contains_key(&account.id))
						{
							if (request.handler.is_none() ||
								*request.handler.as_ref().unwrap() == account.id.clone())
							{
								f(
									request,
									Signer::<T, T::AuthorityId>::all_accounts()
										.with_filter(vec![account.public.clone()]),
								)
							};
						};
					},
					_ => {},
				};
			};
		}

		pub fn process_acknowledged_request_with_authorization<F>(request_id: u64, f: F)
		where
			F: FnOnce(EnclaveRequest<T::AccountId>, Signer<T, <T as Config>::AuthorityId, ForAll>),
		{
			if let Some(acknowledged_request) = AcknowledgedRequests::<T>::get(request_id) {
				let signer = Signer::<T, T::AuthorityId>::all_accounts();

				if let Some(ref account) = signer
					.accounts_from_keys()
					.find(|account| acknowledged_request.handler == account.id)
				{
					f(
						acknowledged_request.request,
						Signer::<T, T::AuthorityId>::all_accounts()
							.with_filter(vec![account.public.clone()]),
					)
				};
			};
		}

		fn create_enclave() {}
	}
}
