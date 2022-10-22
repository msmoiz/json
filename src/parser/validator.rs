#![allow(dead_code, unused_variables)]

use super::types::Token;

pub fn validate_json(tokens: Vec<Token>) -> bool {
    let json = json(&tokens);
    json.0 && json.1 == tokens.len()
}

fn json(tokens: &[Token]) -> (bool, usize) {
    element(tokens)
}

fn value(tokens: &[Token]) -> (bool, usize) {
    match tokens.get(0) {
        None => (false, 0),
        Some(token) => match token {
            Token::String(_) => (true, 1),
            Token::Number => (true, 1),
            Token::True => (true, 1),
            Token::False => (true, 1),
            Token::Null => (true, 1),
            Token::Punct(punct) => match punct {
                '[' => array(tokens),
                '{' => object(tokens),
                _ => (false, 0),
            },
        },
    }
}

fn object(tokens: &[Token]) -> (bool, usize) {
    let begins_with_open_brace = tokens
        .get(0)
        .map_or(false, |token| matches!(token, Token::Punct('{')));

    let followed_by_close_brace = tokens
        .get(1)
        .map_or(false, |token| matches!(token, Token::Punct('}')));

    if begins_with_open_brace && followed_by_close_brace {
        return (true, 2);
    }

    if begins_with_open_brace {
        let members = members(&tokens[1..]);
        let followed_by_close_brace = tokens
            .get(members.1 + 1)
            .map_or(false, |token| matches!(token, Token::Punct('}')));
        return (members.0 && followed_by_close_brace, members.1 + 2);
    }

    (false, 0)
}

fn members(tokens: &[Token]) -> (bool, usize) {
    let member = member(tokens);
    if !member.0 {
        return (false, 0);
    }

    let followed_by_comma = tokens
        .get(member.1)
        .map_or(false, |token| matches!(token, Token::Punct(',')));

    if !followed_by_comma {
        return member;
    }

    let members = members(&tokens[member.1 + 1..]);
    if !members.0 {
        return member;
    }

    (true, member.1 + members.1 + 1)
}

fn member(tokens: &[Token]) -> (bool, usize) {
    let begins_with_string = tokens
        .get(0)
        .map_or(false, |token| matches!(token, Token::String(_)));

    let followed_by_colon = tokens
        .get(1)
        .map_or(false, |token| matches!(token, Token::Punct(':')));

    let element = element(&tokens[2..]);
    if begins_with_string && followed_by_colon && element.0 {
        return (true, element.1 + 2);
    }

    (false, 0)
}

fn array(tokens: &[Token]) -> (bool, usize) {
    let begins_with_open_brace = tokens
        .get(0)
        .map_or(false, |token| matches!(token, Token::Punct('[')));

    let followed_by_close_brace = tokens
        .get(1)
        .map_or(false, |token| matches!(token, Token::Punct(']')));

    if begins_with_open_brace && followed_by_close_brace {
        return (true, 2);
    }

    if begins_with_open_brace {
        let elements = elements(&tokens[1..]);
        let followed_by_close_brace = tokens
            .get(elements.1 + 1)
            .map_or(false, |token| matches!(token, Token::Punct(']')));
        return (elements.0 && followed_by_close_brace, elements.1 + 2);
    }

    (false, 0)
}

fn elements(tokens: &[Token]) -> (bool, usize) {
    let element = element(tokens);
    if !element.0 {
        return (false, 0);
    }

    let followed_by_comma = tokens
        .get(element.1)
        .map_or(false, |token| matches!(token, Token::Punct(',')));

    if !followed_by_comma {
        return element;
    }

    let elements = elements(&tokens[element.1 + 1..]);
    if !elements.0 {
        return element;
    }

    (true, element.1 + elements.1 + 1)
}

fn element(tokens: &[Token]) -> (bool, usize) {
    value(tokens)
}

#[cfg(test)]
mod tests {
    use super::{validate_json, Token::*};

    #[test]
    fn rejects_empty_input() {
        let tokens = vec![];
        assert!(!validate_json(tokens));
    }

    #[test]
    fn accepts_string() {
        let tokens = vec![String("".to_owned())];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_number() {
        let tokens = vec![Number];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_true() {
        let tokens = vec![True];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_false() {
        let tokens = vec![False];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_null() {
        let tokens = vec![Null];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_empty_array() {
        let tokens = vec![Punct('['), Punct(']')];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_array_with_single_element() {
        let tokens = vec![Punct('['), String("".to_owned()), Punct(']')];
        assert!(validate_json(tokens));
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
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_array_with_nested_array() {
        let tokens = vec![Punct('['), Punct('['), Punct(']'), Punct(']')];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_array_with_elements_of_different_types() {
        let tokens = vec![
            Punct('['),
            String("".to_owned()),
            Punct(','),
            Number,
            Punct(','),
            Null,
            Punct(']'),
        ];
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_empty_object() {
        let tokens = vec![Punct('{'), Punct('}')];
        assert!(validate_json(tokens));
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
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_object_with_multiple_elements() {
        let tokens = vec![
            Punct('{'),
            String("".to_owned()),
            Punct(':'),
            String("".to_owned()),
            Punct(','),
            String("".to_owned()),
            Punct(':'),
            String("".to_owned()),
            Punct('}'),
        ];
        assert!(validate_json(tokens));
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
        assert!(validate_json(tokens));
    }

    #[test]
    fn accepts_object_with_elements_of_different_types() {
        let tokens = vec![
            Punct('{'),
            String("".to_owned()),
            Punct(':'),
            String("".to_owned()),
            Punct(','),
            String("".to_owned()),
            Punct(':'),
            Number,
            Punct(','),
            String("".to_owned()),
            Punct(':'),
            Null,
            Punct('}'),
        ];
        assert!(validate_json(tokens));
    }
}
