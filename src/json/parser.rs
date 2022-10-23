use std::{collections::HashMap, iter};

use super::types::{Error, Result, Token, Value};

struct Node {
    value: Value,
    len: usize,
}

/// Parses an input token list into a JSON value.
/// It will return an error under the following
/// conditions:
///
/// * The token list is empty.
/// * The token list contains a punctuation mark
///   in an unexpected position.
/// * An array does not have matching brackets.
/// * An array is missing necessary commas.
/// * An object does not have matching braces.
/// * An object is missing necessary commas.
/// * An object is missing necessary keys, colons, values.
///
/// This implementation matches the specification declared
/// at https://www.json.org.
pub fn parse(tokens: Vec<Token>) -> Result<Value> {
    let json = json(&tokens)?;
    (json.len == tokens.len())
        .then_some(json.value)
        .ok_or(Error)
}

fn json(tokens: &[Token]) -> Result<Node> {
    element(tokens)
}

fn value(tokens: &[Token]) -> Result<Node> {
    match tokens.get(0) {
        None => Err(Error),
        Some(token) => match token {
            Token::String(string) => Ok(Node {
                value: Value::String(string.clone()),
                len: 1,
            }),
            Token::Number(number) => Ok(Node {
                value: Value::Number(*number),
                len: 1,
            }),
            Token::True => Ok(Node {
                value: Value::Boolean(true),
                len: 1,
            }),
            Token::False => Ok(Node {
                value: Value::Boolean(false),
                len: 1,
            }),
            Token::Null => Ok(Node {
                value: Value::Null,
                len: 1,
            }),
            Token::Punct(punct) => match punct {
                '[' => array(tokens),
                '{' => object(tokens),
                _ => Err(Error),
            },
        },
    }
}

fn object(tokens: &[Token]) -> Result<Node> {
    let begins_with_open_brace = matches!(tokens.get(0), Some(Token::Punct(char)) if char == &'{');
    let followed_by_close_brace = matches!(tokens.get(1), Some(Token::Punct(char)) if char == &'}');

    if !begins_with_open_brace {
        return Err(Error);
    }

    if begins_with_open_brace && followed_by_close_brace {
        return Ok(Node {
            value: Value::Object(HashMap::new()),
            len: 2,
        });
    }

    let members = members(&tokens[1..])?;
    let mem_len = members
        .iter()
        .fold(members.len() - 1, |acc, mem| acc + mem.1.len + 2);

    let followed_by_close_brace =
        matches!(tokens.get(mem_len + 1), Some(Token::Punct(char)) if char == &'}');

    followed_by_close_brace
        .then_some(Node {
            value: Value::Object(
                members
                    .into_iter()
                    .map(|mem| (mem.0, mem.1.value))
                    .collect(),
            ),
            len: mem_len + 2,
        })
        .ok_or(Error)
}

fn members(tokens: &[Token]) -> Result<HashMap<String, Node>> {
    let member = member(tokens)?;

    let followed_by_comma =
        matches!(tokens.get(member.1.len + 2), Some(Token::Punct(char)) if char == &',');

    if !followed_by_comma {
        return Ok(HashMap::from([member]));
    }

    match members(&tokens[member.1.len + 2 + 1..]) {
        Err(_) => Ok(HashMap::from([member])),
        Ok(members) => Ok(HashMap::from([member]).into_iter().chain(members).collect()),
    }
}

fn member(tokens: &[Token]) -> Result<(String, Node)> {
    if let Some(Token::String(string)) = tokens.get(0) {
        let followed_by_colon = matches!(tokens.get(1), Some(Token::Punct(char)) if char == &':');
        if !followed_by_colon {
            return Err(Error);
        }

        let element = element(&tokens[2..])?;
        return Ok((string.clone(), element));
    }

    Err(Error)
}

fn array(tokens: &[Token]) -> Result<Node> {
    let begins_with_open_bracket =
        matches!(tokens.get(0), Some(Token::Punct(char)) if char == &'[');

    let followed_by_close_bracket =
        matches!(tokens.get(1), Some(Token::Punct(char)) if char == &']');

    if !begins_with_open_bracket {
        return Err(Error);
    }

    if begins_with_open_bracket && followed_by_close_bracket {
        return Ok(Node {
            value: Value::Array(vec![]),
            len: 2,
        });
    }

    let elements = elements(&tokens[1..])?;
    let elem_len = elements
        .iter()
        .fold(elements.len() - 1, |acc, elem| acc + elem.len);

    let followed_by_close_bracket =
        matches!(tokens.get(elem_len + 1), Some(Token::Punct(char)) if char == &']');

    followed_by_close_bracket
        .then_some(Node {
            value: Value::Array(elements.into_iter().map(|elem| elem.value).collect()),
            len: elem_len + 2,
        })
        .ok_or(Error)
}

fn elements(tokens: &[Token]) -> Result<Vec<Node>> {
    let element = element(tokens)?;

    let followed_by_comma =
        matches!(tokens.get(element.len), Some(Token::Punct(char)) if char == &',');

    if !followed_by_comma {
        return Ok(vec![element]);
    }

    match elements(&tokens[element.len + 1..]) {
        Err(_) => Ok(vec![element]),
        Ok(elements) => Ok(iter::once(element).chain(elements).collect()),
    }
}

fn element(tokens: &[Token]) -> Result<Node> {
    value(tokens)
}

#[cfg(test)]
mod tests {
    use super::{parse, Token::*};

    #[test]
    fn rejects_empty_input() {
        let tokens = vec![];
        assert!(parse(tokens).is_err());
    }

    #[test]
    fn rejects_overrun_input() {
        let tokens = vec![Punct('{'), Punct('}'), String("".to_owned())];
        assert!(parse(tokens).is_err());
    }

    #[test]
    fn accepts_string() {
        let tokens = vec![String("".to_owned())];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_number() {
        let tokens = vec![Number(0.0)];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_true() {
        let tokens = vec![True];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_false() {
        let tokens = vec![False];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_null() {
        let tokens = vec![Null];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_empty_array() {
        let tokens = vec![Punct('['), Punct(']')];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_array_with_single_element() {
        let tokens = vec![Punct('['), String("".to_owned()), Punct(']')];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_array_with_multiple_elements() {
        let tokens = vec![
            Punct('['),
            String("".to_owned()),
            Punct(','),
            String("".to_owned()),
            Punct(']'),
        ];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_array_with_nested_array() {
        let tokens = vec![Punct('['), Punct('['), Punct(']'), Punct(']')];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_array_with_elements_of_different_types() {
        let tokens = vec![
            Punct('['),
            String("".to_owned()),
            Punct(','),
            Number(0.0),
            Punct(','),
            Null,
            Punct(']'),
        ];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn rejects_incomplete_array() {
        let tokens = vec![Punct('[')];
        assert!(parse(tokens).is_err());
    }

    #[test]
    fn accepts_empty_object() {
        let tokens = vec![Punct('{'), Punct('}')];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_object_with_single_element() {
        let tokens = vec![
            Punct('{'),
            String("".to_owned()),
            Punct(':'),
            String("".to_owned()),
            Punct('}'),
        ];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_object_with_multiple_elements() {
        let tokens = vec![
            Punct('{'),
            String("hello".to_owned()),
            Punct(':'),
            String("".to_owned()),
            Punct(','),
            String("goodbye".to_owned()),
            Punct(':'),
            String("".to_owned()),
            Punct('}'),
        ];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_object_with_nested_object() {
        let tokens = vec![
            Punct('{'),
            String("".to_owned()),
            Punct(':'),
            Punct('{'),
            Punct('}'),
            Punct('}'),
        ];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn accepts_object_with_elements_of_different_types() {
        let tokens = vec![
            Punct('{'),
            String("hello".to_owned()),
            Punct(':'),
            String("".to_owned()),
            Punct(','),
            String("goodbye".to_owned()),
            Punct(':'),
            Number(0.0),
            Punct(','),
            String("morning".to_owned()),
            Punct(':'),
            Null,
            Punct('}'),
        ];
        assert!(parse(tokens).is_ok());
    }

    #[test]
    fn rejects_incomplete_object() {
        let tokens = vec![Punct('{')];
        assert!(parse(tokens).is_err());
    }

    #[test]
    fn rejects_object_with_missing_key() {
        let tokens = vec![Punct('{'), Punct(':'), String("".to_owned()), Punct('}')];
        assert!(parse(tokens).is_err());
    }

    #[test]
    fn rejects_object_with_missing_colon() {
        let tokens = vec![
            Punct('{'),
            String("".to_owned()),
            String("".to_owned()),
            Punct('}'),
        ];
        assert!(parse(tokens).is_err());
    }

    #[test]
    fn rejects_object_with_missing_value() {
        let tokens = vec![Punct('{'), String("".to_owned()), Punct(':'), Punct('}')];
        assert!(parse(tokens).is_err());
    }
}
