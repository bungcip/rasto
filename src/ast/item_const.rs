use crate::ast::expressions::Expr;
use crate::ast::metadata::Md;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `const` item, such as `const MAX: u16 = 234342;`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemConst {
    /// The name of the const item.
    pub ident: String,
    /// The type of the const item.
    pub ty: Type,
    /// The value of the const item.
    pub expr: Box<Expr>,
    /// Metadata about the const item, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemConst {
    /// Formats the `ItemConst` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
