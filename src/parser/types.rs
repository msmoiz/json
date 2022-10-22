/// An enumeration of tokens that may appear within JSON
/// text. The tokens contain information that is relevant
/// to each variant.
#[derive(PartialEq, Debug)]
pub enum Token {
    Punct(char),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}
