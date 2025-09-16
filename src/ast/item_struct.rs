//! Defines the AST nodes for a struct definition.

use crate::ast::metadata::Md;
use crate::ast::types::Type;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// A struct definition.
    pub struct ItemStruct with generics {
        /// The fields of the struct.
        pub fields: ThinVec<Field>,
    }
}

/// A field of a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub ident: String,
    /// The type of the field.
    pub ty: Type,
    /// Metadata about the field, including attributes and comments.
    pub md: Option<Box<Md>>,
}
