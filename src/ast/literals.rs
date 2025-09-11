/// A literal.
#[derive(Debug, Clone)]
pub enum Lit {
    /// A string literal: `"..."`
    Str(String),
    /// An integer literal: `123`
    Int(u64),
    /// A boolean literal: `true` or `false`
    Bool(bool),
}

impl From<String> for Lit {
    fn from(s: String) -> Self {
        Lit::Str(s)
    }
}

impl From<&str> for Lit {
    fn from(s: &str) -> Self {
        Lit::Str(s.to_string())
    }
}

impl From<u64> for Lit {
    fn from(i: u64) -> Self {
        Lit::Int(i)
    }
}

impl From<i32> for Lit {
    fn from(i: i32) -> Self {
        Lit::Int(i as u64)
    }
}

impl From<bool> for Lit {
    fn from(b: bool) -> Self {
        Lit::Bool(b)
    }
}
