mod parser;

fn main() {
    if parser::validate_json_text(
        r#"{
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
    }"#,
    ) {
        println!("JSON is valid");
    } else {
        println!("JSON is invalid");
    }

    if parser::validate_json_text("{") {
        println!("JSON is valid");
    } else {
        println!("JSON is invalid");
    }

    if parser::validate_json_text("[") {
        println!("JSON is valid");
    } else {
        println!("JSON is invalid");
    }
}
