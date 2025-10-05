//! Defines the AST nodes for an `impl` block.
//!
//! An `impl` block is used to implement methods on a type, or to implement a
//! trait for a type.

use crate::ast::{
    associated_const::AssociatedConst, associated_type::AssociatedType, generics::GenericParams,
    item_fn::ItemFn, metadata::Md, types::Type,
};
use crate::pretty_printer::{pp_begin, pp_end, BreakStyle, PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// Represents an `impl` block, which is used to define implementations
/// of methods on a type or to implement a trait for a type.
///
/// # Example
///
/// Implementing a method on a struct:
/// ```rust
/// struct MyStruct;
///
/// impl MyStruct {
///     fn my_method(&self) {}
/// }
/// ```
///
/// Implementing a trait for a struct:
/// ```rust
/// trait MyTrait {
///     fn trait_method(&self);
/// }
/// struct MyStruct;
///
/// impl MyTrait for MyStruct {
///     fn trait_method(&self) {}
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ItemImpl {
    /// `true` if the `impl` block is `unsafe`.
    pub is_unsafe: bool,
    /// `true` if the `impl` block is a negative implementation (e.g., `impl !Send for MyType`).
    pub is_negative: bool,
    /// The trait being implemented, if any.
    ///
    /// If this is `None`, it is an inherent `impl`.
    pub trait_: Option<Type>,
    /// The type that the `impl` block is for.
    pub ty: Type,
    /// The list of items within the `impl` block, such as methods,
    /// associated types, and associated constants.
    pub items: ThinVec<ImplItem>,
    /// The generic parameters of the `impl` block.
    pub generics: GenericParams,
    /// Metadata about the `impl` block, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl_display_for_item!(ItemImpl);

impl PrettyPrinter for ItemImpl {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        if self.is_unsafe {
            printer.string("unsafe ");
        }
        printer.string("impl");
        self.generics.pretty_print(printer)?;
        printer.string(" ");

        if let Some(trait_) = &self.trait_ {
            if self.is_negative {
                printer.string("!");
            }
            trait_.pretty_print(printer)?;
            printer.string(" for ");
        }

        self.ty.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.items.is_empty() {
            printer.hard_break();
            let num_items = self.items.len();
            for (i, item) in self.items.iter().enumerate() {
                item.pretty_print(printer)?;
                if i < num_items - 1 {
                    printer.hard_break();
                }
            }
        }
        printer.end("}");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

/// Represents an item that can appear within an `impl` block.
#[derive(Debug, Clone, PartialEq)]
pub enum ImplItem {
    /// A function or method.
    Fn(ItemFn),
    /// An associated type.
    Type(AssociatedType),
    /// An associated constant.
    Const(AssociatedConst),
}

impl PrettyPrinter for ImplItem {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            ImplItem::Fn(item) => item.pretty_print(printer),
            ImplItem::Type(item) => item.pretty_print(printer),
            ImplItem::Const(item) => item.pretty_print(printer),
        }
    }
}

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