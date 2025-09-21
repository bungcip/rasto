//! Defines the AST nodes for top-level items in a Rust file.
//!
//! Items are the primary components of a Rust program, such as functions, structs, enums,
//! impl blocks, and traits. They are the top-level declarations that make up a crate.

use crate::ast::associated_const::AssociatedConst;
use crate::ast::generics::GenericParams;
use crate::ast::item_asm::ItemAsm;
use crate::ast::item_const::ItemConst;
use crate::ast::item_enum::ItemEnum;
use crate::ast::item_extern_block::ItemExternBlock;
use crate::ast::item_extern_crate::ItemExternCrate;
use crate::ast::item_fn::{ItemFn, Signature};
use crate::ast::item_foreign_mod::ItemForeignMod;
use crate::ast::item_impl::ImplItem;
use crate::ast::item_macro::ItemMacro;
use crate::ast::item_mod::ItemMod;
use crate::ast::item_static::ItemStatic;
use crate::ast::item_struct::ItemStruct;
use crate::ast::item_trait::ItemTrait;
use crate::ast::item_trait_alias::ItemTraitAlias;
use crate::ast::item_type_alias::ItemTypeAlias;
use crate::ast::item_union::ItemUnion;
use crate::ast::item_use::ItemUse;
use crate::ast::metadata::Md;
use crate::ast::statements::Block;
use crate::ast::types::Type;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

/// A top-level item in a Rust file.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// An `asm!` block.
    Asm(ItemAsm),
    /// A `const` item: `const FOO: u32 = 42;`.
    Const(ItemConst),
    /// A function item: `fn foo() { ... }`.
    Fn(ItemFn),
    /// A struct item: `struct Foo { ... }`.
    Struct(ItemStruct),
    /// A static item: `static FOO: u32 = 42;`.
    Static(ItemStatic),
    /// An enum item: `enum Foo { ... }`.
    Enum(ItemEnum),
    /// An `impl` block: `impl Foo { ... }`.
    Impl(ItemImpl),
    /// A trait item: `trait Foo { ... }`.
    Trait(ItemTrait),
    /// An `extern crate` item: `extern crate semver;`.
    ExternCrate(ItemExternCrate),
    /// A foreign module: `extern "C" { ... }`.
    ForeignMod(ItemForeignMod),
    /// An `extern` block: `extern "C" { ... }`.
    ExternBlock(ItemExternBlock),
    /// A macro definition: `macro_rules! ...`.
    Macro(ItemMacro),
    /// A module: `mod foo { ... }`.
    Mod(ItemMod),
    /// A trait alias: `trait Foo = Bar;`.
    TraitAlias(ItemTraitAlias),
    /// A type alias: `type Foo = Bar;`.
    TypeAlias(ItemTypeAlias),
    /// A `union` item: `union Foo { ... }`.
    Union(ItemUnion),
    /// A `use` item: `use std::collections::HashMap;`.
    Use(ItemUse),
    /// A test-only item.
    #[cfg(test)]
    Test(TestItem),
}

impl_display_for_item!(Item);

/// A test-only item.
#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct TestItem {
    /// The visibility of the item.
    pub vis: crate::ast::visibility::Visibility,
    /// The name of the item.
    pub ident: crate::ast::ident::Ident,
    /// The generic parameters of the item.
    pub generics: crate::ast::generics::GenericParams,
    /// Metadata about the item, including attributes and comments.
    pub md: Option<Box<crate::ast::metadata::Md>>,
    /// The fields of the item.
    pub fields: Vec<String>,
}

/// An item within a trait definition.
#[derive(Debug, Clone, PartialEq)]
pub enum TraitItem {
    /// A function item within a trait: `fn foo();`.
    Fn(TraitItemFn),
    /// A const item within a trait: `const FOO: usize;`.
    Const(AssociatedConst),
}

/// A function item within a trait.
#[derive(Debug, Clone, PartialEq)]
pub struct TraitItemFn {
    /// The function signature.
    pub sig: Signature,
    /// An optional default implementation of the function.
    pub block: Option<Block>,
    /// Metadata about the function, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl_display_for_item!(ItemImpl);

/// An `impl` block.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemImpl {
    /// The type the `impl` block is for.
    pub ty: Type,
    /// The trait being implemented, if any.
    pub trait_: Option<Type>,
    /// Whether the `impl` is `unsafe`.
    pub is_unsafe: bool,
    /// Whether the `impl` is negative.
    pub is_negative: bool,
    /// The generic parameters of the `impl` block.
    pub generics: GenericParams,
    /// The items within the `impl` block.
    pub items: ThinVec<ImplItem>,
    /// Metadata about the `impl` block, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl From<TraitItemFn> for TraitItem {
    /// Converts a `TraitItemFn` into a `TraitItem::Fn` variant.
    fn from(item: TraitItemFn) -> Self {
        TraitItem::Fn(item)
    }
}

impl From<AssociatedConst> for TraitItem {
    /// Converts an `AssociatedConst` into a `TraitItem::Const` variant.
    fn from(item: AssociatedConst) -> Self {
        TraitItem::Const(item)
    }
}

impl From<ItemImpl> for Item {
    /// Converts an `ItemImpl` into an `Item::Impl` variant.
    fn from(item: ItemImpl) -> Self {
        Item::Impl(item)
    }
}
