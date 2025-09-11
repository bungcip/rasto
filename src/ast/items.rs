use crate::ast::comments::Comment;
use crate::ast::statements::Block;
use crate::pretty_printer_v2::{Printer, PrettyPrintV2};

/// A top-level item in a Rust file.
#[derive(Debug, Clone)]
pub enum Item {
    /// A function item: `fn foo() { ... }`
    Fn(ItemFn),
    /// A struct item: `struct Foo { ... }`
    Struct(ItemStruct),
    /// An enum item: `enum Foo { ... }`
    Enum(ItemEnum),
    /// An impl block: `impl Foo { ... }`
    Impl(ItemImpl),
    /// A trait item: `trait Foo { ... }`
    Trait(ItemTrait),
}

/// A trait item: `trait Foo { ... }`
#[derive(Debug, Clone)]
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

/// An item within a trait.
#[derive(Debug, Clone)]
pub enum TraitItem {
    /// A function item: `fn foo();`
    Fn(TraitItemFn),
}

/// A function item within a trait.
#[derive(Debug, Clone)]
pub struct TraitItemFn {
    /// Comments that appear before the function.
    pub leading_comments: Vec<Comment>,
    /// The function signature.
    pub sig: Signature,
    /// An optional function body.
    pub block: Option<Block>,
    /// Comments that appear after the function.
    pub trailing_comments: Vec<Comment>,
}

impl ItemFn {
    /// Pretty-prints the function to a string.
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
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl ItemImpl {
    /// Pretty-prints the impl to a string.
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
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

/// A struct item: `struct Foo { ... }`
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Field {
    /// The name of the field.
    pub ident: String,
    /// The type of the field.
    pub ty: String,
}

/// An enum item: `enum Foo { ... }`
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Variant {
    /// The name of the variant.
    pub ident: String,
}

/// An impl block: `impl Foo { ... }`
#[derive(Debug, Clone)]
pub struct ItemImpl {
    /// Comments that appear before the impl block.
    pub leading_comments: Vec<Comment>,
    /// The type the impl block is for.
    pub ident: String,
    /// The functions within the impl block.
    pub fns: Vec<ItemFn>,
    /// Comments that appear after the impl block.
    pub trailing_comments: Vec<Comment>,
}

/// A function item: `fn foo() { ... }`
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Signature {
    // The `fn` token would go here.
    /// The name of the function.
    pub ident: String,
    // For simplicity, we'll omit arguments and return type for now.
}

impl From<ItemFn> for Item {
    fn from(item: ItemFn) -> Self {
        Item::Fn(item)
    }
}

impl From<ItemStruct> for Item {
    fn from(item: ItemStruct) -> Self {
        Item::Struct(item)
    }
}

impl From<ItemEnum> for Item {
    fn from(item: ItemEnum) -> Self {
        Item::Enum(item)
    }
}

impl From<ItemImpl> for Item {
    fn from(item: ItemImpl) -> Self {
        Item::Impl(item)
    }
}

impl From<ItemTrait> for Item {
    fn from(item: ItemTrait) -> Self {
        Item::Trait(item)
    }
}
