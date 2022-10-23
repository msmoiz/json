use std::io::{stdin, Read};

mod json;

fn main() {
    let mut text = String::new();
    if stdin().read_to_string(&mut text).is_err() {
        println!("Input text does not contain valid UTF-8.");
    }

    match json::parse(&text) {
        Err(_) => println!("Input text does not contain valid JSON."),
        Ok(value) => println!("{}", value),
    }
}
