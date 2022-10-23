use std::{collections::HashMap, fmt::Display};

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

impl Value {
    // @todo: Review implementation
    // @todo: Consider adding color
    // @todo: Decide whether to add
    //        additional methods to improve ease of use
    // @todo: Figure out what the right way to handle
    //        the ordering is
    fn display(value: &Value, depth: usize) -> String {
        match value {
            Value::String(string) => format!(r#""{}""#, string),
            Value::Number(number) => number.to_string(),
            Value::Boolean(bool) => bool.to_string(),
            Value::Null => "null".to_string(),
            Value::Object(object) => format!(
                "{{\n{}\n{}}}",
                object
                    .iter()
                    .map(|member| format!(
                        r#"{}"{}" : {}"#,
                        "    ".repeat(depth + 1),
                        member.0,
                        Value::display(member.1, depth + 1)
                    ))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                "    ".repeat(depth)
            ),
            Value::Array(array) => format!(
                "[ {} ]",
                array
                    .iter()
                    .map(|element| Value::display(element, depth))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Value::display(self, 0))
    }
}
