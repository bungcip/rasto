use crate::ast::associated_const::AssociatedConst;
use crate::ast::associated_type::AssociatedType;
use crate::ast::item_fn::ItemFn;

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
