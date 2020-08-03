use wasm_bindgen::prelude::*;

pub trait ExtendedJsValue {
    fn to_bytes(&self) -> Option<Vec<u8>>;
}

impl ExtendedJsValue for JsValue {
    fn to_bytes(&self) -> Option<Vec<u8>> {
        match self.as_string() {
            Some(v) => Option::from(Vec::from(v)),
            None => None
        }
    }
}
