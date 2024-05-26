use super::Executor;
use crate::storage::Storage;
use log::debug;
use std::sync::{Arc, Mutex};

pub struct GetExecutor {
    storage: Arc<Mutex<Storage>>,
    key: Box<[u8]>,
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

impl GetExecutor {
    pub fn new(storage: Arc<Mutex<Storage>>, key: Box<[u8]>) -> GetExecutor {
        GetExecutor { storage, key }
    }
}
