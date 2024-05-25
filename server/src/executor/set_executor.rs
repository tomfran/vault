use super::Executor;
use crate::storage::Storage;
use derive_builder::Builder;
use log::debug;
use std::sync::{Arc, Mutex};

#[derive(Builder)]
pub struct SetExecutor {
    key: Box<[u8]>,
    value: Box<[u8]>,
    storage: Arc<Mutex<Storage>>,
}

impl Executor for SetExecutor {
    fn execute(&self) -> Box<[u8]> {
        debug!("Executing SET({:?}, {:?})", self.key, self.value);

        self.storage.lock().unwrap().set(&self.key, &self.value);
        Box::new([1])
    }
}
