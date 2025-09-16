//! An item, such as a `const` definition.

use crate::{
    ast::{expressions::Expr, types::Type},
    pretty_printer::PrettyPrinter,
};

ast_item! {
    /// A `const` item, such as `const MAX: u16 = 234342;`.
    pub struct ItemConst {
        /// The type of the const item.
        pub(crate) ty: Type,
        /// The value of the const item.
        pub(crate) expr: Box<Expr>,
    }
}
