use super::Executor;
use log::debug;

pub struct PingExecutor {}

impl Executor for PingExecutor {
    fn execute(&self) -> Box<[u8]> {
        debug!("Executing PING");

        "PONG".as_bytes().into()
    }
}

impl PingExecutor {
    pub fn new() -> PingExecutor {
        PingExecutor {}
    }
}
