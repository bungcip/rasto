//! Defines the AST node for an enum definition.

use crate::ast::{ident::Ident, metadata::Md};
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents an `enum` definition, which is a type that can be one of
    /// several variants.
    ///
    /// # Example
    ///
    /// ```rust
    /// enum MyEnum {
    ///     Variant1,
    ///     Variant2(u32),
    ///     Variant3 { x: i32, y: i32 },
    /// }
    /// ```
    pub struct ItemEnum as Enum with generics {
        /// The list of variants that make up the enum.
        pub variants: ThinVec<Variant>,
    }
}

/// Represents a single variant within an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The name of the variant.
    pub ident: Ident,
    /// Metadata, such as attributes and comments, attached to the variant.
    pub md: Option<Box<Md>>,
}
