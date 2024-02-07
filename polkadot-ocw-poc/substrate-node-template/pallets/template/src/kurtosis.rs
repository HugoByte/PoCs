use codec::{Codec, Decode, Encode};
use frame_support::traits::IsType;
use scale_info::prelude::string::String;
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use futures::{Future, FutureExt, TryFutureExt};

#[cfg(feature = "std")]
use tokio::sync::Mutex;

#[cfg(feature = "std")]
use parking_lot::ReentrantMutex;

#[cfg(feature = "std")]
use kurtosis_sdk::{
	enclave_api::{
		api_container_service_client::ApiContainerServiceClient,
		run_starlark_package_args::StarlarkPackageContent,
		starlark_run_response_line::RunResponseLine::InstructionResult,
		starlark_run_response_line::RunResponseLine::RunFinishedEvent, RunStarlarkPackageArgs,
		RunStarlarkScriptArgs,
	},
	engine_api::{engine_service_client::EngineServiceClient, CreateEnclaveArgs},
};

#[cfg(feature = "std")]
use sp_core::offchain::{OffchainDbExt, StorageKind};

#[cfg(feature = "std")]
use sp_core::traits::SpawnNamed;

#[cfg(feature = "std")]
use sp_externalities::ExternalitiesExt;
use sp_runtime::WeakBoundedVec;
use sp_std::collections::btree_map::BTreeMap;

#[cfg(feature = "std")]
use tokio::sync::Notify;

#[cfg(feature = "std")]
use std::sync::mpsc;

#[cfg(feature = "std")]
use tokio::time::{sleep, timeout, Duration};

use core::{future::IntoFuture, pin::Pin};
use sp_core::ConstU32;
use sp_runtime_interface::runtime_interface;
use sp_std::{any::Any, boxed::Box, sync::Arc};

#[cfg(feature = "std")]
use async_trait::async_trait;

use crate::RequestId;

#[cfg(feature = "std")]
pub trait KurtosisClientTrait: Any + Send + Sync {
	fn as_any(&self) -> &dyn Any;
}

#[derive(Serialize, Deserialize)]
pub struct NodeArgs {
	request_id: u64,
	provider_url: String,
	api_container_host: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackageParams {
	node_type: String,
	node_args: NodeArgs,
	bootnodes: String,
}

#[cfg(feature = "std")]
pub enum KurtosisClientState<T> {
	Failed(tonic::transport::Error),
	Pending(Pin<Box<dyn Future<Output = Result<T, tonic::transport::Error>> + Send + 'static>>),
	Ready(T),
	Uninitialized,
}

#[cfg(feature = "std")]
pub struct KurtosisClient<T> {
	client: Arc<Mutex<KurtosisClientState<T>>>,
}

#[cfg(feature = "std")]
pub struct KurtosisContainer {
	client: Arc<dyn KurtosisClientTrait + Send + Sync>,
	spawner: Box<dyn SpawnNamed>,
}

#[cfg(feature = "std")]
impl KurtosisContainer {
	pub fn new(
		client: Arc<impl KurtosisClientTrait + std::marker::Sync + std::marker::Send + 'static>,
		spawner: impl SpawnNamed + 'static,
	) -> Self {
		Self { client, spawner: Box::new(spawner) }
	}

	pub fn engine_service(
		&self,
	) -> Option<&KurtosisClient<EngineServiceClient<tonic::transport::Channel>>> {
		self.client
			.as_any()
			.downcast_ref::<KurtosisClient<EngineServiceClient<tonic::transport::Channel>>>()
	}

	pub fn api_container_service(
		&self,
	) -> Option<&KurtosisClient<ApiContainerServiceClient<tonic::transport::Channel>>> {
		self.client
			.as_any()
			.downcast_ref::<KurtosisClient<ApiContainerServiceClient<tonic::transport::Channel>>>()
	}
}

#[cfg(feature = "std")]
impl KurtosisClient<EngineServiceClient<tonic::transport::Channel>> {
	pub fn new_with_engine(host: Option<String>) -> Arc<Self> {
		let future = async move {
			EngineServiceClient::connect(host.unwrap_or("https://[::1]:9710".to_string())).await
		};

		Arc::new(Self {
			client: Arc::new(Mutex::new(KurtosisClientState::Pending(Box::pin(future)))),
		})
	}
}

#[cfg(feature = "std")]
impl KurtosisClientTrait for KurtosisClient<EngineServiceClient<tonic::transport::Channel>> {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

#[cfg(feature = "std")]
impl KurtosisClient<ApiContainerServiceClient<tonic::transport::Channel>> {
	pub fn new_with_api_container(host: Option<String>) -> Arc<Self> {
		let future = async move {
			ApiContainerServiceClient::connect(host.unwrap_or("https://[::1]:7443".to_string()))
				.await
		};

		Arc::new(Self {
			client: Arc::new(Mutex::new(KurtosisClientState::Pending(Box::pin(future)))),
		})
	}
}

#[cfg(feature = "std")]
impl KurtosisClientTrait for KurtosisClient<ApiContainerServiceClient<tonic::transport::Channel>> {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

#[cfg(feature = "std")]
impl<T> KurtosisClient<T>
where
	T: Send + Sync + 'static,
{
	pub fn initialize(&self, spawner: impl SpawnNamed + 'static) {
		let client = self.client.clone();

		spawner.spawn(
			"kurtosis-client-init",
			None,
			Box::pin(async move {
				log::info!("Kurtosis client is initializing.");
				let mut client_lock = client.lock().await;
				match &mut *client_lock {
					KurtosisClientState::Pending(ref mut future) => match future.as_mut().await {
						Ok(client) => {
							log::info!("Kurtosis client is ready.");
							*client_lock = KurtosisClientState::Ready(client);
						},
						Err(e) => {
							log::error!("Kurtosis client failed to initialize: {:?}", e);
							*client_lock = KurtosisClientState::Failed(e);
						},
					},
					_ => log::warn!("Kurtosis client is not in a pending state."),
				}
			}),
		);
	}
}

#[cfg(feature = "std")]
#[async_trait]
pub trait EngineClientTrait {
	async fn with_client<F, U>(&self, f: F) -> Result<U, String>
	where
		F: FnOnce(EngineServiceClient<tonic::transport::Channel>) -> U + std::marker::Send;
}

#[cfg(feature = "std")]
#[async_trait]
impl EngineClientTrait for KurtosisClient<EngineServiceClient<tonic::transport::Channel>> {
	async fn with_client<F, U>(&self, f: F) -> Result<U, String>
	where
		F: FnOnce(EngineServiceClient<tonic::transport::Channel>) -> U + std::marker::Send,
	{
		let client = timeout(Duration::from_secs(30), async {
			loop {
				let client_state = self.client.lock().await;
				match &*client_state {
					KurtosisClientState::Ready(client) => break client.clone(),
					KurtosisClientState::Uninitialized | KurtosisClientState::Failed(_) => {
						sleep(Duration::from_millis(1000)).await;
						continue;
					},
					_ => {},
				}
			}
		})
		.await
		.map_err(|_| "Timeout waiting for client to be ready".to_string())?;

		Ok(f(client))
	}
}

#[cfg(feature = "std")]
#[async_trait]
pub trait ApiContainerClientTrait {
	async fn with_client<F, U>(&self, f: F) -> Result<U, String>
	where
		F: FnOnce(ApiContainerServiceClient<tonic::transport::Channel>) -> U + std::marker::Send;
}

#[cfg(feature = "std")]
#[async_trait]
impl ApiContainerClientTrait
	for KurtosisClient<ApiContainerServiceClient<tonic::transport::Channel>>
{
	async fn with_client<F, U>(&self, f: F) -> Result<U, String>
	where
		F: FnOnce(ApiContainerServiceClient<tonic::transport::Channel>) -> U + std::marker::Send,
	{
		let client = timeout(Duration::from_secs(30), async {
			loop {
				let client_state = self.client.lock().await;
				match &*client_state {
					KurtosisClientState::Ready(client) => break client.clone(),
					KurtosisClientState::Uninitialized | KurtosisClientState::Failed(_) => {
						sleep(Duration::from_millis(1000)).await;
						continue;
					},
					_ => {},
				}
			}
		})
		.await
		.map_err(|_| "Timeout waiting for client to be ready".to_string())?;

		Ok(f(client))
	}
}

#[cfg(feature = "std")]
sp_externalities::decl_extension! {
	pub struct KurtosisExt(Arc<KurtosisContainer>);
}

#[cfg(feature = "std")]
impl KurtosisExt {
	pub fn new(client: Arc<KurtosisContainer>) -> Self {
		Self(client)
	}
}

#[cfg(feature = "std")]
pub type HostFunctions = (kurtosis::HostFunctions,);

#[runtime_interface]
pub trait Kurtosis {
	fn create_enclave(&mut self, request_id: RequestId) {
		#[cfg(not(test))]
		if let Some(kurtosis_ext) = self.extension::<KurtosisExt>() {
			let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

			let response = rt.block_on(async {
				let engine_service =
					kurtosis_ext.0.engine_service().expect("Failed to get engine service");
				(engine_service
					.with_client(|mut client| async move {
						client
							.create_enclave(CreateEnclaveArgs {
								enclave_name: None,
								api_container_log_level: Some("info".to_string()),
								api_container_version_tag: None,
								mode: Some(0),
							})
							.await
							.unwrap()
							.into_inner()
					})
					.await
					.unwrap()
					.await)
			});

			let spawner = kurtosis_ext.0.spawner.clone();
			rt.block_on(async {
				let enclave = response
					.enclave_info
					.expect("Enclave info must be present")
					.api_container_info
					.expect("Enclave info must be present");

				let api_container_service = KurtosisClient::<
					ApiContainerServiceClient<tonic::transport::Channel>,
				>::new_with_api_container(Some(format!(
					"https://{}:{}",
					enclave.ip_inside_enclave.clone(),
					enclave.grpc_port_inside_enclave.clone()
				)));

				api_container_service.initialize(spawner);

				let offchain_db_ext = self.extension::<OffchainDbExt>().unwrap();
				let endpoint: String = offchain_db_ext
					.local_storage_get(StorageKind::PERSISTENT, crate::PUBLIC_ENDPOINT_STORAGE)
					.and_then(|bytes| Decode::decode(&mut &bytes[..]).ok())
					.unwrap();

				let bootnodes: String = offchain_db_ext
					.local_storage_get(StorageKind::PERSISTENT, crate::BOOTNODES_STORAGE)
					.and_then(|bytes| Decode::decode(&mut &bytes[..]).ok())
					.unwrap();

				let package_params = PackageParams {
					node_type: "conduit".to_string(),
					node_args: NodeArgs {
						provider_url: endpoint,
						request_id,
						api_container_host: format!(
							"https://{}:{}",
							enclave.ip_inside_enclave.clone(),
							enclave.grpc_port_inside_enclave.clone()
						),
					},
					bootnodes,
				};

				let mut result = api_container_service
					.with_client(|mut client| async move {
						client
							.run_starlark_package(RunStarlarkPackageArgs {
								package_id: "github.com/hugobyte/pocs/polkadot-ocw-poc/substrate-node-template".to_string(),
								parallelism: Some(4),
								serialized_params: Some(serde_json::to_string(&package_params).unwrap()),
								dry_run: None,
								clone_package: Some(true),
								relative_path_to_main_file: Some("./main.star".to_string()),
								main_function_name: Some("run".to_string()),
								experimental_features: vec![],
								cloud_instance_id: None,
								cloud_user_id: None,
								image_download_mode: None,
								starlark_package_content: Some(StarlarkPackageContent::Remote(true)),
							})
							.await
							.unwrap()
							.into_inner()
					})
					.await
					.unwrap()
					.await;

				while let Some(next_message) = result.message().await.unwrap() {
					next_message.run_response_line.map(|line| match line {
						InstructionResult(result) => {
							println!("{}", result.serialized_instruction_result)
						},
						_ => (),
					});
				}
			});
		}
	}

	fn setup_enclave(
		&mut self,
		setup_script: Option<WeakBoundedVec<u8, ConstU32<{ u32::MAX }>>>,
	) -> Result<(), ()> {
		#[cfg(not(test))]
		if let Some(kurtosis_ext) = self.extension::<KurtosisExt>() {
			let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
			let api_container_service = kurtosis_ext
				.0
				.api_container_service()
				.expect("Failed to get api container service");

			rt.block_on(async {
				let mut result = api_container_service
					.with_client(|mut client| async move {
						client
							.run_starlark_script(RunStarlarkScriptArgs {
								parallelism: Some(4),
								serialized_script: String::from_utf8(
									setup_script
										.expect("Need script for setup")
										.into_iter()
										.collect(),
								)
								.unwrap_or_else(|_| String::from("Invalid UTF-8")),
								serialized_params: Some("{}".to_string()),
								dry_run: None,
								main_function_name: Some("run".to_string()),
								experimental_features: vec![],
								cloud_instance_id: None,
								cloud_user_id: None,
								image_download_mode: None,
							})
							.await
							.unwrap()
							.into_inner()
					})
					.await
					.unwrap()
					.await;

				while let Some(next_message) = result.message().await.unwrap() {
					next_message.run_response_line.map(|line| match line {
						InstructionResult(result) => {
							println!("{}", result.serialized_instruction_result)
						},
						_ => (),
					});
				}
			});
		}

		Ok(())
	}

	fn execute_in_enclave(
		&mut self,
		setup_script: Option<WeakBoundedVec<u8, ConstU32<{ u32::MAX }>>>,
	) -> Result<WeakBoundedVec<u8, ConstU32<{ u32::MAX }>>, ()> {
		let (tx, rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();

		let kurtosis_ext = self.extension::<KurtosisExt>().unwrap().clone();
		let spawner = kurtosis_ext.spawner.clone();

		#[cfg(not(test))]
		spawner.spawn(
			"kurtosis-client-execute-in-enclave",
			None,
			Box::pin(async move {
				let mut result = kurtosis_ext
					.api_container_service()
					.unwrap()
					.with_client(|mut client| async move {
						client
							.run_starlark_script(RunStarlarkScriptArgs {
								parallelism: Some(4),
								serialized_script: String::from_utf8(
									setup_script
										.expect("Need script for setup")
										.into_iter()
										.collect(),
								)
								.unwrap_or_else(|_| String::from("Invalid UTF-8")),
								serialized_params: Some("{}".to_string()),
								dry_run: None,
								main_function_name: Some("run".to_string()),
								experimental_features: vec![0],
								cloud_instance_id: None,
								cloud_user_id: None,
								image_download_mode: None,
							})
							.await
							.unwrap()
							.into_inner()
					})
					.await
					.unwrap()
					.await;

				let mut results = Vec::new();
				while let Some(next_message) = result.message().await.unwrap() {
					if let Some(line) = next_message.run_response_line {
						match line {
							RunFinishedEvent(result) => {
								results.extend_from_slice(
									result.serialized_output.unwrap_or_default().as_bytes(),
								);
							},
							_ => (),
						}
					}
				}

				let _ = tx.send(results);
			}),
		);

		let results = rx.recv().map_err(|_| ())?;

		WeakBoundedVec::<u8, ConstU32<{ u32::MAX }>>::try_from(results).map_err(|_| ())
	}
}
