use std::env;

mod json;

fn main() {
    // @todo: Implement piping support

    let args: Vec<String> = env::args().collect();

    let text = args.get(1);
    if text.is_none() {
        println!("No input text provided.");
        return;
    }

    match json::parse(text.unwrap()) {
        Err(_) => println!("Input text does not contain valid JSON."),
        Ok(value) => println!("{}", value),
    }
}
