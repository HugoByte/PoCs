#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use sp_core::crypto::KeyTypeId;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;

	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::dispatch::Vec;
	use sp_runtime::offchain::{http, storage::StorageValueRef, Duration};

	use frame_support::{pallet_prelude::*, traits::IsType};
	use frame_system::pallet_prelude::*;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct ChainDetails {
		network_type: String,
		chain: String,
		nodes: Vec<NodeDetails>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct NodeDetails {
		node_name: String,
		node_type: String,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	use frame_system::offchain::AppCrypto;

	#[pallet::config]
	pub trait Config:
		frame_system::offchain::CreateSignedTransaction<Call<Self>> + frame_system::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		#[pallet::constant]
		type MaxPings: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Pinged { who: T::AccountId, claim: u32 },
		DeployNode(ChainDetails),
	}

	#[pallet::error]
	pub enum Error<T> {
		HttpFetchingError,
	}


	#[derive(Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Ping(pub u32);

	#[pallet::storage]
	#[pallet::getter(fn pings)]
	pub(super) type Pings<T: Config> = StorageValue<_, BoundedVec<Ping, T::MaxPings>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(0)]
		#[pallet::call_index(1)]
		pub fn increment_ping(origin: OriginFor<T>, claim: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let mut a = Pings::<T>::get();
			a.try_push(Ping(claim));

			Pings::<T>::set(a);
			Self::deposit_event(Event::Pinged { who: sender, claim });

			Ok(())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(2)]
		pub fn node_deploy(origin: OriginFor<T>, node_details: ChainDetails) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			Self::deposit_event(Event::DeployNode(node_details));

			Ok(())
		}
	}

	use frame_system::offchain::Signer;
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		<T as frame_system::Config>::RuntimeEvent: From<pallet::Event<T>>,
		<T as frame_system::Config>::RuntimeEvent: TryInto<pallet::Event<T>>,
	{
		/// Offchain worker entry point.
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			log::info!("Hello from pallet-ocw");
			// let asd = frame_system::Pallet::<T>::read_events_no_consensus();
			for (index, event) in frame_system::Pallet::<T>::read_events_no_consensus().enumerate()
			{
				log::info!("{:?}", index);
				if let Ok(Event::<T>::DeployNode(chain)) = event.event.try_into() {
					let mut nodes = vec![];
					use serde::Serialize;
					#[derive(Serialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
					pub struct Node {
						name: String,
						node_type: String,
						prometheus: bool,
					}

					for i in chain.nodes.iter() {
						// let temp = format!(
						// 	r#"{{"name": "{}", "node_type": "{}" , "prometheus": false}}"#,
						// 	i.node_name, i.node_type
						// );
						let node = Node{
							name: i.node_name.clone(),
							node_type: i.node_type.clone(),
							prometheus: false,
						};
						nodes.push(node);
					}

					let node = serde_json::to_string(&nodes).unwrap();

					let config = format!(
						r#"{{ "chain_type": "{}",
						"relaychain": {{
						  "name": "{}",
						  "nodes": {:?}
						}},
						"para": [],
						"explorer": true
					  }}"#,
						chain.network_type,chain.chain, node
					);

					log::info!("{:?}", config);
					match Self::post_kurtosis(config) {
						Ok(kurtosis) => log::info!("{:?}", kurtosis),
						Err(x) => log::error!("Error: {:?}", x),
					}
				}
			}
		}
	}

	use codec::alloc::string::ToString;
	use frame_system::offchain::*;
	use scale_info::prelude::string::String;
	use scale_info::prelude::vec;
	use scale_info::prelude::format;
	use serde_json::Value;

	impl<T: Config> Pallet<T>
	where
		<T as frame_system::Config>::RuntimeEvent: From<pallet::Event<T>>,
		<T as frame_system::Config>::RuntimeEvent: TryInto<pallet::Event<T>>,
	{

		fn get_kurtosis() -> Result<String, http::Error> {
			let request = http::Request::get("http://127.0.0.1:8080/items");

			let timeout = sp_io::offchain::timestamp().add(Duration::from_millis(240000));

			let pending = request.deadline(timeout).send().map_err(|e| {
				log::error!("11 {:?}", e);
				http::Error::IoError
			})?;

			let response = pending.try_wait(timeout).map_err(|e| {
				log::error!("25 {:?}", e);
				http::Error::IoError
			})??;

			if response.code != 200 {
				log::error!("Unexpected http request status code: {}", response.code);
			}
			log::info!("{:?}", response.body().clone().collect::<Vec<u8>>());
			let body = response.body().collect::<Vec<u8>>();
			let body_str = String::from_utf8_lossy(&body);

			Ok(body_str.to_string())
		}

		fn post_kurtosis(config: String) -> Result<(), http::Error> {
			let body = config.as_bytes();
			let request = http::Request::post("http://127.0.0.1:8080/spawn", vec![body]);

			let timeout = sp_io::offchain::timestamp().add(Duration::from_millis(240000));

			let pending = request.deadline(timeout)
				.send().map_err(|e| {
				log::error!("11 {:?}", e);
				http::Error::IoError
			})?;

			let response = pending.try_wait(timeout).map_err(|e| {
				log::error!("25 {:?}", e);
				http::Error::IoError
			})??;

			if response.code != 200 {
				log::error!("Unexpected http request status code: {}", response.code);
			}
			log::info!("{:?}", response.body().clone().collect::<Vec<u8>>());
			let body = response.body().collect::<Vec<u8>>();

			// let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
			// 	log::warn!("No UTF8 body");
			// 	http::Error::Unknown
			// })?;
			// let body_str: ChainDetails = ChainDetails::decode(&mut &body[..]).unwrap();
			let body_str = String::from_utf8_lossy(&body);
			// let res = serde_json::from_slice::<String>(&body);
			log::info!("hello {:?}", body_str);

			Ok(())
		}
	}
}
