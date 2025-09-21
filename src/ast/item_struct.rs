//! Defines the AST node for a struct definition.

use crate::ast::{ident::Ident, metadata::Md, types::Type};
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents a `struct` definition, which is a custom data type that
    /// groups together related values.
    ///
    /// # Example
    ///
    /// ```rust
    /// struct MyStruct {
    ///     field1: i32,
    ///     field2: String,
    /// }
    /// ```
    pub struct ItemStruct as Struct with generics {
        /// The list of fields that make up the struct.
        pub fields: ThinVec<Field>,
    }
}

/// Represents a single field within a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub ident: Ident,
    /// The data type of the field.
    pub ty: Type,
    /// Metadata, such as attributes and comments, attached to the field.
    pub md: Option<Box<Md>>,
}
