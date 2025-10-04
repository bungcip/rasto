//! Defines the metadata for an AST node.

use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use thin_vec::ThinVec;

/// Metadata for an AST node, including attributes and comments.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Md {
    /// Attributes that appear before the node.
    pub attrs: ThinVec<Attribute>,
    /// Comments that appear before the node.
    pub comments: ThinVec<Comment>,
    /// Comments that appear after the node.
    pub trailing_comments: ThinVec<Comment>,
}

/// A builder for constructing `Md` (metadata) for an AST node.
///
/// This builder provides a fluent interface for adding attributes and comments
/// to an AST node's metadata.
#[derive(Debug, Clone, Default)]
pub struct MdBuilder {
    attrs: ThinVec<Attribute>,
    comments: ThinVec<Comment>,
    trailing_comments: ThinVec<Comment>,
}

impl MdBuilder {
    /// Creates a new, empty `MdBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an attribute to the metadata.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: Attribute) -> Self {
        self.attrs.push(attr);
        self
    }

    /// Adds a comment to the metadata.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: Comment) -> Self {
        self.comments.push(comment);
        self
    }

    /// Adds a trailing comment to the metadata.
    ///
    /// Trailing comments are comments that appear on the same line after an item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The trailing comment to add.
    pub fn trailing_comment(mut self, comment: Comment) -> Self {
        self.trailing_comments.push(comment);
        self
    }

    /// Builds and returns the `Md` struct.
    pub fn build(self) -> Md {
        Md {
            attrs: self.attrs,
            comments: self.comments,
            trailing_comments: self.trailing_comments,
        }
    }
}
