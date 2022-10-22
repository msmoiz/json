#![allow(dead_code, unused_variables)]

pub enum Token {
    Punct(char),
    String(String),
    Number,
    True,
    False,
    Null,
}
