//! Defines the AST nodes for top-level items in a Rust file.
//!
//! Items are the primary components of a Rust program, such as functions, structs, enums,
//! impl blocks, and traits. They are the top-level declarations that make up a crate.

use crate::ast::comments::Comment;
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
use crate::ast::statements::Block;
use crate::pretty_printer_v2::{Printer, PrettyPrintV2};

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
    #[allow(missing_docs)]
    Const(ItemConst),
    #[allow(missing_docs)]
    ExternCrate(ItemExternCrate),
    #[allow(missing_docs)]
    ForeignMod(ItemForeignMod),
    #[allow(missing_docs)]
    Macro(ItemMacro),
    #[allow(missing_docs)]
    Mod(ItemMod),
    #[allow(missing_docs)]
    Static(ItemStatic),
    #[allow(missing_docs)]
    TraitAlias(ItemTraitAlias),
    #[allow(missing_docs)]
    Type(ItemType),
    #[allow(missing_docs)]
    Union(ItemUnion),
    #[allow(missing_docs)]
    Use(ItemUse),
}

/// A trait item: `trait Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemTrait {
    /// Comments that appear before the trait.
    pub leading_comments: Vec<Comment>,
    /// The name of the trait.
    pub ident: String,
    /// The items within the trait.
    pub items: Vec<TraitItem>,
    /// Comments that appear after the trait.
    pub trailing_comments: Vec<Comment>,
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
    /// Comments that appear before the function.
    pub leading_comments: Vec<Comment>,
    /// The function signature.
    pub sig: Signature,
    /// An optional default implementation of the function.
    pub block: Option<Block>,
    /// Comments that appear after the function.
    pub trailing_comments: Vec<Comment>,
}

impl ItemFn {
    /// Pretty-prints the function to a string.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted function.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl ItemStruct {
    /// Pretty-prints the struct to a string.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted struct.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl ItemEnum {
    /// Pretty-prints the enum to a string.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted enum.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl ItemImpl {
    /// Pretty-prints the `impl` block to a string.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted `impl` block.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl ItemTrait {
    /// Pretty-prints the trait to a string.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted trait.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

/// A struct item: `struct Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStruct {
    /// Comments that appear before the struct.
    pub leading_comments: Vec<Comment>,
    /// The name of the struct.
    pub ident: String,
    /// The fields of the struct.
    pub fields: Vec<Field>,
    /// Comments that appear after the struct.
    pub trailing_comments: Vec<Comment>,
}

/// A field of a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub ident: String,
    /// The type of the field.
    pub ty: String,
}

/// An enum item: `enum Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemEnum {
    /// Comments that appear before the enum.
    pub leading_comments: Vec<Comment>,
    /// The name of the enum.
    pub ident: String,
    /// The variants of the enum.
    pub variants: Vec<Variant>,
    /// Comments that appear after the enum.
    pub trailing_comments: Vec<Comment>,
}

/// A variant of an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// The name of the variant.
    pub ident: String,
}

/// An `impl` block: `impl Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemImpl {
    /// Comments that appear before the `impl` block.
    pub leading_comments: Vec<Comment>,
    /// The type the `impl` block is for.
    pub ident: String,
    /// The functions within the `impl` block.
    pub fns: Vec<ItemFn>,
    /// Comments that appear after the `impl` block.
    pub trailing_comments: Vec<Comment>,
}

/// A function item: `fn foo() { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemFn {
    /// Comments that appear before the function.
    pub leading_comments: Vec<Comment>,
    /// The function signature.
    pub sig: Signature,
    /// The function body.
    pub block: Block,
    /// Comments that appear after the function.
    pub trailing_comments: Vec<Comment>,
}

/// A function signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    // The `fn` token would go here.
    /// The name of the function.
    pub ident: String,
    // For simplicity, we'll omit arguments and return type for now.
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
