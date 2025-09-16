//! Defines the AST nodes for items within an `impl` block.

use crate::ast::associated_const::AssociatedConst;
use crate::ast::associated_type::AssociatedType;
use crate::ast::item_fn::ItemFn;

impl From<ItemFn> for ImplItem {
    /// Converts an `ItemFn` into an `ImplItem`.
    fn from(item: ItemFn) -> Self {
        ImplItem::Fn(item)
    }
}

impl From<AssociatedType> for ImplItem {
    /// Converts an `AssociatedType` into an `ImplItem`.
    fn from(item: AssociatedType) -> Self {
        ImplItem::Type(item)
    }
}

impl From<AssociatedConst> for ImplItem {
    /// Converts an `AssociatedConst` into an `ImplItem`.
    fn from(item: AssociatedConst) -> Self {
        ImplItem::Const(item)
    }
}

/// Represents an item that can appear within an `impl` block, such as a
/// method, an associated type, or an associated constant.
#[derive(Debug, Clone, PartialEq)]
pub enum ImplItem {
    /// A function or method defined in the `impl` block.
    Fn(ItemFn),
    /// An associated type definition within the `impl` block.
    Type(AssociatedType),
    /// An associated constant definition within the `impl` block.
    Const(AssociatedConst),
}
