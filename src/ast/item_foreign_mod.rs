use crate::ast::items::Item;
use crate::ast::metadata::Md;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A foreign module, such as `extern "C" { ... }`.
///
/// This contains a list of items that are defined in a foreign library.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemForeignMod {
    /// The ABI of the foreign module, e.g., `"C"`.
    pub abi: String,
    /// The items within the foreign module.
    pub items: ThinVec<Item>,
    /// Metadata about the foreign module, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemForeignMod {
    /// Formats the `ItemForeignMod` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
