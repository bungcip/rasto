//! Defines the AST nodes for top-level items in a Rust file.
//!
//! Items are the primary components of a Rust program, such as functions, structs, enums,
//! impl blocks, and traits. They are the top-level declarations that make up a crate.

use crate::ast::generics::GenericParams;
use crate::ast::item_asm::ItemAsm;
use crate::ast::item_def::ItemDef;
use crate::ast::item_extern_crate::ItemExternCrate;
use crate::ast::item_foreign_mod::ItemForeignMod;
use crate::ast::item_macro::ItemMacro;
use crate::ast::item_mod::ItemMod;
use crate::ast::item_trait::ItemTrait;
use crate::ast::item_trait_alias::ItemTraitAlias;
use crate::ast::item_union::ItemUnion;
use crate::ast::item_use::ItemUse;
use crate::ast::metadata::Md;
use crate::ast::patterns::Pat;
use crate::ast::statements::Block;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A top-level item in a Rust file.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// An `asm!` block.
    Asm(ItemAsm),
    /// A function item: `fn foo() { ... }`.
    Fn(ItemFn),
    /// A struct item: `struct Foo { ... }`.
    Struct(ItemStruct),
    /// An enum item: `enum Foo { ... }`.
    Enum(ItemEnum),
    /// An `impl` block: `impl Foo { ... }`.
    Impl(ItemImpl),
    /// A trait item: `trait Foo { ... }`.
    Trait(ItemTrait),
    /// A `const`, `static`, or `type` item.
    Def(ItemDef),
    /// An `extern crate` item: `extern crate semver;`.
    ExternCrate(ItemExternCrate),
    /// A foreign module: `extern "C" { ... }`.
    ForeignMod(ItemForeignMod),
    /// A macro definition: `macro_rules! ...`.
    Macro(ItemMacro),
    /// A module: `mod foo { ... }`.
    Mod(ItemMod),
    /// A trait alias: `trait Foo = Bar;`.
    TraitAlias(ItemTraitAlias),
    /// A `union` item: `union Foo { ... }`.
    Union(ItemUnion),
    /// A `use` item: `use std::collections::HashMap;`.
    Use(ItemUse),
}

impl fmt::Display for Item {
    /// Formats the `Item` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

/// An item within a trait definition.
#[derive(Debug, Clone, PartialEq)]
pub enum TraitItem {
    /// A function item within a trait: `fn foo();`.
    Fn(TraitItemFn),
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

impl fmt::Display for ItemFn {
    /// Formats the `ItemFn` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemStruct {
    /// Formats the `ItemStruct` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemEnum {
    /// Formats the `ItemEnum` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemImpl {
    /// Formats the `ItemImpl` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemTrait {
    /// Formats the `ItemTrait` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

/// A struct definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStruct {
    /// The name of the struct.
    pub ident: String,
    /// The generic parameters of the struct.
    pub generics: GenericParams,
    /// The fields of the struct.
    pub fields: ThinVec<Field>,
    /// Metadata about the struct, including attributes and comments.
    pub md: Option<Box<Md>>,
}

/// A field of a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub ident: String,
    /// The type of the field.
    pub ty: Type,
    /// Metadata about the field, including attributes and comments.
    pub md: Option<Box<Md>>,
}

/// An enum definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemEnum {
    /// The name of the enum.
    pub ident: String,
    /// The generic parameters of the enum.
    pub generics: GenericParams,
    /// The variants of the enum.
    pub variants: ThinVec<Variant>,
    /// Metadata about the enum, including attributes and comments.
    pub md: Option<Box<Md>>,
}

/// A variant of an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The name of the variant.
    pub ident: String,
    /// Metadata about the variant, including attributes and comments.
    pub md: Option<Box<Md>>,
}

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
    /// The functions within the `impl` block.
    pub fns: ThinVec<ItemFn>,
    /// Metadata about the `impl` block, including attributes and comments.
    pub md: Option<Box<Md>>,
}

/// A function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemFn {
    /// The function signature.
    pub sig: Signature,
    /// The function body.
    pub block: Block,
    /// Metadata about the function, including attributes and comments.
    pub md: Option<Box<Md>>,
}

/// A function signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    // The `fn` token would go here.
    /// The name of the function.
    pub ident: String,
    /// The generic parameters of the function.
    pub generics: GenericParams,
    /// The arguments of the function.
    pub inputs: ThinVec<Pat>,
    /// The return type of the function.
    pub output: Option<Type>,
}

impl From<ItemAsm> for Item {
    /// Converts an `ItemAsm` into an `Item::Asm` variant.
    fn from(item: ItemAsm) -> Self {
        Item::Asm(item)
    }
}

impl From<ItemFn> for Item {
    /// Converts an `ItemFn` into an `Item::Fn` variant.
    fn from(item: ItemFn) -> Self {
        Item::Fn(item)
    }
}

impl From<ItemStruct> for Item {
    /// Converts an `ItemStruct` into an `Item::Struct` variant.
    fn from(item: ItemStruct) -> Self {
        Item::Struct(item)
    }
}

impl From<ItemEnum> for Item {
    /// Converts an `ItemEnum` into an `Item::Enum` variant.
    fn from(item: ItemEnum) -> Self {
        Item::Enum(item)
    }
}

impl From<ItemImpl> for Item {
    /// Converts an `ItemImpl` into an `Item::Impl` variant.
    fn from(item: ItemImpl) -> Self {
        Item::Impl(item)
    }
}

impl From<ItemTrait> for Item {
    /// Converts an `ItemTrait` into an `Item::Trait` variant.
    fn from(item: ItemTrait) -> Self {
        Item::Trait(item)
    }
}

impl From<ItemDef> for Item {
    /// Converts an `ItemDef` into an `Item::Def` variant.
    fn from(item: ItemDef) -> Self {
        Item::Def(item)
    }
}

impl From<ItemExternCrate> for Item {
    /// Converts an `ItemExternCrate` into an `Item::ExternCrate` variant.
    fn from(item: ItemExternCrate) -> Self {
        Item::ExternCrate(item)
    }
}

impl From<ItemForeignMod> for Item {
    /// Converts an `ItemForeignMod` into an `Item::ForeignMod` variant.
    fn from(item: ItemForeignMod) -> Self {
        Item::ForeignMod(item)
    }
}

impl From<ItemMacro> for Item {
    /// Converts an `ItemMacro` into an `Item::Macro` variant.
    fn from(item: ItemMacro) -> Self {
        Item::Macro(item)
    }
}

impl From<ItemMod> for Item {
    /// Converts an `ItemMod` into an `Item::Mod` variant.
    fn from(item: ItemMod) -> Self {
        Item::Mod(item)
    }
}

impl From<ItemTraitAlias> for Item {
    /// Converts an `ItemTraitAlias` into an `Item::TraitAlias` variant.
    fn from(item: ItemTraitAlias) -> Self {
        Item::TraitAlias(item)
    }
}

impl From<ItemUnion> for Item {
    /// Converts an `ItemUnion` into an `Item::Union` variant.
    fn from(item: ItemUnion) -> Self {
        Item::Union(item)
    }
}

impl From<ItemUse> for Item {
    /// Converts an `ItemUse` into an `Item::Use` variant.
    fn from(item: ItemUse) -> Self {
        Item::Use(item)
    }
}
