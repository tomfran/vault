use crate::{executor::get_command_executor, protocol::parser::parse_command, storage::Storage};
use std::sync::{Arc, Mutex};

pub struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn handle(&self, command: &[u8], storage: Arc<Mutex<Storage>>) -> Option<Box<[u8]>> {
        parse_command(command)
            .map(|c| get_command_executor(c, storage))
            .map(|e| e.execute())
    }
}
