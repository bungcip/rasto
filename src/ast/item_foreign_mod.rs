//! Defines the AST node for a foreign module.

use crate::ast::items::Item;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents a foreign module, which is a block of declarations that are
    /// defined in a foreign, non-Rust library.
    ///
    /// This is typically used for interoperability with C libraries.
    ///
    /// # Example
    ///
    /// ```rust
/// unsafe extern "C" {
    ///     static MY_STATIC: i32;
    ///     fn my_function();
    /// }
    /// ```
    pub struct ItemForeignMod as ForeignMod without vis and ident {
        /// The Application Binary Interface (ABI) of the foreign module, such
        /// as `"C"` or `"system"`.
        pub abi: String,
        /// The list of items declared within the foreign module.
        pub items: ThinVec<Item>,
    }
}
