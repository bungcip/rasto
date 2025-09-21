//! Defines the AST node for an `extern` block.

use crate::ast::{ident::Ident, item_fn::ItemFn, item_macro::ItemMacro, types::Type};
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents an `extern` block, which is used to declare functions and
    /// static variables that are defined in a foreign library.
    ///
    /// # Example
    ///
    /// ```rust
/// unsafe extern "C" {
    ///     fn my_c_function(arg: i32) -> i32;
    /// }
    /// ```
    pub struct ItemExternBlock as ExternBlock without ident {
        /// `true` if the `extern` block is marked as `unsafe`.
        pub is_unsafe: bool,
        /// The Application Binary Interface (ABI) for the `extern` block,
        /// such as `"C"` or `"system"`.
        pub abi: Option<String>,
        /// The list of items declared within the `extern` block.
        pub items: ThinVec<ExternalItem>,
    }
}

/// Represents an item that can be declared within an `extern` block.
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalItem {
    /// A `static` variable declared in a foreign library.
    ///
    /// The `String` is the name of the variable, and the `Type` is its data type.
    Static(Ident, Type),
    /// A function declared in a foreign library.
    Fn(ItemFn),
    /// A macro invocation within an `extern` block.
    Macro(ItemMacro),
}
