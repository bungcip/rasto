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
    pub leading_comments: ThinVec<Comment>,
    /// Comments that appear after the node.
    pub trailing_comments: ThinVec<Comment>,
}

/// Builder for `Md`.
#[derive(Debug, Clone, Default)]
pub struct MdBuilder {
    attrs: ThinVec<Attribute>,
    leading_comments: ThinVec<Comment>,
    trailing_comments: ThinVec<Comment>,
}

impl MdBuilder {
    /// Creates a new `MdBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an attribute.
    pub fn attr(mut self, attr: Attribute) -> Self {
        self.attrs.push(attr);
        self
    }

    /// Adds a leading comment.
    pub fn leading_comment(mut self, comment: Comment) -> Self {
        self.leading_comments.push(comment);
        self
    }

    /// Adds a trailing comment.
    pub fn trailing_comment(mut self, comment: Comment) -> Self {
        self.trailing_comments.push(comment);
        self
    }

    /// Builds the `Md`.
    pub fn build(self) -> Md {
        Md {
            attrs: self.attrs,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}
