use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    position: usize,
    details: String,
}

pub type ParserResult<T> = Result<T, ParserError>;

impl ParserError {
    pub fn new(position: usize, details: String) -> ParserError {
        ParserError { position, details }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String;
        if self.details.is_empty() {
            msg = format!("Error at position {}", self.position);
        } else {
            msg = self.details.clone();
        }

        write!(f, "{}^\n '{}'", " ".repeat(self.position), msg)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        "Cannot interpret the input"
    }
}
