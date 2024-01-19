use frame_support::traits::IsType;

#[cfg(feature = "std")]
use futures::{
	lock::{MappedMutexGuard, Mutex, MutexGuard},
	Future, FutureExt, TryFutureExt,
};

use kurtosis_sdk::enclave_api::RunStarlarkPackageArgs;
#[cfg(feature = "std")]
use kurtosis_sdk::{
	enclave_api::{
		api_container_service_client::ApiContainerServiceClient,
		run_starlark_package_args::StarlarkPackageContent,
		starlark_run_response_line::RunResponseLine::InstructionResult, RunStarlarkScriptArgs,
	},
	engine_api::{engine_service_client::EngineServiceClient, CreateEnclaveArgs},
};

#[cfg(feature = "std")]
use sp_core::traits::SpawnNamed;

#[cfg(feature = "std")]
use sp_externalities::ExternalitiesExt;

#[cfg(feature = "std")]
use tokio::sync::Notify;

use core::{future::IntoFuture, pin::Pin};
use sp_runtime_interface::runtime_interface;
use sp_std::{any::Any, boxed::Box, sync::Arc};

#[cfg(feature = "std")]
use async_trait::async_trait;

#[cfg(feature = "std")]
pub trait KurtosisClientTrait: Any + Send + Sync {
	fn as_any(&self) -> &dyn Any;
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
	spawner: Box<dyn SpawnNamed>,
	state_changed: Arc<Notify>,
}

#[cfg(feature = "std")]
pub struct KurtosisContainer(Arc<dyn KurtosisClientTrait + Send + Sync>);

#[cfg(feature = "std")]
impl KurtosisContainer {
	pub fn new(
		client: Arc<impl KurtosisClientTrait + std::marker::Sync + std::marker::Send + 'static>,
	) -> Self {
		Self(client)
	}

	pub fn engine_service(
		&self,
	) -> Option<&KurtosisClient<EngineServiceClient<tonic::transport::Channel>>> {
		self.0
			.as_any()
			.downcast_ref::<KurtosisClient<EngineServiceClient<tonic::transport::Channel>>>()
	}

	pub fn api_container_service(
		&self,
	) -> Option<&KurtosisClient<ApiContainerServiceClient<tonic::transport::Channel>>> {
		self.0
			.as_any()
			.downcast_ref::<KurtosisClient<ApiContainerServiceClient<tonic::transport::Channel>>>()
	}
}

#[cfg(feature = "std")]
impl KurtosisClient<EngineServiceClient<tonic::transport::Channel>> {
	pub fn new_with_engine(spawner: impl SpawnNamed + 'static) -> Self {
		let future = async move { EngineServiceClient::connect("https://[::1]:9710").await };

		Self {
			client: Arc::new(Mutex::new(KurtosisClientState::Pending(Box::pin(future)))),
			spawner: Box::new(spawner),
			state_changed: Arc::new(Notify::new()),
		}
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
	pub fn new_with_api_container(port: u32, spawner: impl SpawnNamed + 'static) -> Arc<Self> {
		let future = async move {
			ApiContainerServiceClient::connect(format!("https://[::1]:{}", port)).await
		};

		Arc::new(Self {
			client: Arc::new(Mutex::new(KurtosisClientState::Pending(Box::pin(future)))),
			spawner: Box::new(spawner),
			state_changed: Arc::new(Notify::new()),
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
	T: Send + 'static,
{
	pub fn initialize(&self) {
		let client_clone = self.client.clone();
		let state_changed = self.state_changed.clone();

		self.spawner.spawn(
			"kurtosis-client-init",
			None,
			Box::pin(async move {
				log::info!("Kurtosis client is initializing.");
				let mut client_lock = client_clone.lock().await;
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

				state_changed.notify_one();
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
		let client_state = self.client.lock().await;
		loop {
			match &*client_state {
				KurtosisClientState::Uninitialized =>
					break Err("Client has not been initialized".to_string()),
				KurtosisClientState::Ready(client) => break Ok(f(client.clone())),
				KurtosisClientState::Pending(_) => self.state_changed.notified().await,
				KurtosisClientState::Failed(reason) =>
					break Err(format!("Client initialization failed: {:?}", reason)),
			}
		}
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
		self.state_changed.notified().await;
		let client_state = self.client.lock().await;
		loop {
			match &*client_state {
				KurtosisClientState::Uninitialized =>
					break Err("Client has not been initialized".to_string()),
				KurtosisClientState::Ready(client) => break Ok(f(client.clone())),
				KurtosisClientState::Pending(_) => self.state_changed.notified().await,
				KurtosisClientState::Failed(reason) =>
					break Err(format!("Client initialization failed: {:?}", reason)),
			}
		}
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

const STARTUP_SCRIPT: &str = r#"
def main(plan):
    plan.print('Hello World!')
"#;

#[runtime_interface]
pub trait Kurtosis {
	fn create_enclave(&mut self) {
		if let Some(kurtosis_ext) = self.extension::<KurtosisExt>() {
			let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

			rt.block_on(async {
				let engine_service =
					kurtosis_ext.0.engine_service().expect("Failed to get engine service");
				let response = engine_service
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
					.await;

				let enclave_port = response
					.enclave_info
					.expect("Enclave info must be present")
					.api_container_host_machine_info
					.expect("Enclave host machine info must be present")
					.grpc_port_on_host_machine;

				let api_container_service = KurtosisClient::<
					ApiContainerServiceClient<tonic::transport::Channel>,
				>::new_with_api_container(
					enclave_port, engine_service.spawner.clone()
				);

				api_container_service.initialize();

				let mut result = api_container_service
					.with_client(|mut client| async move {
						client
							.run_starlark_package(RunStarlarkPackageArgs {
								package_id: "github.com/hugobyte/polkadot-kurtosis-package".to_string(),
								parallelism: Some(4),
								serialized_params: Some(r#"{ "chain_type": "local", "relaychain": { "name": "rococo-local", "nodes": [ { "name": "alice", "node_type": "validator", "prometheus": false }, { "name": "bob", "node_type": "full", "prometheus": true } ] }, "parachains": [ { "name": "frequency", "nodes": [ { "name": "alice", "node_type": "validator", "prometheus": false }, { "name": "bob", "node_type": "full", "prometheus": true } ] } ], "explorer": true }"#.to_string()),
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
		} else {
			log::error!("KurtosisExt not found in externalities");
		}
	}
}
