use crate::ast::metadata::{self, Md};
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `use` item, such as `use std::collections::HashMap;`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemUse {
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

impl PrettyPrinter for ItemUse {
    /// Pretty-prints the `ItemUse` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("use ");
        printer.string(&self.path);
        printer.string(";");
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
