use crate::ast::metadata::Md;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// An `extern crate` item, such as `extern crate serde;`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemExternCrate {
    /// The name of the crate being imported.
    pub ident: String,
    /// Metadata about the extern crate item, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemExternCrate {
    /// Formats the `ItemExternCrate` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
