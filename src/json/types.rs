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
    // @todo: Consider adding color
    // @todo: Decide whether to add
    //        additional methods to improve ease of use
    // @todo: Figure out what the right way to handle
    //        the ordering is
    fn display(value: &Value, depth: usize) -> String {
        match value {
            Value::String(string) => Value::display_string(string),
            Value::Number(number) => Value::display_number(number),
            Value::Boolean(bool) => Value::display_bool(bool),
            Value::Object(object) => Value::display_object(object, depth),
            Value::Array(array) => Value::display_array(array, depth),
            Value::Null => Value::display_null(),
        }
    }

    fn display_string(string: &str) -> String {
        format!(r#""{}""#, string)
    }

    fn display_number(number: &f64) -> String {
        number.to_string()
    }

    fn display_bool(bool: &bool) -> String {
        bool.to_string()
    }

    fn display_null() -> String {
        "null".to_string()
    }

    fn display_object(object: &HashMap<String, Value>, depth: usize) -> String {
        format!(
            "{{{newline}{members}{newline}{indent}}}",
            newline = if !object.is_empty() { "\n" } else { "" },
            indent = " ".repeat(4).repeat(depth),
            members = object
                .iter()
                .map(|member| format!(
                    r#"{indent}"{key}" : {value}"#,
                    indent = " ".repeat(4).repeat(depth + 1),
                    key = member.0,
                    value = Value::display(member.1, depth + 1)
                ))
                .collect::<Vec<_>>()
                .join(",\n"),
        )
    }

    fn display_array(array: &[Value], depth: usize) -> String {
        format!(
            "[{newline}{values}{newline}{indent}]",
            newline = if !array.is_empty() { "\n" } else { "" },
            indent = " ".repeat(4).repeat(depth),
            values = array
                .iter()
                .map(|value| format!(
                    "{indent}{value}",
                    indent = " ".repeat(4).repeat(depth + 1),
                    value = Value::display(value, depth + 1)
                ))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Value::display(self, 0))
    }
}
