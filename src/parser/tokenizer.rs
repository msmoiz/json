// #![allow(dead_code, unused_variables)]

// use lazy_static::lazy_static;
// use regex::Regex;

// use super::types::Token;

// pub fn tokenize_text(text: &str) -> Vec<Token> {
//     tokenize_text_r(text)
// }

// fn tokenize_text_r(text: &str) -> Vec<Token> {
//     lazy_static! {
//         static ref STRING_RE: Regex =
//             Regex::new(r#"^"((\\(["\\/bfnrt]|u[0-9a-fA-F]{4}))|[^"\\[:cntrl:]]+)*""#)
//                 .expect("Regex was invalid");
//         static ref NUMBER_RE: Regex = Regex::new("-?(0|[1-9][0-9]*)(\\.[0-9]+)?([eE][+-]?[0-9]+)?")
//             .expect("Regex was invalid");
//     }

//     match text.chars().next() {
//         None => vec![],
//         Some(c) => match c {
//             ' ' | '\n' | '\r' | '\t' => tokenize_text_r(&text[1..]),
//             '{' | '}' | '[' | ']' | ',' | ':' => vec![Token::Punct(c)]
//                 .into_iter()
//                 .chain(tokenize_text_r(&text[1..]).into_iter())
//                 .collect(),
//             't' => {
//                 if let Some(substring) = text.strip_prefix("true") {
//                     vec![Token::True]
//                         .into_iter()
//                         .chain(tokenize_text_r(substring).into_iter())
//                         .collect()
//                 } else {
//                     vec![]
//                 }
//             }
//             'f' => {
//                 if let Some(substring) = text.strip_prefix("false") {
//                     vec![Token::False]
//                         .into_iter()
//                         .chain(tokenize_text_r(substring).into_iter())
//                         .collect()
//                 } else {
//                     vec![]
//                 }
//             }
//             'n' => {
//                 if let Some(substring) = text.strip_prefix("null") {
//                     vec![Token::Null]
//                         .into_iter()
//                         .chain(tokenize_text_r(substring).into_iter())
//                         .collect()
//                 } else {
//                     vec![]
//                 }
//             }
//             '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
//                 match NUMBER_RE.shortest_match(text) {
//                     Some(pos) => vec![Token::Number]
//                         .into_iter()
//                         .chain(tokenize_text_r(&text[pos..]).into_iter())
//                         .collect(),
//                     None => vec![],
//                 }
//             }
//             '"' => match STRING_RE.shortest_match(text) {
//                 Some(pos) => vec![Token::String(text[1..pos - 1].to_owned())]
//                     .into_iter()
//                     .chain(tokenize_text_r(&text[pos..]).into_iter())
//                     .collect(),
//                 None => vec![],
//             },
//             _ => vec![],
//         },
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::parser::types::Token;

//     use super::tokenize_text;

//     #[test]
//     fn recognizes_open_brace() {
//         let text = "{";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct('{')));
//     }

//     #[test]
//     fn recognizes_close_brace() {
//         let text = "}";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct('}')));
//     }

//     #[test]
//     fn recognizes_open_bracket() {
//         let text = "[";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct('[')));
//     }

//     #[test]
//     fn recognizes_close_bracket() {
//         let text = "]";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct(']')));
//     }

//     #[test]
//     fn recognizes_comma() {
//         let text = ",";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct(',')));
//     }

//     #[test]
//     fn recognizes_colon() {
//         let text = ":";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct(':')));
//     }

//     #[test]
//     fn recognizes_true() {
//         let text = "true";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::True));
//     }

//     #[test]
//     fn recognizes_false() {
//         let text = "false";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::False));
//     }

//     #[test]
//     fn recognizes_null() {
//         let text = "null";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Null));
//     }

//     #[test]
//     fn recognizes_empty_string() {
//         let text = r#""""#;
//         let tokens = tokenize_text(text);
//         let token = Token::String(String::from("abcd"));
//         assert!(matches!(&tokens[0], token));
//     }

//     #[test]
//     fn recognizes_ascii_string() {
//         let text = r#""hello""#;
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::String(_)));
//     }

//     #[test]
//     fn recognizes_unicode_string() {
//         let text = r#""😀""#;
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::String(_)));
//     }

//     #[test]
//     fn recognizes_string_with_escapes() {
//         let text = r#""hello\\\/\b\f\n\r\tworld""#;
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::String(_)));
//     }

//     #[test]
//     fn recognizes_zero() {
//         let text = "0";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Number));
//     }

//     #[test]
//     fn recognizes_positive_number() {
//         let text = "123";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Number));
//     }

//     #[test]
//     fn recognizes_negative_number() {
//         let text = "-123";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Number));
//     }

//     #[test]
//     fn recognizes_number_with_fraction() {
//         let text = "123.456";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Number));
//     }

//     #[test]
//     fn recognizes_number_with_exponent() {
//         let text = "123e+456";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Number));
//     }

//     #[test]
//     fn recognizes_multiple_tokens() {
//         let text = "{}";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct('{')));
//         assert!(matches!(tokens[1], Token::Punct('}')));
//     }

//     #[test]
//     fn ignores_whitespace() {
//         let text = " {} ";
//         let tokens = tokenize_text(text);
//         assert!(matches!(tokens[0], Token::Punct('{')));
//         assert!(matches!(tokens[1], Token::Punct('}')));
//     }
// }
