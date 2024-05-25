use derive_builder::Builder;
use log::debug;

use super::Executor;

#[derive(Builder)]
pub struct GetExecutor {
    key: String,
}

impl Executor for GetExecutor {
    fn execute(&self) {
        debug!("Get executor {}", self.key);
    }
}
