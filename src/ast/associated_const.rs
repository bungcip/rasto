//! An associated constant in a trait or impl.

use crate::ast::expressions::Expr;
use crate::ast::metadata::Md;
use crate::ast::types::Type;
use compact_str::CompactString;

/// An associated constant in a trait or impl, such as `const MAX: u16 = 234342;`.
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedConst {
    /// The name of the const item.
    pub ident: CompactString,
    /// The type of the const item.
    pub ty: Type,
    /// The value of the const item.
    pub expr: Option<Box<Expr>>,
    /// Metadata about the item, including attributes and comments.
    pub md: Option<Box<Md>>,
}
