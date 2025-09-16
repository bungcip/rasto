//! Defines the AST node for a foreign module.

use crate::ast::items::Item;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// A foreign module, such as `extern "C" { ... }`.
    ///
    /// This contains a list of items that are defined in a foreign library.
    pub struct ItemForeignMod without vis and ident {
        /// The ABI of the foreign module, e.g., `"C"`.
        pub abi: String,
        /// The items within the foreign module.
        pub items: ThinVec<Item>,
    }
}
