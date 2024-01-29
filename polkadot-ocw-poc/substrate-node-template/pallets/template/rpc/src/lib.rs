use codec::{Codec, Decode, Encode};
use core::fmt::Display;
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use pallet_template::{RequestId, PENDING_AUTHORIZED_CONDUIT_NODES_STORAGE, PUBLIC_ENDPOINT_STORAGE, BOOTNODES_STORAGE};
use parking_lot::RwLock;
use sc_rpc_api::DenyUnsafe;
use sp_core::offchain::OffchainStorage;
use std::{collections::BTreeMap, sync::Arc};
use sc_network::{config::MultiaddrWithPeerId, multiaddr::Protocol};

#[rpc(client, server)]
pub trait TemplateApi<AccountId> {
	#[method(name = "template_authorizeNode")]
	fn authorize_node(&self, account: AccountId, request_id: RequestId) -> RpcResult<()>;
	#[method(name = "template_setBootnodes")]
	fn set_bootnodes(&self, address: String) -> RpcResult<()>;
	#[method(name = "template_setPublicEndpoint")]
	fn set_public_endpoint(&self, endpoint: String) -> RpcResult<()>;
}

pub struct TemplateImpl<T: OffchainStorage> {
	storage: Arc<RwLock<T>>,
	deny_unsafe: DenyUnsafe,
}

impl<T: OffchainStorage> TemplateImpl<T> {
	pub fn new(storage: T, deny_unsafe: DenyUnsafe) -> Self {
		Self { storage: Arc::new(RwLock::new(storage)), deny_unsafe }
	}
}

impl<T, AccountId> TemplateApiServer<AccountId> for TemplateImpl<T>
where
	T: OffchainStorage + 'static,
	AccountId: Clone + Display + Codec + Send + std::cmp::PartialEq + 'static,
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

		self.storage.write().set(
			sp_offchain::STORAGE_PREFIX,
            PUBLIC_ENDPOINT_STORAGE,
            &serialized,
        );

        Ok(())
	}

	fn set_bootnodes(&self, address: String) -> RpcResult<()> {
		MultiaddrWithPeerId::try_from(address.clone()).map_err(|e| JsonRpseeError::Custom(e.to_string()))?;

		let serialized = address.encode();

		self.storage.write().set(
			sp_offchain::STORAGE_PREFIX,
            BOOTNODES_STORAGE,
            &serialized,
        );

        Ok(())
	}
}
