use crate::ast::generics::GenericParams;
use crate::ast::items::Field;
use crate::ast::metadata::Md;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A `union` definition, such as `union MyUnion { f1: u32, f2: f32 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemUnion {
    /// The name of the union.
    pub ident: String,
    /// The generic parameters of the union.
    pub generics: GenericParams,
    /// The fields of the union.
    pub fields: ThinVec<Field>,
    /// Metadata about the union, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemUnion {
    /// Formats the `ItemUnion` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
