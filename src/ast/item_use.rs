use crate::ast::metadata::Md;
use crate::ast::visibility::Visibility;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `use` item, such as `use std::collections::HashMap;`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemUse {
    /// The visibility of the use item.
    pub vis: Visibility,
    /// The path being imported.
    pub path: String,
    /// Metadata about the use item, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemUse {
    /// Formats the `ItemUse` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
