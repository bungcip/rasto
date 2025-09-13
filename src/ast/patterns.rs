//! Defines the AST nodes for patterns.
//!
//! Patterns are used in `let` bindings, function parameters, and `match` expressions to destructure
//! data. They are a powerful feature of Rust that allows for expressive and concise code.

/// Represents a pattern in a `let` binding, function parameter, or `match` arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pat {
    /// A wildcard pattern, `_`.
    Wild,
    /// A rest pattern, i.e., `..`.
    Rest,
    /// An identifier pattern, e.g., `x` or `mut x`.
    Ident(PatIdent),
    /// A tuple pattern, e.g., `(a, b)`.
    Tuple(Vec<Pat>),
}

/// An identifier pattern, which can be mutable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatIdent {
    /// The name of the identifier.
    pub ident: String,
    /// Whether the binding is mutable.
    pub is_mut: bool,
}
