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
    /// use rasto::builder::enum_def;
    ///
    /// let item = enum_def("MyEnum")
    ///     .variant("Variant1")
    ///     .variant("Variant2")
    ///     .build();
    /// ```
    ///
    /// The above builder would generate:
    /// ```rust
    /// enum MyEnum {
    ///     Variant1,
    ///     Variant2,
    /// }
    /// ```
    pub struct ItemEnum with generics {
        /// The list of variants that make up the enum.
        pub variants: ThinVec<Variant>,
    }
}

/// Represents a single, unit-like variant within an enum.
///
/// **Note:** Currently, only unit-like variants (e.g., `Variant1`) are supported.
/// Variants with data, like tuple or struct variants, are not yet represented in the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The name of the variant.
    pub ident: Ident,
    /// Metadata, such as attributes and comments, attached to the variant.
    pub md: Option<Box<Md>>,
}
