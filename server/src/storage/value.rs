pub enum Value {
    Single(Box<[u8]>),
}

impl Value {
    pub fn serialize(&self) -> Box<[u8]> {
        match self {
            Value::Single(b) => b.clone(),
        }
    }
}
