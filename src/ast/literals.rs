//! Defines the AST nodes for literals.
//!
//! Literals are values that are written directly in the source code, such as strings,
//! numbers, and booleans.

/// A literal expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    /// A string literal, e.g., `"hello"`.
    Str(String),
    /// An integer literal, e.g., `42`.
    Int(u64),
    /// A boolean literal, e.g., `true` or `false`.
    Bool(bool),
}

impl From<String> for Lit {
    /// Converts a `String` into a `Lit::Str`.
    fn from(s: String) -> Self {
        Lit::Str(s)
    }
}

impl From<&str> for Lit {
    /// Converts a `&str` into a `Lit::Str`.
    fn from(s: &str) -> Self {
        Lit::Str(s.to_string())
    }
}

impl From<u64> for Lit {
    /// Converts a `u64` into a `Lit::Int`.
    fn from(i: u64) -> Self {
        Lit::Int(i)
    }
}

impl From<i32> for Lit {
    /// Converts an `i32` into a `Lit::Int`.
    fn from(i: i32) -> Self {
        Lit::Int(i as u64)
    }
}

impl From<bool> for Lit {
    /// Converts a `bool` into a `Lit::Bool`.
    fn from(b: bool) -> Self {
        Lit::Bool(b)
    }
}
