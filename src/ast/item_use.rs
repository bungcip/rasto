//! Defines the AST node for a `use` item.

use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// A `use` item, such as `use std::collections::HashMap;`.
    pub struct ItemUse without ident {
        /// The path being imported.
        pub path: String,
    }
}
