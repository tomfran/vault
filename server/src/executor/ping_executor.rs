use derive_builder::Builder;
use log::debug;

use super::Executor;

#[derive(Builder)]
pub struct PingExecutor {}

impl Executor for PingExecutor {
    fn execute(&self) {
        debug!("Ping executor");
    }
}
