mod value;

use self::value::Value;
use std::collections::HashMap;

#[derive(Default)]
pub struct Storage {
    map: HashMap<Box<[u8]>, Value>,
}

impl Storage {
    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        self.map.insert(key.into(), Value::Single(value.into()));
    }

    pub fn get(&self, key: &[u8]) -> Option<&Value> {
        self.map.get(key)
    }
}
