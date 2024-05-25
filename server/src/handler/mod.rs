use crate::{executor::get_command_executor, protocol::parser::parse_command};

pub struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn handle(&self, command: &[u8]) {
        if let Some(e) = parse_command(command).map(get_command_executor) {
            e.execute();
        }
    }
}
