//! Defines the AST node for a top-level `extern type` item.
//!
//! An `extern type` is a declaration of an opaque type whose size and alignment
//! are not known to the compiler. It is used to represent types from foreign
//! libraries. This feature is currently unstable and requires the `extern_types`
//! feature gate.

use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// Represents a top-level `extern type` item, which declares an opaque type
    /// defined in a foreign library.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # #![feature(extern_types)]
    /// use rasto::builder::extern_type_item;
    /// use rasto::ast::Visibility;
    /// use rasto::pretty;
    ///
    /// let item = extern_type_item("MyForeignType")
    ///     .vis(Visibility::Public)
    ///     .build();
    ///
    /// assert_eq!(pretty(&item), "pub extern type MyForeignType;");
    /// ```
    pub struct ItemExternType {}
}
