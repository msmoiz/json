mod parser;
mod tokenizer;
mod types;

use parser::parse as parse_internal;
use tokenizer::tokenize;
use types::{Error, Result};

pub use self::types::Value;

/// Parse a JSON string.
pub fn parse(text: &str) -> Result<Value> {
    match tokenize(text) {
        Err(_) => Err(Error),
        Ok(tokens) => parse_internal(tokens),
    }
}
