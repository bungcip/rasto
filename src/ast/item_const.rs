//! Defines the AST node for a `const` item.
//!
//! A `const` item is a value that is computed at compile time and can be used
//! in any constant context.

use crate::{
    ast::{expressions::Expr, types::Type},
    pretty_printer::PrettyPrinter,
};

ast_item! {
    /// A `const` item, such as `const MAX: u16 = 234342;`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rasto::builder::{const_def, expr};
    ///
    /// let item = const_def("MY_CONST", "u32", expr().lit(123)).build();
    /// ```
    pub struct ItemConst {
        /// The type of the const item.
        pub ty: Type,
        /// The value of the const item.
        pub expr: Box<Expr>,
    }
}
