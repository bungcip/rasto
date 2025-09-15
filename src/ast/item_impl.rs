use crate::ast::associated_type::AssociatedType;
use crate::ast::expressions::Expr;
use crate::ast::items::ItemFn;
use crate::ast::metadata::Md;
use crate::ast::types::Type;

impl From<ItemFn> for ImplItem {
    fn from(item: ItemFn) -> Self {
        ImplItem::Fn(item)
    }
}

impl From<AssociatedType> for ImplItem {
    fn from(item: AssociatedType) -> Self {
        ImplItem::Type(item)
    }
}

impl From<AssociatedConst> for ImplItem {
    fn from(item: AssociatedConst) -> Self {
        ImplItem::Const(item)
    }
}

/// An item within an `impl` block.
#[derive(Debug, Clone, PartialEq)]
pub enum ImplItem {
    /// A function item within the `impl` block.
    Fn(ItemFn),
    /// An associated type within the `impl` block.
    Type(AssociatedType),
    /// An associated constant within the `impl` block.
    Const(AssociatedConst),
}

/// An associated constant within an `impl` block.
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedConst {
    /// The name of the constant.
    pub ident: String,
    /// The type of the constant.
    pub ty: Type,
    /// The value of the constant.
    pub expr: Expr,
    /// Metadata about the constant, including attributes and comments.
    pub md: Option<Box<Md>>,
}
