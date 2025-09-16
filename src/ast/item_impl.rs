//! Defines the AST nodes for `impl` items.

use crate::ast::associated_const::AssociatedConst;
use crate::ast::associated_type::AssociatedType;
use crate::ast::item_fn::ItemFn;

impl From<ItemFn> for ImplItem {
    /// Converts an `ItemFn` into an `ImplItem::Fn`.
    fn from(item: ItemFn) -> Self {
        ImplItem::Fn(item)
    }
}

impl From<AssociatedType> for ImplItem {
    /// Converts an `AssociatedType` into an `ImplItem::Type`.
    fn from(item: AssociatedType) -> Self {
        ImplItem::Type(item)
    }
}

impl From<AssociatedConst> for ImplItem {
    /// Converts an `AssociatedConst` into an `ImplItem::Const`.
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
