use codec::{Codec, Decode, Encode};
use core::fmt::Display;
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use pallet_template::{
	RequestId, BOOTNODES_STORAGE, PENDING_AUTHORIZED_CONDUIT_NODES_STORAGE, PUBLIC_ENDPOINT_STORAGE,
};
pub use pallet_template_rpc_runtime_api::TemplateRuntimeApi;
use parking_lot::RwLock;
use sc_network::{config::MultiaddrWithPeerId, multiaddr::Protocol};
use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::offchain::OffchainStorage;
use sp_runtime::traits::Block;
use std::{collections::BTreeMap, sync::Arc};

#[rpc(client, server)]
pub trait TemplateApi<AccountId> {
	#[method(name = "template_authorizeNode")]
	fn authorize_node(&self, account: AccountId, request_id: RequestId) -> RpcResult<()>;
	#[method(name = "template_setBootnodes")]
	fn set_bootnodes(&self, address: String) -> RpcResult<()>;
	#[method(name = "template_setPublicEndpoint")]
	fn set_public_endpoint(&self, endpoint: String) -> RpcResult<()>;

	#[method(name = "template_getProviders")]
	fn get_providers(&self) -> RpcResult<Vec<AccountId>>;
	#[method(name = "template_getProviderEnclaves")]
	fn get_provider_enclaves(&self, provider: AccountId) -> RpcResult<Vec<AccountId>>;
}

pub struct TemplateImpl<T: OffchainStorage, C, B> {
	client: Arc<C>,
	storage: Arc<RwLock<T>>,
	deny_unsafe: DenyUnsafe,
	_marker: std::marker::PhantomData<B>,
}

impl<T: OffchainStorage, C, B> TemplateImpl<T, C, B> {
	pub fn new(client: Arc<C>, storage: T, deny_unsafe: DenyUnsafe) -> Self {
		Self {
			client,
			storage: Arc::new(RwLock::new(storage)),
			deny_unsafe,
			_marker: Default::default(),
		}
	}
}

impl<T, AccountId, C, B> TemplateApiServer<AccountId> for TemplateImpl<T, C, B>
where
	T: OffchainStorage + 'static,
	B: Block,
	AccountId: Clone + Display + Codec + Send + std::cmp::PartialEq + 'static,
	C: Send + Sync + 'static + ProvideRuntimeApi<B> + HeaderBackend<B>,
	C::Api: TemplateRuntimeApi<B, AccountId>,
{
	fn authorize_node(&self, account: AccountId, request_id: RequestId) -> RpcResult<()> {
		let account = account.clone();

		let mut pending_authorized_nodes: BTreeMap<RequestId, AccountId> = self
			.storage
			.read()
			.get(sp_offchain::STORAGE_PREFIX, PENDING_AUTHORIZED_CONDUIT_NODES_STORAGE)
			.and_then(|bytes| Decode::decode(&mut &bytes[..]).ok())
			.unwrap_or_default();

		pending_authorized_nodes.insert(request_id, account.clone());

		let serialized = pending_authorized_nodes.encode();

		self.storage.write().set(
			sp_offchain::STORAGE_PREFIX,
			PENDING_AUTHORIZED_CONDUIT_NODES_STORAGE,
			&serialized,
		);

		Ok(())
	}

	fn set_public_endpoint(&self, endpoint: String) -> RpcResult<()> {
		let serialized = endpoint.encode();

		self.storage
			.write()
			.set(sp_offchain::STORAGE_PREFIX, PUBLIC_ENDPOINT_STORAGE, &serialized);

		Ok(())
	}

	fn set_bootnodes(&self, address: String) -> RpcResult<()> {
		MultiaddrWithPeerId::try_from(address.clone())
			.map_err(|e| JsonRpseeError::Custom(e.to_string()))?;

		let serialized = address.encode();

		self.storage
			.write()
			.set(sp_offchain::STORAGE_PREFIX, BOOTNODES_STORAGE, &serialized);

		Ok(())
	}

	fn get_providers(&self) -> RpcResult<Vec<AccountId>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;
		let result = api.get_providers(at).map_err(into_rpc_err);

		Ok(result.unwrap())
	}

	fn get_provider_enclaves(&self, provider: AccountId) -> RpcResult<Vec<AccountId>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;
		let result = api.get_provider_enclaves(at, provider).map_err(into_rpc_err);

		Ok(result.unwrap())
	}
}

fn into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(1, "Runtime error", Some(format!("{:?}", err)))).into()
}
