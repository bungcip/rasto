//! Defines the representation of comments in the AST.

/// Represents a comment in the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
    /// A line comment, starting with `//`.
    ///
    /// The string contains the content of the comment, without the `//`.
    Line(String),
    /// A block comment, enclosed in `/* ... */`.
    ///
    /// The string contains the content of the comment, without the `/*` and `*/`.
    Block(String),
}
