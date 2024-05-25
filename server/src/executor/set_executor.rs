use derive_builder::Builder;
use log::debug;

use super::Executor;

#[derive(Builder)]
pub struct SetExecutor {
    key: String,
    value: String,
}

impl Executor for SetExecutor {
    fn execute(&self) {
        debug!("Set executor {} {}", self.key, self.value);
    }
}
