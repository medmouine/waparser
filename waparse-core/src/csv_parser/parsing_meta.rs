use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    parse_headers: bool,
    delimiter: char,
    linebreak: char,
    truncated: bool,
    aborted: bool,
    cursor: u16
}

impl Meta {
    pub fn build() -> Meta {
        Meta {
            parse_headers: true,
            delimiter: ',',
            linebreak: '\n',
            truncated: false,
            aborted: false,
            cursor: 0
        }
    }

    pub fn without_headers(mut self) -> Meta {
        self.parse_headers = false;
        self
    }

    pub fn with_delimiter(mut self, delimiter: char) -> Meta {
        self.delimiter = delimiter;
        self
    }

    pub fn with_linebreak(mut self, linebreak: char) -> Meta {
        self.linebreak = linebreak;
        self
    }

    pub fn truncated(mut self) -> Meta {
        self.truncated = true;
        self
    }

    pub fn aborted(&mut self, cursor: u16) {
        self.aborted = true;
        self.cursor = cursor
    }
}