//! Defines the AST node for an identifier.
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// An identifier, such as `my_variable` or `r#true`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Ident {
    /// The name of the identifier, without the `r#` prefix.
    pub name: String,
    /// `true` if the identifier is a raw identifier (e.g., `r#true`).
    pub is_raw: bool,
}

impl Ident {
    /// Creates a new identifier.
    pub fn new(name: impl Into<String>, is_raw: bool) -> Self {
        Self {
            name: name.into(),
            is_raw,
        }
    }
}

impl<T: Into<String>> From<T> for Ident {
    fn from(s: T) -> Self {
        let s = s.into();
        if let Some(s) = s.strip_prefix("r#") {
            Self::new(s, true)
        } else {
            Self::new(s, false)
        }
    }
}

impl PrettyPrinter for Ident {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if self.is_raw {
            printer.string("r#");
        }
        printer.string(&self.name);
        Ok(())
    }
}
