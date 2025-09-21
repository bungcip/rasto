//! An item, such as a `const` definition.

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
    pub struct ItemConst as Const {
        /// The type of the const item.
        pub(crate) ty: Type,
        /// The value of the const item.
        pub(crate) expr: Box<Expr>,
    }
}
