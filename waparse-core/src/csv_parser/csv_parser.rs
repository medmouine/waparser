extern crate csv;

use wasm_bindgen::prelude::*;
use waparse_common::extenders::web::extended_jsvalue::ExtendedJsValue;
use web_sys::FileReader;
use wasm_bindgen::JsValue;
use crate::csv_parser::default_csv_parser::parse_data_as_bytes;

#[wasm_bindgen]
pub fn parse_csv(file_reader: FileReader) -> JsValue {
    match file_reader.result() {
        Ok(r) => {
            match r.to_bytes() {
                Some(b) => JsValue::from_serde(&parse_data_as_bytes(&b)).unwrap(),
                None => JsValue::from("{}")
            }
        }
        Err(_) => JsValue::UNDEFINED
    }
}
