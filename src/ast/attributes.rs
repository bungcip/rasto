//! Defines the AST nodes for attributes.
//!
//! Attributes are metadata that can be attached to various items in Rust code. They are
//! enclosed in `#[...]` for outer attributes and `#![...]` for inner attributes.

use crate::ast::literals::Lit;
use compact_str::CompactString;
use thin_vec::ThinVec;

/// An attribute, such as `#[repr(C)]` or `#![allow(dead_code)]`.
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    /// An inner attribute, such as `#![allow(dead_code)]`.
    ///
    /// Inner attributes apply to the item that contains them.
    Inner(Meta),
    /// An outer attribute, such as `#[repr(C)]`.
    ///
    /// Outer attributes apply to the item that follows them.
    Outer(Meta),
}

/// The meta item within an attribute.
///
/// For example, in `#[repr(C)]`, the meta item is `repr(C)`.
#[derive(Debug, Clone, PartialEq)]
pub enum Meta {
    /// A meta list, such as `repr(C)`. This is a path followed by a list of
    /// meta items in parentheses.
    List(MetaList),
    /// A meta path, such as `test`. This is a single path.
    Path(CompactString),
    /// A meta name-value pair, such as `key = "value"`.
    NameValue(MetaNameValue),
}

/// A meta list, such as `repr(C)`.
#[derive(Debug, Clone, PartialEq)]
pub struct MetaList {
    /// The path of the meta list, e.g., `repr`.
    pub path: CompactString,
    /// The meta items within the list, e.g., `C`.
    pub metas: ThinVec<Meta>,
}

/// A meta name-value pair, such as `key = "value"`.
#[derive(Debug, Clone, PartialEq)]
pub struct MetaNameValue {
    /// The path of the meta name-value pair, e.g., `key`.
    pub path: CompactString,
    /// The value of the meta name-value pair, e.g., `"value"`.
    pub value: Lit,
}

impl From<&str> for Meta {
    /// Converts a string slice into a `Meta::Path`.
    ///
    /// This is a convenience function for creating simple path-based meta items.
    fn from(value: &str) -> Meta {
        Meta::Path(value.into())
    }
}
