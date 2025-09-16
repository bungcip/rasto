use crate::ast::expressions::Expr;
use crate::ast::types::Type;
use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// A static item.
    pub struct ItemStatic {
        /// Whether the static item is mutable.
        pub is_mut: bool,
        /// The type of the static item.
        pub ty: Type,
        /// The value of the static item.
        pub expr: Box<Expr>,
    }
}
