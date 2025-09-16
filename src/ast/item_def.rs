use crate::ast::expressions::Expr;
use crate::ast::generics::GenericParams;
use crate::ast::types::Type;
use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// An item, such as a `const`, `static`, or `type` definition.
    pub struct ItemDef {
        /// The kind of the item.
        pub kind: ItemDefKind,
    }
}

/// The kind of an item.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemDefKind {
    /// A `const` item, such as `const MAX: u16 = 234342;`.
    Const {
        /// The type of the const item.
        ty: Type,
        /// The value of the const item.
        expr: Box<Expr>,
    },
    /// A `static` item, such as `static COUNTER: u32 = 0;`.
    Static {
        /// The type of the static item.
        ty: Type,
        /// The value of the static item.
        expr: Box<Expr>,
    },
    /// A type alias, such as `type MyResult<T> = Result<T, MyError>;`.
    TypeAlias {
        /// The generic parameters of the type alias.
        generics: GenericParams,
        /// The type being aliased.
        ty: Type,
    },
}
