//! Defines the AST node for an enum definition.

use crate::ast::metadata::Md;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// An enum definition.
    pub struct ItemEnum with generics {
        /// The variants of the enum.
        pub variants: ThinVec<Variant>,
    }
}

/// A variant of an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The name of the variant.
    pub ident: String,
    /// Metadata about the variant, including attributes and comments.
    pub md: Option<Box<Md>>,
}
