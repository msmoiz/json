#![allow(dead_code, unused_variables)]

use std::iter;

use lazy_static::lazy_static;
use regex::Regex;

use super::types::Token;

lazy_static! {
    static ref STRING_RE: Regex =
        Regex::new(r#"^"((\\(["\\/bfnrt]|u[0-9a-fA-F]{4}))|[^"\\[:cntrl:]]+)*""#)
            .expect("String regex was invalid");
    static ref NUMBER_RE: Regex = Regex::new("^-?(0|[1-9][0-9]*)(\\.[0-9]+)?([eE][+-]?[0-9]+)?")
        .expect("Number regex was invalid");
    static ref LEADING_ZERO_RE: Regex =
        Regex::new("^-?0+[1-9]").expect("Leading zero regex was invalid");
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error;

pub fn tokenize_text(text: &str) -> Result<Vec<Token>> {
    match text.chars().next() {
        None => Ok(vec![]),
        Some(char) => match char {
            '"' => Ok(match_string(text)?),
            't' => Ok(match_true(text)?),
            'f' => Ok(match_false(text)?),
            'n' => Ok(match_null(text)?),
            '-' | '0'..='9' => Ok(match_number(text)?),
            ' ' | '\n' | '\r' | '\t' => Ok(match_whitespace(text)?),
            '{' | '}' | '[' | ']' | ',' | ':' => Ok(match_punct(text)?),
            _ => Err(Error),
        },
    }
}

fn match_whitespace(text: &str) -> Result<Vec<Token>> {
    tokenize_text(&text[1..])
}

fn match_punct(text: &str) -> Result<Vec<Token>> {
    Ok(iter::once(Token::Punct(text.chars().next().unwrap()))
        .chain(tokenize_text(&text[1..])?)
        .collect())
}

fn match_true(text: &str) -> Result<Vec<Token>> {
    match text.strip_prefix("true") {
        None => Err(Error),
        Some(substring) => Ok(iter::once(Token::True)
            .chain(tokenize_text(substring)?)
            .collect()),
    }
}

fn match_false(text: &str) -> Result<Vec<Token>> {
    match text.strip_prefix("false") {
        None => Err(Error),
        Some(substring) => Ok(iter::once(Token::False)
            .chain(tokenize_text(substring)?)
            .collect()),
    }
}

fn match_null(text: &str) -> Result<Vec<Token>> {
    match text.strip_prefix("null") {
        None => Err(Error),
        Some(substring) => Ok(iter::once(Token::Null)
            .chain(tokenize_text(substring)?)
            .collect()),
    }
}

fn match_number(text: &str) -> Result<Vec<Token>> {
    match NUMBER_RE.find(text) {
        None => Err(Error),
        Some(mat) => match LEADING_ZERO_RE.find(mat.as_str()) {
            Some(_) => Err(Error),
            None => Ok(iter::once(Token::Number(mat.as_str().parse().unwrap()))
                .chain(tokenize_text(&text[mat.end()..])?)
                .collect()),
        },
    }
}

fn match_string(text: &str) -> Result<Vec<Token>> {
    match STRING_RE.find(text) {
        None => Err(Error),
        Some(mat) => Ok(
            iter::once(Token::String(text[1..mat.end() - 1].to_string()))
                .chain(tokenize_text(&text[mat.end()..])?)
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::types::Token;

    use super::tokenize_text;

    #[test]
    fn recognizes_open_brace() {
        let text = "{";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Punct('{'));
    }

    #[test]
    fn recognizes_close_brace() {
        let text = "}";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Punct('}'));
    }

    #[test]
    fn recognizes_open_bracket() {
        let text = "[";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Punct('['));
    }

    #[test]
    fn recognizes_close_bracket() {
        let text = "]";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Punct(']'));
    }

    #[test]
    fn recognizes_comma() {
        let text = ",";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Punct(','));
    }

    #[test]
    fn recognizes_colon() {
        let text = ":";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Punct(':'));
    }

    #[test]
    fn recognizes_true() {
        let text = "true";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::True);
    }

    #[test]
    fn rejects_partial_true() {
        let text = "tru";
        let tokens = tokenize_text(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_false() {
        let text = "false";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::False);
    }

    #[test]
    fn rejects_partial_false() {
        let text = "fals";
        let tokens = tokenize_text(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_null() {
        let text = "null";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Null);
    }

    #[test]
    fn rejects_partial_null() {
        let text = "nul";
        let tokens = tokenize_text(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_empty_string() {
        let text = r#""""#;
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::String(String::from("")));
    }

    #[test]
    fn recognizes_ascii_string() {
        let text = r#""hello""#;
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::String(String::from("hello")));
    }

    #[test]
    fn recognizes_unicode_string() {
        let text = r#""😀""#;
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::String(String::from("😀")));
    }

    #[test]
    fn recognizes_string_with_escapes() {
        let text = r#""hello\\\/\b\f\n\r\tworld""#;
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(
            tokens[0],
            Token::String(String::from("hello\\\\\\/\\b\\f\\n\\r\\tworld"))
        );
    }

    #[test]
    fn rejects_string_with_unmatched_quote() {
        let text = r#"""#;
        let tokens = tokenize_text(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_zero() {
        let text = "0";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Number(0.0));
    }

    #[test]
    fn recognizes_positive_number() {
        let text = "123";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Number(123.0));
    }

    #[test]
    fn recognizes_negative_number() {
        let text = "-123";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Number(-123.0));
    }

    #[test]
    fn recognizes_number_with_fraction() {
        let text = "123.456";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Number(123.456));
    }

    #[test]
    fn recognizes_number_with_exponent() {
        let text = "123e+3";
        let tokens = tokenize_text(text).unwrap();
        assert_eq!(tokens[0], Token::Number(123e+3));
    }

    #[test]
    fn rejects_number_with_leading_zero() {
        // @todo: fix this
        let text = "0123";
        let tokens = tokenize_text(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn rejects_number_with_incomplete_decimal() {
        let text = "123.";
        let tokens = tokenize_text(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn ignores_whitespace() {
        let text = " {} ";
        let tokens = tokenize_text(text).unwrap();
        assert!(matches!(tokens[0], Token::Punct('{')));
        assert!(matches!(tokens[1], Token::Punct('}')));
    }

    #[test]
    fn recognizes_multiple_tokens() {
        let text = "{}";
        let tokens = tokenize_text(text).unwrap();
        assert!(matches!(tokens[0], Token::Punct('{')));
        assert!(matches!(tokens[1], Token::Punct('}')));
    }
}
