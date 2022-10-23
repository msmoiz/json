use std::iter;

use lazy_static::lazy_static;
use regex::Regex;

use super::types::{Error, Result, Token};

lazy_static! {
    static ref STRING_RE: Regex =
        Regex::new(r#"^"((\\(["\\/bfnrt]|u[0-9a-fA-F]{4}))|[^"\\[:cntrl:]]+)*""#)
            .expect("String regex was invalid");
    static ref NUMBER_RE: Regex =
        Regex::new("^-?([0-9]+)(\\.[0-9]+)?([eE][+-]?[0-9]+)?").expect("Number regex was invalid");
    static ref LEADING_ZERO_RE: Regex =
        Regex::new("^-?0+[1-9]").expect("Leading zero regex was invalid");
}

/// Converts an input text into a list of tokens.
/// Strings are parsed without any transformations
/// and numbers are parsed to double-precision floats.
/// In addition, the following punctuation symbols will be
/// parsed into individual tokens: `{}[],:`. The method
/// ignores whitespace. It will return an error under the
/// following conditions:
///
/// * A segment beginning with '"' does not match a string.
/// * A segment beginning with 't' does not match `true`.
/// * A segment beginning with 'f' does not match `false`.
/// * A segment beginning with 'n' does not match `null`.
/// * A segment beginning with '-' or a digit does not match a number.
///
/// This implementation matches the specification declared
/// at https://www.json.org.
pub fn tokenize(text: &str) -> Result<Vec<Token>> {
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
    tokenize(&text[1..])
}

fn match_punct(text: &str) -> Result<Vec<Token>> {
    Ok(iter::once(Token::Punct(text.chars().next().unwrap()))
        .chain(tokenize(&text[1..])?)
        .collect())
}

fn match_true(text: &str) -> Result<Vec<Token>> {
    match text.strip_prefix("true") {
        None => Err(Error),
        Some(substring) => Ok(iter::once(Token::True)
            .chain(tokenize(substring)?)
            .collect()),
    }
}

fn match_false(text: &str) -> Result<Vec<Token>> {
    match text.strip_prefix("false") {
        None => Err(Error),
        Some(substring) => Ok(iter::once(Token::False)
            .chain(tokenize(substring)?)
            .collect()),
    }
}

fn match_null(text: &str) -> Result<Vec<Token>> {
    match text.strip_prefix("null") {
        None => Err(Error),
        Some(substring) => Ok(iter::once(Token::Null)
            .chain(tokenize(substring)?)
            .collect()),
    }
}

fn match_number(text: &str) -> Result<Vec<Token>> {
    match NUMBER_RE.find(text) {
        None => Err(Error),
        Some(mat) => match LEADING_ZERO_RE.find(mat.as_str()) {
            Some(_) => Err(Error),
            None => Ok(iter::once(Token::Number(mat.as_str().parse().unwrap()))
                .chain(tokenize(&text[mat.end()..])?)
                .collect()),
        },
    }
}

fn match_string(text: &str) -> Result<Vec<Token>> {
    match STRING_RE.find(text) {
        None => Err(Error),
        Some(mat) => Ok(
            iter::once(Token::String(text[1..mat.end() - 1].to_string()))
                .chain(tokenize(&text[mat.end()..])?)
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::json::types::Token;

    use super::tokenize;

    #[test]
    fn recognizes_open_brace() {
        let text = "{";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Punct('{'));
    }

    #[test]
    fn recognizes_close_brace() {
        let text = "}";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Punct('}'));
    }

    #[test]
    fn recognizes_open_bracket() {
        let text = "[";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Punct('['));
    }

    #[test]
    fn recognizes_close_bracket() {
        let text = "]";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Punct(']'));
    }

    #[test]
    fn recognizes_comma() {
        let text = ",";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Punct(','));
    }

    #[test]
    fn recognizes_colon() {
        let text = ":";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Punct(':'));
    }

    #[test]
    fn recognizes_true() {
        let text = "true";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::True);
    }

    #[test]
    fn rejects_partial_true() {
        let text = "tru";
        let tokens = tokenize(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_false() {
        let text = "false";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::False);
    }

    #[test]
    fn rejects_partial_false() {
        let text = "fals";
        let tokens = tokenize(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_null() {
        let text = "null";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Null);
    }

    #[test]
    fn rejects_partial_null() {
        let text = "nul";
        let tokens = tokenize(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_empty_string() {
        let text = r#""""#;
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::String(String::from("")));
    }

    #[test]
    fn recognizes_ascii_string() {
        let text = r#""hello""#;
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::String(String::from("hello")));
    }

    #[test]
    fn recognizes_unicode_string() {
        let text = r#""😀""#;
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::String(String::from("😀")));
    }

    #[test]
    fn recognizes_string_with_escapes() {
        let text = r#""hello\\\/\b\f\n\r\tworld""#;
        let tokens = tokenize(text).unwrap();
        assert_eq!(
            tokens[0],
            Token::String(String::from("hello\\\\\\/\\b\\f\\n\\r\\tworld"))
        );
    }

    #[test]
    fn rejects_string_with_unmatched_quote() {
        let text = r#"""#;
        let tokens = tokenize(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_zero() {
        let text = "0";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Number(0.0));
    }

    #[test]
    fn recognizes_positive_number() {
        let text = "123";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Number(123.0));
    }

    #[test]
    fn recognizes_negative_number() {
        let text = "-123";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Number(-123.0));
    }

    #[test]
    fn recognizes_number_with_fraction() {
        let text = "123.456";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Number(123.456));
    }

    #[test]
    fn recognizes_number_with_exponent() {
        let text = "123e+3";
        let tokens = tokenize(text).unwrap();
        assert_eq!(tokens[0], Token::Number(123e+3));
    }

    #[test]
    fn rejects_number_with_leading_zero() {
        let text = "0123";
        let tokens = tokenize(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn rejects_number_with_incomplete_decimal() {
        let text = "123.";
        let tokens = tokenize(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn ignores_whitespace() {
        let text = " {} ";
        let tokens = tokenize(text).unwrap();
        assert!(matches!(tokens[0], Token::Punct('{')));
        assert!(matches!(tokens[1], Token::Punct('}')));
    }

    #[test]
    fn recognizes_multiple_tokens() {
        let text = "{}";
        let tokens = tokenize(text).unwrap();
        assert!(matches!(tokens[0], Token::Punct('{')));
        assert!(matches!(tokens[1], Token::Punct('}')));
    }

    #[test]
    fn rejects_unrecognized_character() {
        let text = "-";
        let tokens = tokenize(text);
        assert!(tokens.is_err());
    }

    #[test]
    fn recognizes_complex_text() {
        use super::Token::*;

        let text = r#"{
            "glossary": {
                "title": "example glossary",
                "GlossDiv": {
                    "title": "S",
                    "GlossList": {
                        "GlossEntry": {
                            "ID": "SGML",
                            "SortAs": "SGML",
                            "GlossTerm": "Standard Generalized Markup Language",
                            "Acronym": "SGML",
                            "Abbrev": "ISO 8879:1986",
                            "GlossDef": {
                                "para": "A meta-markup language, used to create markup languages such as DocBook.",
                                "GlossSeeAlso": ["GML", "XML"]
                            },
                            "GlossSee": "markup"
                        }
                    }
                }
            }
        }"#;
        let tokens = tokenize(text).unwrap();
        let expected = vec![
            Punct('{'),
            String("glossary".to_owned()),
            Punct(':'),
            Punct('{'),
            String("title".to_owned()),
            Punct(':'),
            String("example glossary".to_owned()),
            Punct(','),
            String("GlossDiv".to_owned()),
            Punct(':'),
            Punct('{'),
            String("title".to_owned()),
            Punct(':'),
            String("S".to_owned()),
            Punct(','),
            String("GlossList".to_owned()),
            Punct(':'),
            Punct('{'),
            String("GlossEntry".to_owned()),
            Punct(':'),
            Punct('{'),
            String("ID".to_owned()),
            Punct(':'),
            String("SGML".to_owned()),
            Punct(','),
            String("SortAs".to_owned()),
            Punct(':'),
            String("SGML".to_owned()),
            Punct(','),
            String("GlossTerm".to_owned()),
            Punct(':'),
            String("Standard Generalized Markup Language".to_owned()),
            Punct(','),
            String("Acronym".to_owned()),
            Punct(':'),
            String("SGML".to_owned()),
            Punct(','),
            String("Abbrev".to_owned()),
            Punct(':'),
            String("ISO 8879:1986".to_owned()),
            Punct(','),
            String("GlossDef".to_owned()),
            Punct(':'),
            Punct('{'),
            String("para".to_owned()),
            Punct(':'),
            String(
                "A meta-markup language, used to create markup languages such as DocBook."
                    .to_owned(),
            ),
            Punct(','),
            String("GlossSeeAlso".to_owned()),
            Punct(':'),
            Punct('['),
            String("GML".to_owned()),
            Punct(','),
            String("XML".to_owned()),
            Punct(']'),
            Punct('}'),
            Punct(','),
            String("GlossSee".to_owned()),
            Punct(':'),
            String("markup".to_owned()),
            Punct('}'),
            Punct('}'),
            Punct('}'),
            Punct('}'),
            Punct('}'),
        ];
        assert_eq!(tokens, expected);
    }
}
