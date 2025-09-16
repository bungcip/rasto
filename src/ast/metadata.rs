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

/// A builder for creating `Md` instances.
#[derive(Debug, Clone, Default)]
pub struct MdBuilder {
    /// The attributes of the metadata.
    pub attrs: ThinVec<Attribute>,
    /// The comments of the metadata.
    pub comments: ThinVec<Comment>,
    /// The trailing comments of the metadata.
    pub trailing_comments: ThinVec<Comment>,
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
    /// * `attr` - The attribute to add.
    pub fn attr(mut self, attr: Attribute) -> Self {
        self.attrs.push(attr);
        self
    }

    /// Adds a comment to the metadata.
    ///
    /// # Parameters
    ///
    /// * `comment` - The comment to add.
    pub fn comment(mut self, comment: Comment) -> Self {
        self.comments.push(comment);
        self
    }

    /// Adds a trailing comment to the metadata.
    ///
    /// # Parameters
    ///
    /// * `comment` - The trailing comment to add.
    pub fn trailing_comment(mut self, comment: Comment) -> Self {
        self.trailing_comments.push(comment);
        self
    }

    /// Builds the `Md` instance.
    pub fn build(self) -> Md {
        Md {
            attrs: self.attrs,
            comments: self.comments,
            trailing_comments: self.trailing_comments,
        }
    }
}
