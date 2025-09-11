/// A literal.
#[derive(Debug, Clone)]
pub enum Lit {
    /// A string literal: `"..."`
    Str(String),
    /// An integer literal: `123`
    Int(u64),
}
