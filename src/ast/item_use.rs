//! Defines the AST node for a `use` item.

use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// Represents a `use` item, which is used to bring paths into scope.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// ```
    pub struct ItemUse without ident {
        /// The path that is being imported into the current scope.
        pub path: String,
    }
}
