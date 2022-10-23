mod json;

fn main() {
    if let Ok(json::Value::Boolean(bool)) = json::parse("false") {
        println!("{}", bool);
    }
}
