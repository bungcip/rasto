//! Defines the AST node for a `union` definition.

use crate::ast::item_struct::Field;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents a `union` definition, which is a data structure that can hold
    /// one of several possible variants at a time.
    ///
    /// Accessing the fields of a `union` is `unsafe` because the compiler
    /// cannot guarantee which variant is currently active.
    ///
    /// # Example
    ///
    /// ```rust
    /// union MyUnion {
    ///     f1: u32,
    ///     f2: f32,
    /// }
    /// ```
    pub struct ItemUnion with generics {
        /// The list of fields that are part of the union.
        pub fields: ThinVec<Field>,
    }
}
