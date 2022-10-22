mod parser;

fn main() {
    if parser::validate_json(vec![]) {
        println!("JSON is valid");
    } else {
        println!("JSON is invalid");
    }
}
