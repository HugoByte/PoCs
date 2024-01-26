use super::*;
use async_trait::async_trait;
use futures::lock::Mutex;
use sp_std::{any::Any, sync::Arc};

pub struct TestKurtosisClient;

impl TestKurtosisClient {
	pub fn new() -> Arc<Self> {
		Arc::new(Self)
	}
}

#[async_trait]
impl kurtosis::KurtosisClientTrait for TestKurtosisClient {
	fn as_any(&self) -> &dyn Any {
		self
	}
}