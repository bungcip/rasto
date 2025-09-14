//! Defines the visibility of an item.

/// Represents the visibility of an item in the AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Visibility {
    /// Public visibility, denoted by `pub`.
    Public,
    /// Crate-level visibility, denoted by `pub(crate)`.
    Crate,
    /// Default visibility, which is private.
    #[default]
    Default,
}

use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

impl PrettyPrinter for Visibility {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Visibility::Public => {
                printer.string("pub ");
            }
            Visibility::Crate => {
                printer.string("pub(crate) ");
            }
            Visibility::Default => {}
        }
        Ok(())
    }
}
