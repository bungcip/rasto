//! Defines the AST node for a module definition.

use crate::ast::items::Item;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents a module, which is a container for other items.
    ///
    /// Modules can be declared inline with curly braces `{}` or in a separate
    /// file with a semicolon `;`.
    ///
    /// # Examples
    ///
    /// Inline module:
    ///
    /// ```rust
    /// mod my_module {
    ///     fn my_function() {}
    /// }
    /// ```
    ///
    /// Module in a separate file:
    ///
    /// ```rust,ignore
    /// mod my_module;
    /// ```
    pub struct ItemMod as Mod {
        /// The content of the module, if it is defined inline.
        ///
        /// If this is `None`, the module is declared with a semicolon, and its
        /// content is expected to be in a separate file.
        pub content: Option<ThinVec<Item>>,
    }
}
