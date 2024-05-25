pub mod get_executor;
pub mod ping_executor;
pub mod set_executor;

use self::{
    get_executor::GetExecutorBuilder, ping_executor::PingExecutorBuilder,
    set_executor::SetExecutorBuilder,
};
use crate::{protocol::command::Command, storage::Storage};
use std::sync::{Arc, Mutex};

pub trait Executor {
    fn execute(&self) -> Box<[u8]>;
}

pub fn get_command_executor(cmd: Command, storage: Arc<Mutex<Storage>>) -> Box<dyn Executor> {
    match cmd {
        Command::Ping => Box::new(PingExecutorBuilder::default().build().unwrap()),
        Command::Set(key, value) => Box::new(
            SetExecutorBuilder::default()
                .storage(storage)
                .key(key)
                .value(value)
                .build()
                .unwrap(),
        ),
        Command::Get(key) => Box::new(
            GetExecutorBuilder::default()
                .storage(storage)
                .key(key)
                .build()
                .unwrap(),
        ),
    }
}
