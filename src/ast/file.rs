//! Defines the top-level AST node for a Rust file.

use crate::ast::items::Item;
use crate::pretty_printer::{PrettyPrinter, Printer};

/// Represents a Rust source file.
///
/// A `File` is the root of the AST and contains a list of items,
/// which are the top-level declarations in the file, such as functions,
/// structs, and modules.
#[derive(Debug, Clone, PartialEq)]
pub struct File {
    /// The top-level items in the file.
    pub items: Vec<Item>,
}

impl File {
    /// Pretty-prints the file to a string.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted Rust code.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}
