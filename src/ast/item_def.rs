use crate::ast::expressions::Expr;
use crate::ast::generics::GenericParams;
use crate::ast::metadata::Md;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// An item, such as a `const`, `static`, or `type` definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemDef {
    /// The name of the item.
    pub ident: String,
    /// Metadata about the item, including attributes and comments.
    pub md: Option<Box<Md>>,
    /// The kind of the item.
    pub kind: ItemDefKind,
}

/// The kind of an item.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemDefKind {
    /// A `const` item, such as `const MAX: u16 = 234342;`.
    Const {
        /// The type of the const item.
        ty: Type,
        /// The value of the const item.
        expr: Box<Expr>,
    },
    /// A `static` item, such as `static COUNTER: u32 = 0;`.
    Static {
        /// The type of the static item.
        ty: Type,
        /// The value of the static item.
        expr: Box<Expr>,
    },
    /// A type alias, such as `type MyResult<T> = Result<T, MyError>;`.
    TypeAlias {
        /// The generic parameters of the type alias.
        generics: GenericParams,
        /// The type being aliased.
        ty: Type,
    },
}

impl fmt::Display for ItemDef {
    /// Formats the `ItemDef` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
