use crate::ast::expressions::Expr;
use crate::ast::metadata::Md;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `static` item, such as `static COUNTER: u32 = 0;`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStatic {
    /// The name of the static item.
    pub ident: String,
    /// The type of the static item.
    pub ty: Type,
    /// The value of the static item.
    pub expr: Box<Expr>,
    /// Metadata about the static item, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemStatic {
    /// Formats the `ItemStatic` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
