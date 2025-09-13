//! Defines the AST nodes for attributes.
//!
//! Attributes are metadata that can be attached to various items in Rust code. They are
//! enclosed in `#[...]` for outer attributes and `#![...]` for inner attributes.

use crate::ast::literals::Lit;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// An attribute, such as `#[repr(C)]` or `#![allow(dead_code)]`.
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    /// An inner attribute, such as `#![allow(dead_code)]`.
    Inner(Meta),
    /// An outer attribute, such as `#[repr(C)]`.
    Outer(Meta),
}

/// The meta item within an attribute.
///
/// For example, in `#[repr(C)]`, the meta item is `repr(C)`.
#[derive(Debug, Clone, PartialEq)]
pub enum Meta {
    /// A meta list, such as `repr(C)`.
    List(MetaList),
    /// A meta path, such as `test`.
    Path(String),
    /// A meta name-value pair, such as `key = "value"`.
    NameValue(MetaNameValue),
}

/// A meta list, such as `repr(C)`.
#[derive(Debug, Clone, PartialEq)]
pub struct MetaList {
    /// The path of the meta list, e.g., `repr`.
    pub path: String,
    /// The meta items within the list, e.g., `C`.
    pub metas: ThinVec<Meta>,
}

/// A meta name-value pair, such as `key = "value"`.
#[derive(Debug, Clone, PartialEq)]
pub struct MetaNameValue {
    /// The path of the meta name-value pair, e.g., `key`.
    pub path: String,
    /// The value of the meta name-value pair, e.g., `"value"`.
    pub value: Lit,
}

impl PrettyPrinter for Attribute {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Attribute::Inner(meta) => {
                printer.string("#![");
                meta.pretty_print(printer)?;
                printer.string("]");
            }
            Attribute::Outer(meta) => {
                printer.string("#[");
                meta.pretty_print(printer)?;
                printer.string("]");
            }
        }
        Ok(())
    }
}

impl PrettyPrinter for Meta {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Meta::List(list) => list.pretty_print(printer),
            Meta::Path(path) => {
                printer.string(path);
                Ok(())
            }
            Meta::NameValue(name_value) => name_value.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for MetaList {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.path);
        printer.string("(");
        for (i, meta) in self.metas.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            meta.pretty_print(printer)?;
        }
        printer.string(")");
        Ok(())
    }
}

impl PrettyPrinter for MetaNameValue {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.path);
        printer.string(" = ");
        self.value.pretty_print(printer)
    }
}
