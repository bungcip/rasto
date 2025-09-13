//! Defines the AST nodes for top-level items in a Rust file.
//!
//! Items are the primary components of a Rust program, such as functions, structs, enums,
//! impl blocks, and traits. They are the top-level declarations that make up a crate.

use crate::ast::generics::GenericParams;
use crate::ast::item_const::ItemConst;
use crate::ast::item_extern_crate::ItemExternCrate;
use crate::ast::item_foreign_mod::ItemForeignMod;
use crate::ast::item_macro::ItemMacro;
use crate::ast::item_mod::ItemMod;
use crate::ast::item_static::ItemStatic;
use crate::ast::item_trait_alias::ItemTraitAlias;
use crate::ast::item_type::ItemType;
use crate::ast::item_union::ItemUnion;
use crate::ast::item_use::ItemUse;
use crate::ast::metadata::Md;
use crate::ast::patterns::Pat;
use crate::ast::statements::Block;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use thin_vec::ThinVec;
use std::fmt;

/// A top-level item in a Rust file.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
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
    /// A `const` item: `const FOO: u32 = 42;`.
    Const(ItemConst),
    /// An `extern crate` item: `extern crate semver;`.
    ExternCrate(ItemExternCrate),
    /// A foreign module: `extern "C" { ... }`.
    ForeignMod(ItemForeignMod),
    /// A macro definition: `macro_rules! ...`.
    Macro(ItemMacro),
    /// A module: `mod foo { ... }`.
    Mod(ItemMod),
    /// A `static` item: `static FOO: u32 = 42;`.
    Static(ItemStatic),
    /// A trait alias: `trait Foo = Bar;`.
    TraitAlias(ItemTraitAlias),
    /// A type alias: `type Foo = Bar;`.
    Type(ItemType),
    /// A `union` item: `union Foo { ... }`.
    Union(ItemUnion),
    /// A `use` item: `use std::collections::HashMap;`.
    Use(ItemUse),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

/// A trait item: `trait Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemTrait {
    /// The name of the trait.
    pub ident: String,
    /// The generic parameters of the trait.
    pub generics: GenericParams,
    /// The items within the trait.
    pub items: ThinVec<TraitItem>,
    /// Metadata about the trait.
    pub md: Option<Box<Md>>,
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
    /// Metadata about the function.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl fmt::Display for ItemTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

/// A struct item: `struct Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStruct {
    /// The name of the struct.
    pub ident: String,
    /// The generic parameters of the struct.
    pub generics: GenericParams,
    /// The fields of the struct.
    pub fields: ThinVec<Field>,
    /// Metadata about the struct.
    pub md: Option<Box<Md>>,
}

/// A field of a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub ident: String,
    /// The type of the field.
    pub ty: Type,
    /// Metadata about the field.
    pub md: Option<Box<Md>>,
}

/// An enum item: `enum Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemEnum {
    /// The name of the enum.
    pub ident: String,
    /// The generic parameters of the enum.
    pub generics: GenericParams,
    /// The variants of the enum.
    pub variants: ThinVec<Variant>,
    /// Metadata about the enum.
    pub md: Option<Box<Md>>,
}

/// A variant of an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The name of the variant.
    pub ident: String,
    /// Metadata about the variant.
    pub md: Option<Box<Md>>,
}

/// An `impl` block: `impl Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemImpl {
    /// The type the `impl` block is for.
    pub ty: Type,
    /// The functions within the `impl` block.
    pub fns: ThinVec<ItemFn>,
    /// Metadata about the `impl` block.
    pub md: Option<Box<Md>>,
}

/// A function item: `fn foo() { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemFn {
    /// The function signature.
    pub sig: Signature,
    /// The function body.
    pub block: Block,
    /// Metadata about the function.
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

impl From<ItemFn> for Item {
    /// Converts an `ItemFn` into an `Item`.
    fn from(item: ItemFn) -> Self {
        Item::Fn(item)
    }
}

impl From<ItemStruct> for Item {
    /// Converts an `ItemStruct` into an `Item`.
    fn from(item: ItemStruct) -> Self {
        Item::Struct(item)
    }
}

impl From<ItemEnum> for Item {
    /// Converts an `ItemEnum` into an `Item`.
    fn from(item: ItemEnum) -> Self {
        Item::Enum(item)
    }
}

impl From<ItemImpl> for Item {
    /// Converts an `ItemImpl` into an `Item`.
    fn from(item: ItemImpl) -> Self {
        Item::Impl(item)
    }
}

impl From<ItemTrait> for Item {
    /// Converts an `ItemTrait` into an `Item`.
    fn from(item: ItemTrait) -> Self {
        Item::Trait(item)
    }
}

impl From<ItemConst> for Item {
    fn from(item: ItemConst) -> Self {
        Item::Const(item)
    }
}

impl From<ItemExternCrate> for Item {
    fn from(item: ItemExternCrate) -> Self {
        Item::ExternCrate(item)
    }
}

impl From<ItemForeignMod> for Item {
    fn from(item: ItemForeignMod) -> Self {
        Item::ForeignMod(item)
    }
}

impl From<ItemMacro> for Item {
    fn from(item: ItemMacro) -> Self {
        Item::Macro(item)
    }
}

impl From<ItemMod> for Item {
    fn from(item: ItemMod) -> Self {
        Item::Mod(item)
    }
}

impl From<ItemStatic> for Item {
    fn from(item: ItemStatic) -> Self {
        Item::Static(item)
    }
}

impl From<ItemTraitAlias> for Item {
    fn from(item: ItemTraitAlias) -> Self {
        Item::TraitAlias(item)
    }
}

impl From<ItemType> for Item {
    fn from(item: ItemType) -> Self {
        Item::Type(item)
    }
}

impl From<ItemUnion> for Item {
    fn from(item: ItemUnion) -> Self {
        Item::Union(item)
    }
}

impl From<ItemUse> for Item {
    fn from(item: ItemUse) -> Self {
        Item::Use(item)
    }
}
