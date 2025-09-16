//! Defines the AST node for a macro invocation in an item position.

use crate::ast::expressions::Expr;
use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// Represents a macro invocation that appears in a position where an item
    /// is expected.
    ///
    /// This is used for macros that generate items, such as functions, structs,
    /// or modules.
    ///
    /// # Example
    ///
    /// ```rust
    /// // A macro that defines a function
    /// macro_rules! define_my_func {
    ///     () => {
    ///         fn my_func() {}
    ///     };
    /// }
    ///
    /// // The macro invocation is an `ItemMacro`
    /// define_my_func!();
    /// ```
    pub struct ItemMacro without vis and ident {
        /// The macro invocation expression itself.
        pub expr: Box<Expr>,
    }
}
