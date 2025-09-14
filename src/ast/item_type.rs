use crate::ast::generics::GenericParams;
use crate::ast::metadata::Md;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A type alias, such as `type MyResult<T> = Result<T, MyError>;`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemType {
    /// The name of the type alias.
    pub ident: String,
    /// The generic parameters of the type alias.
    pub generics: GenericParams,
    /// The type being aliased.
    pub ty: Type,
    /// Metadata about the type item, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemType {
    /// Formats the `ItemType` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
