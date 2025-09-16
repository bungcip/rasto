//! Defines the AST node for an `extern crate` item.

use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// Represents an `extern crate` item, which is used to link to a crate.
    ///
    /// In modern Rust, this is largely obsolete and `use` statements are
    /// preferred. However, it is still supported for older codebases and
    /// specific use cases.
    ///
    /// # Example
    ///
    /// ```rust
/// // extern crate proc_macro;
    /// ```
    pub struct ItemExternCrate without vis {}
}
