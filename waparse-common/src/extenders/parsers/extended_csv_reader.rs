use csv::{Reader, ReaderBuilder};
use crate::extenders::parsers::extended_reader::ExtendedReader;

const IGNORE_HEADERS_FALSE: bool = false;

impl<'a> ExtendedReader<&'a [u8]> for Reader<&'a [u8]> {
    fn default_new(data: &'a [u8]) -> Reader<&'a [u8]> {
        ReaderBuilder::new()
            .has_headers(IGNORE_HEADERS_FALSE)
            .from_reader(data)
    }
}
