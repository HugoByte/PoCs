use frame_support::traits::IsType;
use futures::{lock::Mutex, Future, FutureExt, TryFutureExt};
use kurtosis_sdk::{
	enclave_api::{
		api_container_service_client::ApiContainerServiceClient,
		starlark_run_response_line::RunResponseLine::InstructionResult, RunStarlarkScriptArgs,
	},
	engine_api::{engine_service_client::EngineServiceClient, CreateEnclaveArgs},
};
use sp_core::traits::SpawnNamed;

#[cfg(feature = "std")]
use sp_externalities::ExternalitiesExt;

use core::future::IntoFuture;
use sp_runtime_interface::runtime_interface;
use std::{pin::Pin, sync::Arc};

#[cfg(feature = "std")]
pub enum KurtosisEngineState {
	Failed(tonic::transport::Error),
	Pending(
		Pin<
			Box<
				dyn Future<
						Output = Result<
							EngineServiceClient<tonic::transport::Channel>,
							tonic::transport::Error,
						>,
					> + Send
					+ 'static,
			>,
		>,
	),
	Ready(EngineServiceClient<tonic::transport::Channel>),
}

#[cfg(feature = "std")]
pub struct KurtosisClient {
	pub engine: Arc<Mutex<KurtosisEngineState>>,
}

impl KurtosisClient {
	pub fn new() -> Arc<Self> {
		let future = async { EngineServiceClient::connect("https://[::1]:9710").await };

		Arc::new(Self {
			engine: Arc::new(Mutex::new(KurtosisEngineState::Pending(Box::pin(future)))),
		})
	}

	pub fn initialize(&self, spawner: impl SpawnNamed + 'static) {
		let engine_clone = self.engine.clone();
		log::info!("Kurtosis client is initializing.");
		spawner.spawn(
			"kurtosis-engine-init",
			None,
			Box::pin(async move {
				let mut engine_lock = engine_clone.lock().await;
				if let KurtosisEngineState::Pending(ref mut future) = *engine_lock {
					match future.as_mut().await {
						Ok(client) => *engine_lock = {
							log::error!("Kurtosis client is ready");
							KurtosisEngineState::Ready(client)
						},
						Err(e) => *engine_lock = { 
							log::error!("Kurtosis client failed to initialize: {:?}", e);
							KurtosisEngineState::Failed(e)
						},
					}
				}
			}),
		);
	}
}

#[cfg(feature = "std")]
sp_externalities::decl_extension! {
	pub struct KurtosisExt(Arc<KurtosisClient>);
}

#[cfg(feature = "std")]
impl KurtosisExt {
	pub fn new(client: Arc<KurtosisClient>) -> Self {
		Self(client)
	}
}

#[cfg(feature = "std")]
pub type HostFunctions = (kurtosis::HostFunctions,);

const STARLARK_SCRIPT: &str = "
def main(plan):
    plan.print('Hello World!')
";

// #[cfg(feature = "std")]
// async fn deploy(engine: &mut EngineServiceClient<tonic::transport::Channel>) {
// 	let create_enclave_response = engine
// 		.create_enclave(CreateEnclaveArgs {
// 			enclave_name: "my-rust-test".to_string(),
// 			api_container_log_level: "info".to_string(),
// 			// Default
// 			api_container_version_tag: "".to_string(),
// 			is_partitioning_enabled: false,
// 		})
// 		.await?
// 		.into_inner();

// 	log::info!("from tokio before sleep");
// 	tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
// 	log::info!("from tokio after sleep");
// }

#[runtime_interface]
pub trait Kurtosis {
	fn call_async(&mut self) {
		if let Some(kurtosis_ext) = self.extension::<KurtosisExt>() {
			let engine_lock = kurtosis_ext.0.engine.clone();
			let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

			rt.block_on(async {
				let engine_state = engine_lock.lock().await;
				match &*engine_state {
					KurtosisEngineState::Ready(client) => {
						log::info!("Kurtosis client is ready.");
					},
					KurtosisEngineState::Pending(_) => {
						log::warn!("Kurtosis client is still pending.");
					},
					KurtosisEngineState::Failed(e) => {
						log::error!("Kurtosis client failed: {:?}", e);
					},
				}
			});
		} else {
			log::error!("KurtosisExt not found in externalities");
		}
	}
	// fn engine(&mut self) -> Option<Box<EngineServiceClient<tonic::transport::Channel>>> {}

	// fn call_async(&mut self) {
	// 	if let Some(mut engine) = self.extension::<KurtosisExt>().map(|ext| ext.0) {
	// 		let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
	// 		rt.block_on(deploy(&mut engine));
	// 	}
	// }
}
