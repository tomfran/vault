pub mod get_executor;
pub mod ping_executor;
pub mod set_executor;

use self::{get_executor::GetExecutor, ping_executor::PingExecutor, set_executor::SetExecutor};
use crate::{protocol::command::Command, storage::Storage};
use std::sync::{Arc, Mutex};

pub trait Executor {
    fn execute(&self) -> Box<[u8]>;
}

#[derive(Clone)]
pub struct ExecutorFactory {
    storage: Arc<Mutex<Storage>>,
}

impl ExecutorFactory {
    pub fn new(storage: Arc<Mutex<Storage>>) -> ExecutorFactory {
        ExecutorFactory { storage }
    }

    pub fn build_executor(&self, cmd: Command) -> Box<dyn Executor> {
        match cmd {
            Command::Ping => Box::new(PingExecutor::new()),
            Command::Set(key, value) => {
                Box::new(SetExecutor::new(self.storage.clone(), key, value))
            }
            Command::Get(key) => Box::new(GetExecutor::new(self.storage.clone(), key)),
        }
    }
}
