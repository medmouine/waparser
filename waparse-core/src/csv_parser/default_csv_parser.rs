use crate::csv_parser::parsing_result::{ParsingResult, Headers, JsObjectString2String, Row};
use crate::csv_parser::parsing_meta::Meta;

use web_sys::console;
use wasm_bindgen::__rt::std::error::Error;
use csv::{Reader, Result as CsvReaderResult };
use waparse_common::extenders::parsers::extended_reader::ExtendedReader;

pub fn parse_data_as_bytes(byte_data: &[u8]) -> ParsingResult {
    let default_meta = Meta::build();
    let mut rdr = Reader::default_new(byte_data);

    match get_headers_from_first_row(&mut rdr) {
        Ok(headers) => {
            let (data, errors) = generate_result_data(&mut rdr, &headers);
            ParsingResult::new(data, errors, default_meta)
        }
        Err(e) => {
            console::log_1(&e.to_string().into());
            ParsingResult::new([].to_vec(), vec![e.to_string()], default_meta)
        }
    }
}

fn generate_result_data(rdr: &mut Reader<&[u8]>, headers: &Headers) -> (Vec<JsObjectString2String>, Vec<String>) {
    let mut errors = vec![];
    (rdr.deserialize()
         .enumerate()
         .map(|(i, record)| -> JsObjectString2String {
             handle_row_parsing(&i, &record, headers, &mut errors)
         }).collect(), errors)
}

fn handle_row_parsing(line_index: &usize, row: &CsvReaderResult<Row>, headers: &Headers, errors: &mut Vec<String>) -> JsObjectString2String {
    match row {
        Ok(row_record) => create_object(line_index, row_record, headers, errors),
        Err(e) => handle_parsing_error(errors, &e.to_string(), &line_index)
    }
}

fn handle_parsing_error(errors_array: &mut Vec<String>, error_message: &String, cursor: &usize) -> JsObjectString2String {
    errors_array.push(format!("Error on line {} : {}", cursor, error_message));
    JsObjectString2String::new()
}

fn create_object(line_index: &usize, row_record: &Row, headers: &Headers, errors: &mut Vec<String>) -> JsObjectString2String {
    if row_record.len() != headers.len() {
        handle_parsing_error(errors, &String::from("Wrong number of fields"), line_index);
        return JsObjectString2String::new();
    }
    headers.into_iter()
        .cloned()
        .zip(row_record.into_iter().cloned())
        .collect()
}

fn get_headers_from_first_row(rdr: &mut Reader<&[u8]>) -> Result<Headers, Box<dyn Error>> {
    match rdr.deserialize().next() {
        Some(r) => {
            let record: Headers = r?;
            Ok(record)
        }
        _ => Err(Box::from("Could not extract headers from first row. File may be empty."))
    }
}
