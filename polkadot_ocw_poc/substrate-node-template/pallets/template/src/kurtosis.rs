use frame_support::traits::IsType;

#[cfg(feature = "std")]
use futures::{lock::Mutex, lock::MutexGuard, Future, FutureExt, TryFutureExt};

#[cfg(feature = "std")]
use kurtosis_sdk::{
	enclave_api::{
		api_container_service_client::ApiContainerServiceClient,
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
use sp_std::{boxed::Box, sync::Arc};

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
	Ready(Arc<EngineServiceClient<tonic::transport::Channel>>),
}

#[cfg(feature = "std")]
pub struct KurtosisClient {
	engine: Arc<Mutex<KurtosisEngineState>>,
	state_changed: Arc<Notify>,
}

#[cfg(feature = "std")]
impl KurtosisClient {
	pub fn new() -> Arc<Self> {
		let future = async { EngineServiceClient::connect("https://[::1]:9710").await };

		Arc::new(Self {
			engine: Arc::new(Mutex::new(KurtosisEngineState::Pending(Box::pin(future)))),
			state_changed: Arc::new(Notify::new()),
		})
	}

	pub fn initialize(&self, spawner: impl SpawnNamed + 'static) {
		let engine_clone = self.engine.clone();
		let state_changed = self.state_changed.clone();

		log::info!("Kurtosis client is initializing.");
		spawner.spawn(
			"kurtosis-engine-init",
			None,
			Box::pin(async move {
				let mut engine_lock = engine_clone.lock().await;
				if let KurtosisEngineState::Pending(ref mut future) = *engine_lock {
					match future.as_mut().await {
						Ok(client) =>
							*engine_lock = {
								log::error!("Kurtosis client is ready");
								KurtosisEngineState::Ready(Arc::new(client))
							},
						Err(e) =>
							*engine_lock = {
								log::error!("Kurtosis client failed to initialize: {:?}", e);
								KurtosisEngineState::Failed(e)
							},
					}
				}

				state_changed.notify_one()
			}),
		);
	}

	pub async fn with_engine<F, T>(&self, f: F) -> Result<T, String>
	where
		F: FnOnce(&EngineServiceClient<tonic::transport::Channel>) -> T,
	{
		let engine_state = self.engine.lock().await;
		loop {
			match &*engine_state {
				KurtosisEngineState::Ready(client) => break Ok(f(&*client)),
				KurtosisEngineState::Pending(_) => self.state_changed.notified().await,
				KurtosisEngineState::Failed(reason) => break Err(reason.to_string()),
			}
		}
	}

	pub async fn with_engine_mut<F, T>(&mut self, f: F) -> Result<T, String>
	where
		F: FnOnce(&mut EngineServiceClient<tonic::transport::Channel>) -> T,
	{
		let mut engine_state = self.engine.lock().await;
		loop {
			match &mut *engine_state {
				KurtosisEngineState::Ready(client) =>
					if let Some(client_mut) = Arc::get_mut(client) {
						break Ok(f(client_mut))
					},
				KurtosisEngineState::Pending(_) => self.state_changed.notified().await,
				KurtosisEngineState::Failed(reason) => break Err(reason.to_string()),
			}
		}
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

#[runtime_interface]
pub trait Kurtosis {
	fn create_enclave(&mut self) {
		if let Some(kurtosis_ext) = self.extension::<KurtosisExt>() {
			let engine_lock = kurtosis_ext.0.engine.clone();
			let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

			rt.block_on(async {
				let result = kurtosis_ext
					.0
					.with_engine(|client| {
						log::info!("Kurtosis client is ready.");
					})
					.await;

				match result {
					Ok(_) => log::info!("Enclave created successfully"),
					Err(e) => log::error!("Failed to create enclave: {}", e),
				}
			});
		} else {
			log::error!("KurtosisExt not found in externalities");
		}
	}
}
