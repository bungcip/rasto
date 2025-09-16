//! Defines the AST node for a module.

use crate::ast::items::Item;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// A module, such as `mod my_module;` or `mod my_module { ... }`.
    pub struct ItemMod {
        /// The content of the module. If this is `None`, the module is declared
        /// with a semicolon, and the content is in a separate file.
        pub content: Option<ThinVec<Item>>,
    }
}
