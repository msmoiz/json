use std::collections::HashMap;

/// An enumeration of tokens that may appear within JSON
/// text. The tokens contain information that is relevant
/// to each variant.
#[derive(PartialEq, Debug)]
pub enum Token {
    Punct(char),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error;

/// An enumeration of values that may appear within JSON
/// text. The enumeration can be traversed as a tree, with
/// object and array types containing nested values.
#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}
