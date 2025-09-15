use crate::ast::expressions::Expr;
use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// A macro invocation in an items position, such as `my_macro!();`.
    pub struct ItemMacro without vis and ident {
        /// The macro invocation expression.
        pub expr: Box<Expr>,
    }
}
