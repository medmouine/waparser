use serde::{Serialize, Deserialize};
use wasm_bindgen::__rt::std::collections::HashMap;
use crate::csv_parser::parsing_meta::Meta;

pub type JsObjectString2String = HashMap<String, String>;

pub type Headers = Vec<String>;
pub type Row = Vec<String>;

#[derive(Serialize, Deserialize)]
pub struct ParsingResult {
    data: Vec<JsObjectString2String>,
    errors: Vec<String>,
    meta: Meta,
}

impl ParsingResult {
    pub(crate) fn new(data: Vec<JsObjectString2String>, errors: Vec<String>, meta: Meta) -> ParsingResult {
        ParsingResult {
            data,
            errors,
            meta,
        }
    }
}
