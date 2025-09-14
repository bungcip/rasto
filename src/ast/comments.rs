//! Defines the representation of comments in the AST.

/// Represents a comment in the source code.
///
/// Comments can be either line comments or block comments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
    /// A line comment, starting with `//`.
    ///
    /// The string contains the content of the comment, without the `//`.
    ///
    /// # Example
    ///
    /// ```text
    /// // This is a line comment.
    /// ```
    Line(String),
    /// A block comment, enclosed in `/* ... */`.
    ///
    /// The string contains the content of the comment, without the `/*` and `*/`.
    ///
    /// # Example
    ///
    /// ```text
    /// /* This is a block comment. */
    /// ```
    Block(String),
}
