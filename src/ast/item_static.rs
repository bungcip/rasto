//! Defines the AST node for a `static` item.

use crate::ast::expressions::Expr;
use crate::ast::types::Type;
use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// Represents a `static` item, which is a value that has a fixed memory
    /// location and can be either immutable or mutable.
    ///
    /// # Examples
    ///
    /// An immutable static item:
    ///
    /// ```rust
    /// static MY_STATIC: i32 = 42;
    /// ```
    ///
    /// A mutable static item:
    ///
    /// ```rust
    /// static mut MY_MUT_STATIC: i32 = 0;
    /// ```
    pub struct ItemStatic as Static {
        /// `true` if the static item is mutable (`static mut`).
        pub is_mut: bool,
        /// The data type of the static item.
        pub ty: Type,
        /// The expression that provides the initial value of the static item.
        pub expr: Box<Expr>,
    }
}
