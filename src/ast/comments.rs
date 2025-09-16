//! Defines the representation of comments in the AST.

use compact_str::CompactString;

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
    Line(CompactString),
    /// A block comment, enclosed in `/* ... */`.
    ///
    /// The string contains the content of the comment, without the `/*` and `*/`.
    ///
    /// # Example
    ///
    /// ```text
    /// /* This is a block comment. */
    /// ```
    Block(CompactString),
    /// A doc comment, starting with `///`.
    ///
    /// The string contains the content of the comment, without the `///`.
    ///
    /// # Example
    ///
    /// ```text
    /// /// This is a doc comment.
    /// ```
    Doc(CompactString),
}
