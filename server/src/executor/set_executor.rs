use super::Executor;
use crate::storage::Storage;
use log::debug;
use std::sync::{Arc, Mutex};

pub struct SetExecutor {
    storage: Arc<Mutex<Storage>>,
    key: Box<[u8]>,
    value: Box<[u8]>,
}

impl Executor for SetExecutor {
    fn execute(&self) -> Box<[u8]> {
        debug!("Executing SET({:?}, {:?})", self.key, self.value);

        self.storage.lock().unwrap().set(&self.key, &self.value);
        Box::new([1])
    }
}

impl SetExecutor {
    pub fn new(storage: Arc<Mutex<Storage>>, key: Box<[u8]>, value: Box<[u8]>) -> SetExecutor {
        SetExecutor {
            storage,
            key,
            value,
        }
    }
}
