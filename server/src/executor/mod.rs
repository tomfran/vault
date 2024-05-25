use crate::protocol::command::Command;

use self::{
    get_executor::GetExecutorBuilder, ping_executor::PingExecutorBuilder,
    set_executor::SetExecutorBuilder,
};

pub mod get_executor;
pub mod ping_executor;
pub mod set_executor;

pub trait Executor {
    fn execute(&self);
}

pub fn get_command_executor(cmd: Command) -> Box<dyn Executor> {
    match cmd {
        Command::Ping => Box::new(PingExecutorBuilder::default().build().unwrap()),
        Command::Set(key, value) => Box::new(
            SetExecutorBuilder::default()
                .key(key)
                .value(value)
                .build()
                .unwrap(),
        ),
        Command::Get(key) => Box::new(GetExecutorBuilder::default().key(key).build().unwrap()),
    }
}
