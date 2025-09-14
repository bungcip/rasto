//! Defines the AST nodes for patterns.
//!
//! Patterns are used in `let` bindings, function parameters, and `match` expressions to destructure
//! data. They are a powerful feature of Rust that allows for expressive and concise code.

use thin_vec::ThinVec;

/// Represents a pattern in a `let` binding, function parameter, or `match` arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pat {
    /// A wildcard pattern, `_`, which matches any value and ignores it.
    Wild,
    /// A rest pattern, `..`, which can be used in tuple and slice patterns to
    /// match multiple elements.
    Rest,
    /// An identifier pattern, e.g., `x` or `mut x`, which binds a variable to a value.
    Ident(PatIdent),
    /// A tuple pattern, e.g., `(a, b)`, which destructures a tuple.
    Tuple(ThinVec<Pat>),
}

/// An identifier pattern, which can be mutable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatIdent {
    /// The name of the identifier.
    pub ident: String,
    /// Whether the binding is mutable.
    pub is_mut: bool,
}
