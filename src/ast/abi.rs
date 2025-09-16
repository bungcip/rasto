//! The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes
//! that represent Rust ABIs.

use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// Represents a Rust ABI (Application Binary Interface).
///
/// This enum is used to specify the calling convention for functions,
/// especially in the context of `extern` blocks and function pointers.
///
/// # Examples
///
/// ```rust
/// use rasto::ast::Abi;
///
/// // A named ABI, like "C"
/// let c_abi = Abi::Named("C".to_string());
///
/// // A named ABI, like "system"
/// let system_abi = Abi::Named("system".to_string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Abi {
    /// Represents a named ABI, specified as a string literal.
    ///
    /// For example, `"C"`, `"system"`, or `"Rust"`.
    Named(String),
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
