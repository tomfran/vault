use crate::storage::Storage;
use derive_builder::Builder;
use log::debug;
use std::sync::{Arc, Mutex};

use super::Executor;

#[derive(Builder)]
pub struct GetExecutor {
    key: Box<[u8]>,
    storage: Arc<Mutex<Storage>>,
}

impl Executor for GetExecutor {
    fn execute(&self) -> Box<[u8]> {
        debug!("Executing GET({:?})", self.key);

        self.storage
            .lock()
            .unwrap()
            .get(&self.key)
            .map_or(Box::new([0]), |v| v.serialize())
    }
}
