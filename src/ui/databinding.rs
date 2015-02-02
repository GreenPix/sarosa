
use std::collections::HashMap;

pub trait Store {

    fn register(&self, registry: &mut DataBinder);
}

pub enum StoreValue {
    IsString(String),
    IsInteger(i32),
}

pub enum RegisterError {
    InvalidKeyFormat,
    KeyAlreadyRegistered
}

pub struct DataBinder {
    globalStore: HashMap<String, StoreValue>,
}

impl DataBinder {

    pub fn register_value(&mut self, key: String, value: StoreValue) -> Result<(), RegisterError> {
        // TODO:
    }
}
