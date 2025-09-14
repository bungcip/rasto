//! Defines the metadata for an AST node.
use core::fmt;

use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::pretty_printer::{PrettyPrinter, Printer};
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

/// Pretty-prints the leading metadata of an AST node.
///
/// This includes attributes and leading comments.
pub fn pp_begin<'a>(md: &'a Option<Box<Md>>, printer: &mut Printer<'a>) -> fmt::Result {
    if let Some(md) = &md {
        for attr in &md.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &md.leading_comments {
            comment.pretty_print(printer)?;
        }
    }
    Ok(())
}

/// Pretty-prints the trailing metadata of an AST node.
///
/// This includes trailing comments.
pub fn pp_end<'a>(md: &'a Option<Box<Md>>, printer: &mut Printer<'a>) -> fmt::Result {
    if let Some(md) = &md {
        for comment in &md.trailing_comments {
            comment.pretty_print(printer)?;
        }
    }
    Ok(())
}
