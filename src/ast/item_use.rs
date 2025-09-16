use crate::pretty_printer::PrettyPrinter;
use compact_str::CompactString;

ast_item! {
    /// A `use` item, such as `use std::collections::HashMap;`.
    pub struct ItemUse without ident {
        /// The path being imported.
        pub path: CompactString,
    }
}
