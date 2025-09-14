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
