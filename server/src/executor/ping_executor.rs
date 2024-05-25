use super::Executor;
use derive_builder::Builder;
use log::debug;

#[derive(Builder)]
pub struct PingExecutor {}

impl Executor for PingExecutor {
    fn execute(&self) -> Box<[u8]> {
        debug!("Executing PING");

        "PONG".as_bytes().into()
    }
}
