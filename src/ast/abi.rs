//! The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes
//! that represent Rust ABIs.

use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// An ABI, such as `"C"` or `"system"`.
#[derive(Debug, Clone, PartialEq)]
pub enum Abi {
    /// A quoted ABI, such as `"C"`.
    Named(String),
    // An unquoted ABI, such as `system`. This is not valid in modern Rust,
    // but we include it for completeness.
    // Unnamed(String),
}

impl PrettyPrinter for Abi {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Abi::Named(name) => {
                printer.string("\"");
                printer.string(name);
                printer.string("\"");
            }
        }
        Ok(())
    }
}
