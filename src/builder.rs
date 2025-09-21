//! Provides a builder API for constructing AST nodes programmatically.
//!
//! The builder pattern allows for a more fluent and readable way of creating complex AST structures.
//! Each builder corresponds to a specific AST node and provides methods for setting its properties.
//!
//! # Examples
//!
//! ```
//! use rasto::builder::*;
//! use rasto::ast::{*, Lit, LitInt};
//! use thin_vec::thin_vec;
//!
//! let file_ast = file()
//!     .item(
//!         fn_def("my_function")
//!             .block(
//!                 block()
//!                     .statement(expr().lit(42))
//!             )
//!             .build(),
//!     )
//!     .build();
//! ```

use crate::ast::item_const::ItemConst;
use crate::ast::item_extern_type::ItemExternType;
use crate::ast::item_type_alias::ItemTypeAlias;
use crate::ast::items::*;
use crate::ast::*;
use std::convert::Into;
use thin_vec::{ThinVec, thin_vec};

/// Creates a new `FileBuilder` to construct a `File` AST node.
///
/// # Returns
///
/// A `FileBuilder` instance.
pub fn file() -> FileBuilder {
    FileBuilder::new()
}

/// A builder for constructing a `File` AST node.
#[derive(Default)]
pub struct FileBuilder {
    items: ThinVec<Item>,
    md: MdBuilder,
}

impl FileBuilder {
    /// Creates a new, empty `FileBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an item to the file.
    ///
    /// # Parameters
    ///
    /// - `item`: The `Item` to add to the file.
    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds a comment to the file.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the file.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `File` AST node.
    ///
    /// # Returns
    ///
    /// A `File` instance.
    pub fn build(self) -> File {
        File {
            items: self.items,
            md: if self.md.is_empty() {
                None
            } else {
                Some(Box::new(self.md.build()))
            },
        }
    }
}

/// Creates a new `ItemConstBuilder` to construct a `const` item.
pub fn const_def(
    name: impl Into<Ident>,
    ty: impl Into<Type>,
    expr: impl Into<Expr>,
) -> ItemConstBuilder {
    ItemConstBuilder::new(name, ty, expr)
}

/// A builder for constructing an `ItemConst` AST node.
pub struct ItemConstBuilder {
    ident: Ident,
    vis: Visibility,
    ty: Type,
    expr: Box<Expr>,
    md: MdBuilder,
}

impl ItemConstBuilder {
    /// Creates a new `ItemConstBuilder` with the given name, type and expression.
    pub fn new(name: impl Into<Ident>, ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            ty: ty.into(),
            expr: Box::new(expr.into()),
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the const item.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a comment to the const item.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the const item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemConst` AST node.
    pub fn build(self) -> ItemConst {
        ItemConst {
            vis: self.vis,
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            md: Some(Box::new(self.md.build())),
        }
    }
}

impl From<ItemConstBuilder> for Item {
    /// Converts an `ItemConstBuilder` into an `Item::Const` variant.
    fn from(builder: ItemConstBuilder) -> Self {
        Item::Const(builder.build())
    }
}

/// Creates a new `ItemTypeAliasBuilder` to construct a `type` alias item.
pub fn type_alias(name: impl Into<Ident>, ty: impl Into<Type>) -> ItemTypeAliasBuilder {
    ItemTypeAliasBuilder::new(name, ty)
}

/// A builder for constructing an `ItemTypeAlias` AST node.
pub struct ItemTypeAliasBuilder {
    ident: Ident,
    vis: Visibility,
    generics: GenericParams,
    ty: Type,
    md: MdBuilder,
}

impl ItemTypeAliasBuilder {
    /// Creates a new `ItemTypeAliasBuilder` with the given name and type.
    pub fn new(name: impl Into<Ident>, ty: impl Into<Type>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            generics: GenericParams::new(),
            ty: ty.into(),
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the type alias.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a generic parameter to the type alias.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Adds a comment to the type alias.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the type alias.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemTypeAlias` AST node.
    pub fn build(self) -> ItemTypeAlias {
        ItemTypeAlias {
            vis: self.vis,
            ident: self.ident,
            generics: self.generics,
            ty: self.ty,
            md: Some(Box::new(self.md.build())),
        }
    }
}

impl From<ItemTypeAliasBuilder> for Item {
    /// Converts an `ItemTypeAliasBuilder` into an `Item::TypeAlias` variant.
    fn from(builder: ItemTypeAliasBuilder) -> Self {
        Item::TypeAlias(builder.build())
    }
}

/// Creates a new `CommentBuilder` to construct a comment.
pub fn comment() -> CommentBuilder {
    CommentBuilder
}

/// A builder for constructing `Comment` AST nodes.
#[derive(Clone, Copy)]
pub struct CommentBuilder;

impl CommentBuilder {
    /// Creates a line comment, e.g., `// A line comment.`
    ///
    /// # Parameters
    ///
    /// - `content`: The text of the comment.
    pub fn line<S: Into<String>>(self, content: S) -> Comment {
        Comment::Line(content.into())
    }

    /// Creates a doc comment, e.g., `/// A doc comment.`
    ///
    /// # Parameters
    ///
    /// - `content`: The text of the comment.
    pub fn doc<S: Into<String>>(self, content: S) -> Comment {
        Comment::Doc(content.into())
    }
}

/// Creates a new `TraitBuilder` to construct a trait definition.
///
/// # Parameters
///
/// - `name`: The name of the trait.
///
/// # Returns
///
/// A `TraitBuilder` instance.
pub fn trait_def(name: impl Into<Ident>) -> TraitBuilder {
    TraitBuilder::new(name)
}

/// A builder for constructing an `ItemTrait` (trait definition) AST node.
pub struct TraitBuilder {
    ident: Ident,
    vis: Visibility,
    generics: GenericParams,
    associated_types: ThinVec<AssociatedType>,
    items: ThinVec<TraitItem>,
    md: MdBuilder,
}

impl TraitBuilder {
    /// Creates a new `TraitBuilder` with the given trait name.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the trait.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            generics: GenericParams::new(),
            associated_types: thin_vec![],
            items: thin_vec![],
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the trait.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a generic parameter to the trait.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Adds an item to the trait.
    ///
    /// # Parameters
    ///
    /// - `item`: The item to add.
    pub fn item(mut self, item: impl Into<TraitItem>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds an associated type to the trait.
    ///
    /// # Parameters
    ///
    /// - `associated_type`: The associated type to add.
    pub fn associated_type(mut self, associated_type: impl Into<AssociatedType>) -> Self {
        self.associated_types.push(associated_type.into());
        self
    }

    /// Adds a comment to the trait.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the trait.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemTrait` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemTrait` instance.
    pub fn build(self) -> ItemTrait {
        ItemTrait {
            vis: self.vis,
            ident: self.ident,
            generics: self.generics,
            associated_types: self.associated_types,
            items: self.items,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// A builder for constructing an `ItemExternBlock` AST node.
#[derive(Default)]
pub struct ItemExternBlockBuilder {
    is_unsafe: bool,
    abi: Option<String>,
    items: ThinVec<ExternalItem>,
    md: MdBuilder,
}

impl ItemExternBlockBuilder {
    /// Creates a new `ItemExternBlockBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `extern` block as `unsafe`.
    pub fn unsafe_(mut self) -> Self {
        self.is_unsafe = true;
        self
    }

    /// Sets the ABI of the `extern` block.
    ///
    /// # Parameters
    ///
    /// - `abi`: The ABI string (e.g., "C").
    pub fn abi(mut self, abi: impl Into<String>) -> Self {
        self.abi = Some(abi.into());
        self
    }

    /// Adds an item to the `extern` block.
    ///
    /// # Parameters
    ///
    /// - `item`: The `ExternalItem` to add.
    pub fn item(mut self, item: impl Into<ExternalItem>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds a comment to the `extern` block.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the `extern` block.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemExternBlock` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemExternBlock` instance.
    pub fn build(self) -> ItemExternBlock {
        ItemExternBlock {
            vis: Visibility::Default,
            is_unsafe: self.is_unsafe,
            abi: self.abi,
            items: self.items,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `AssociatedConstBuilder` to construct an associated constant.
pub fn associated_const(ident: impl Into<Ident>, ty: impl Into<Type>) -> AssociatedConstBuilder {
    AssociatedConstBuilder::new(ident, ty)
}

/// A builder for constructing an `AssociatedConst` AST node.
pub struct AssociatedConstBuilder {
    ident: Ident,
    ty: Type,
    expr: Option<Box<Expr>>,
    md: MdBuilder,
}

impl AssociatedConstBuilder {
    /// Create a new `AssociatedConstBuilder` with the provided identifier.
    ///
    /// # Parameters
    ///
    /// - `ident`: Name of the associated constant.
    /// - `ty`: Type of the associated constant.
    pub fn new(ident: impl Into<Ident>, ty: impl Into<Type>) -> Self {
        Self {
            ident: ident.into(),
            ty: ty.into(),
            expr: None,
            md: MdBuilder::new(),
        }
    }

    /// Sets the expression of the associated const.
    ///
    /// # Parameters
    ///
    /// - `expr`: The `Expr` of the associated const.
    pub fn expr(mut self, expr: impl Into<Expr>) -> Self {
        self.expr = Some(Box::new(expr.into()));
        self
    }

    /// Builds the `AssociatedConst` instance.
    ///
    /// # Returns
    ///
    /// An `AssociatedConst` instance.
    pub fn build(self) -> AssociatedConst {
        AssociatedConst {
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// A builder for constructing an `Arm` AST node.
pub struct ArmBuilder {
    pat: Pat,
    guard: Option<Expr>,
    body: Expr,
}

impl ArmBuilder {
    /// Creates a new `ArmBuilder` with the given pattern.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern for the arm.
    pub fn new(pat: impl Into<Pat>) -> Self {
        Self {
            pat: pat.into(),
            guard: None,
            body: expr().tuple(vec![]),
        }
    }

    /// Sets the guard expression for the arm.
    ///
    /// # Parameters
    ///
    /// - `guard`: The expression for the guard.
    pub fn guard(mut self, guard: impl Into<Expr>) -> Self {
        self.guard = Some(guard.into());
        self
    }

    /// Sets the body of the arm.
    ///
    /// # Parameters
    ///
    /// - `body`: The expression for the body.
    pub fn body(mut self, body: impl Into<Expr>) -> Self {
        self.body = body.into();
        self
    }

    /// Builds the `Arm` AST node.
    ///
    /// # Returns
    ///
    /// An `Arm` instance.
    pub fn build(self) -> Arm {
        Arm {
            pat: self.pat,
            guard: self.guard.map(Box::new),
            body: Box::new(self.body),
        }
    }
}

/// Creates a new `AssociatedTypeBuilder` to construct an associated type for traits.
pub fn associated_type(ident: impl Into<Ident>) -> AssociatedTypeBuilder {
    AssociatedTypeBuilder::new(ident)
}

/// A builder for constructing an `AssociatedType` AST node.
pub struct AssociatedTypeBuilder {
    ident: Ident,
    generics: GenericParams,
    bounds: ThinVec<Type>,
    default: Option<Type>,
    md: Option<Box<Md>>,
}

impl AssociatedTypeBuilder {
    /// Create a new `AssociatedTypeBuilder` with the provided identifier.
    ///
    /// # Parameters
    ///
    /// - `ident`: Name of the associated type.
    pub fn new(ident: impl Into<Ident>) -> Self {
        Self {
            ident: ident.into(),
            generics: GenericParams::new(),
            bounds: thin_vec![],
            default: None,
            md: None,
        }
    }

    /// Adds a generic parameter to the associated type.
    ///
    /// # Parameters
    ///
    /// - `g`: The generic parameter to add.
    pub fn generic(mut self, g: impl Into<GenericParam>) -> Self {
        self.generics.params.push(g.into());
        self
    }

    /// Adds a bound to the associated type.
    ///
    /// # Parameters
    ///
    /// - `t`: The bound to add.
    pub fn bound(mut self, t: impl Into<Type>) -> Self {
        self.bounds.push(t.into());
        self
    }

    /// Sets the default type for the associated type.
    ///
    /// # Parameters
    ///
    /// - `t`: The default type.
    pub fn default(mut self, t: impl Into<Type>) -> Self {
        self.default = Some(t.into());
        self
    }

    /// Sets the metadata for the associated type.
    ///
    /// # Parameters
    ///
    /// - `md`: The metadata to set.
    pub fn md(mut self, md: impl Into<Md>) -> Self {
        self.md = Some(Box::new(md.into()));
        self
    }

    /// Builds the `AssociatedType` instance.
    ///
    /// # Returns
    ///
    /// An `AssociatedType` instance.
    pub fn build(self) -> AssociatedType {
        AssociatedType {
            ident: self.ident,
            generics: self.generics,
            bounds: self.bounds,
            default: self.default,
            md: self.md,
        }
    }
}

/// Creates a new `BlockBuilder` to construct a block of statements.
///
/// # Returns
///
/// A `BlockBuilder` instance.
pub fn block() -> BlockBuilder {
    BlockBuilder::new()
}

/// A builder for constructing a `Block` AST node.
pub struct BlockBuilder {
    stmts: ThinVec<Stmt>,
    has_trailing_semicolon: bool,
    comments: ThinVec<Comment>,
    trailing_comments: ThinVec<Comment>,
}

impl BlockBuilder {
    /// Creates a new, empty `BlockBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a comment to the block.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.comments.push(comment.into());
        self
    }

    /// Adds a statement to the block.
    ///
    /// # Parameters
    ///
    /// - `stmt`: The `Stmt` to add.
    pub fn statement(mut self, stmt: impl Into<Stmt>) -> Self {
        self.stmts.push(stmt.into());
        self
    }

    /// Sets whether the block has a trailing semicolon.
    ///
    /// # Parameters
    ///
    /// - `has_trailing_semicolon`: `true` if the block should have a trailing semicolon.
    pub fn has_trailing_semicolon(mut self, has_trailing_semicolon: bool) -> Self {
        self.has_trailing_semicolon = has_trailing_semicolon;
        self
    }

    /// Adds a trailing comment to the block.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    /// Builds the `Block` AST node.
    ///
    /// # Returns
    ///
    /// A `Block` instance.
    pub fn build(self) -> Block {
        let md = if !self.comments.is_empty() || !self.trailing_comments.is_empty() {
            let mut md_builder = MdBuilder::new();
            for comment in self.comments {
                md_builder = md_builder.comment(comment);
            }
            for comment in self.trailing_comments {
                md_builder = md_builder.trailing_comment(comment);
            }
            Some(Box::new(md_builder.build()))
        } else {
            None
        };

        Block {
            stmts: self.stmts,
            has_trailing_semicolon: self.has_trailing_semicolon,
            md,
        }
    }
}

impl Default for BlockBuilder {
    fn default() -> Self {
        Self {
            stmts: Default::default(),
            has_trailing_semicolon: true,
            comments: Default::default(),
            trailing_comments: Default::default(),
        }
    }
}

/// Creates a new `ImplBuilder` to construct an impl block.
///
/// # Parameters
///
/// - `ty`: The type the impl block is for.
///
/// # Returns
///
/// A `ImplBuilder` instance.
pub fn impl_block(ty: impl Into<Type>) -> ImplBuilder {
    ImplBuilder::new(ty)
}

/// A builder for constructing an `ItemImpl` (impl block) AST node.
pub struct ImplBuilder {
    generics: GenericParams,
    ty: Type,
    trait_: Option<Type>,
    is_unsafe: bool,
    is_negative: bool,
    items: ThinVec<ImplItem>,
}

impl ImplBuilder {
    /// Creates a new `ImplBuilder` with the given type.
    ///
    /// # Parameters
    ///
    /// - `ty`: The type the impl block is for.
    pub fn new(ty: impl Into<Type>) -> Self {
        Self {
            generics: GenericParams::new(),
            ty: ty.into(),
            trait_: None,
            is_unsafe: false,
            is_negative: false,
            items: thin_vec![],
        }
    }

    /// Adds a generic parameter to the impl block.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Sets the trait for the impl block.
    ///
    /// # Parameters
    ///
    /// - `trait_`: The trait to implement.
    pub fn trait_(mut self, trait_: impl Into<Type>) -> Self {
        self.trait_ = Some(trait_.into());
        self
    }

    /// Marks the impl block as `unsafe`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// unsafe impl MyTrait for MyType { ... }
    /// ```
    pub fn unsafe_(mut self) -> Self {
        self.is_unsafe = true;
        self
    }

    /// Marks the impl block as negative, e.g., `impl !MyTrait for MyType { ... }`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// impl !MyTrait for MyType { ... }
    /// ```
    pub fn negative(mut self) -> Self {
        self.is_negative = true;
        self
    }

    /// Adds an item to the impl block.
    ///
    /// # Parameters
    ///
    /// - `item`: The item to add.
    pub fn item(mut self, item: impl Into<ImplItem>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Builds the `ItemImpl` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemImpl` instance.
    pub fn build(self) -> ItemImpl {
        ItemImpl {
            generics: self.generics,
            ty: self.ty,
            trait_: self.trait_,
            is_unsafe: self.is_unsafe,
            is_negative: self.is_negative,
            items: self.items,
            md: None,
        }
    }
}

/// Creates a new `EnumBuilder` to construct an enum definition.
///
/// # Parameters
///
/// - `name`: The name of the enum.
///
/// # Returns
///
/// A `EnumBuilder` instance.
pub fn enum_def(name: impl Into<Ident>) -> EnumBuilder {
    EnumBuilder::new(name)
}

/// A builder for constructing an `ItemEnum` (enum definition) AST node.
pub struct EnumBuilder {
    ident: Ident,
    vis: Visibility,
    generics: GenericParams,
    variants: ThinVec<Variant>,
    md: MdBuilder,
}

impl EnumBuilder {
    /// Creates a new `EnumBuilder` with the given enum name.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the enum.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            generics: GenericParams::new(),
            variants: thin_vec![],
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the enum.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a generic parameter to the enum.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Adds a variant to the enum.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the variant.
    pub fn variant(mut self, name: impl Into<Ident>) -> Self {
        self.variants.push(Variant {
            ident: name.into(),
            md: None,
        });
        self
    }

    /// Adds a comment to the enum.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the enum.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemEnum` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemEnum` instance.
    pub fn build(self) -> ItemEnum {
        ItemEnum {
            vis: self.vis,
            ident: self.ident,
            generics: self.generics,
            variants: self.variants,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `StructBuilder` to construct a struct definition.
///
/// # Parameters
///
/// - `name`: The name of the struct.
///
/// # Returns
///
/// A `StructBuilder` instance.
pub fn struct_def(name: impl Into<Ident>) -> StructBuilder {
    StructBuilder::new(name)
}

/// A builder for constructing an `ItemStruct` (struct definition) AST node.
pub struct StructBuilder {
    ident: Ident,
    vis: Visibility,
    generics: GenericParams,
    fields: ThinVec<Field>,
    md: MdBuilder,
}

impl StructBuilder {
    /// Creates a new `StructBuilder` with the given struct name.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the struct.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            generics: GenericParams::new(),
            fields: thin_vec![],
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the struct.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a generic parameter to the struct.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Adds a field to the struct.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the field.
    /// - `ty`: The type of the field.
    pub fn field(mut self, name: impl Into<Ident>, ty: impl Into<Type>) -> Self {
        self.fields.push(Field {
            ident: name.into(),
            ty: ty.into(),
            md: None,
        });
        self
    }

    /// Adds a comment to the struct.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the struct.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemStruct` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemStruct` instance.
    pub fn build(self) -> ItemStruct {
        ItemStruct {
            vis: self.vis,
            ident: self.ident,
            generics: self.generics,
            fields: self.fields,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `SignatureBuilder` to construct a function signature.
///
/// # Parameters
///
/// - `name`: The name of the function.
///
/// # Returns
///
/// A `SignatureBuilder` instance.
pub fn signature(name: impl Into<Ident>) -> SignatureBuilder {
    SignatureBuilder::new(name)
}

/// A builder for constructing a `Signature` AST node.
#[derive(Default)]
pub struct SignatureBuilder {
    ident: Ident,
    is_const: bool,
    is_async: bool,
    is_unsafe: bool,
    abi: Option<Abi>,
    generics: GenericParams,
    inputs: ThinVec<Pat>,
    is_variadic: bool,
    output: Option<Type>,
    where_clause: Option<WhereClause>,
}

impl SignatureBuilder {
    /// Creates a new `SignatureBuilder` with the given function name.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the function.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            ..Default::default()
        }
    }

    /// Sets the function as `const`.
    pub fn const_(mut self) -> Self {
        self.is_const = true;
        self
    }

    /// Sets the function as `async`.
    pub fn async_(mut self) -> Self {
        self.is_async = true;
        self
    }

    /// Sets the function as `unsafe`.
    pub fn unsafe_(mut self) -> Self {
        self.is_unsafe = true;
        self
    }

    /// Sets the ABI of the function.
    pub fn abi(mut self, abi: Abi) -> Self {
        self.abi = Some(abi);
        self
    }

    /// Adds a generic parameter to the function.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Adds an input parameter to the function.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern for the input parameter.
    pub fn input(mut self, pat: impl Into<Pat>) -> Self {
        self.inputs.push(pat.into());
        self
    }

    /// Adds a typed input parameter to the function.
    ///
    /// This is a convenience method for creating a `Pat::Type` pattern.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the input parameter.
    /// - `ty`: The type of the input parameter.
    pub fn input_typed(mut self, name: impl Into<Ident>, ty: impl Into<Type>) -> Self {
        self.inputs.push(pat().type_(pat().ident(name), ty));
        self
    }

    /// Sets whether the function is variadic.
    pub fn variadic(mut self, is_variadic: bool) -> Self {
        self.is_variadic = is_variadic;
        self
    }

    /// Sets the return type of the function.
    ///
    /// # Parameters
    ///
    /// - `ty`: The return type.
    pub fn output(mut self, ty: impl Into<Type>) -> Self {
        self.output = Some(ty.into());
        self
    }

    /// Sets the `where` clause of the function.
    pub fn where_clause(mut self, where_clause: WhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }

    /// Builds the `Signature` AST node.
    ///
    /// # Returns
    ///
    /// An `Signature` instance.
    pub fn build(self) -> Signature {
        Signature {
            is_const: self.is_const,
            is_async: self.is_async,
            is_unsafe: self.is_unsafe,
            abi: self.abi,
            ident: self.ident,
            generics: self.generics,
            inputs: self.inputs,
            is_variadic: self.is_variadic,
            output: self.output,
            where_clause: self.where_clause,
        }
    }
}

/// Creates a new `FnBuilder` to construct a function definition.
///
/// # Parameters
///
/// - `name`: The name of the function.
///
/// # Returns
///
/// A `FnBuilder` instance.
pub fn fn_def(name: impl Into<Ident>) -> FnBuilder {
    FnBuilder::new(name)
}

/// A builder for constructing an `ItemFn` (function definition) AST node.
pub struct FnBuilder {
    vis: Visibility,
    sig: SignatureBuilder,
    block: Block,
    md: MdBuilder,
}

impl FnBuilder {
    /// Creates a new `FnBuilder` with the given function name.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the function.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            vis: Visibility::Default,
            sig: signature(name),
            block: Block::default(),
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the function.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Sets the function as `const`.
    pub fn const_(mut self) -> Self {
        self.sig = self.sig.const_();
        self
    }

    /// Sets the function as `async`.
    pub fn async_(mut self) -> Self {
        self.sig = self.sig.async_();
        self
    }

    /// Sets the function as `unsafe`.
    pub fn unsafe_(mut self) -> Self {
        self.sig = self.sig.unsafe_();
        self
    }

    /// Sets the ABI of the function.
    pub fn abi(mut self, abi: Abi) -> Self {
        self.sig = self.sig.abi(abi);
        self
    }

    /// Adds a generic parameter to the function.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.sig = self.sig.generic(param);
        self
    }

    /// Adds an input parameter to the function.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern for the input parameter.
    pub fn input(mut self, pat: impl Into<Pat>) -> Self {
        self.sig = self.sig.input(pat);
        self
    }

    /// Adds a typed input parameter to the function.
    ///
    /// This is a convenience method for creating a `Pat::Type` pattern.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the input parameter.
    /// - `ty`: The type of the input parameter.
    pub fn input_typed(mut self, name: impl Into<Ident>, ty: impl Into<Type>) -> Self {
        self.sig = self.sig.input_typed(name, ty);
        self
    }

    /// Sets whether the function is variadic.
    pub fn variadic(mut self, is_variadic: bool) -> Self {
        self.sig = self.sig.variadic(is_variadic);
        self
    }

    /// Sets the return type of the function.
    ///
    /// # Parameters
    ///
    /// - `ty`: The return type.
    pub fn output(mut self, ty: impl Into<Type>) -> Self {
        self.sig = self.sig.output(ty);
        self
    }

    /// Sets the `where` clause of the function.
    pub fn where_clause(mut self, where_clause: WhereClause) -> Self {
        self.sig = self.sig.where_clause(where_clause);
        self
    }

    /// Sets the block of statements for the function.
    ///
    /// # Parameters
    ///
    /// - `block`: The `Block` containing the function's body.
    pub fn block(mut self, block: BlockBuilder) -> Self {
        self.block = block.build();
        self
    }

    /// Sets whether the function's block has a trailing semicolon.
    /// By default, a function body does not have a trailing semicolon.
    pub fn has_trailing_semicolon(mut self, has_trailing_semicolon: bool) -> Self {
        self.block.has_trailing_semicolon = has_trailing_semicolon;
        self
    }

    /// Adds a statement to the function's block.
    ///
    /// # Parameters
    ///
    /// - `stmt`: The statement to add.
    pub fn statement(mut self, stmt: impl Into<Stmt>) -> Self {
        self.block.stmts.push(stmt.into());
        self
    }

    /// Adds an attribute to the function.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Adds a comment to the function.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Builds the `ItemFn` AST node.
    ///
    /// # Panics
    ///
    /// Panics if the block has not been set.
    ///
    /// # Returns
    ///
    /// An `ItemFn` instance.
    pub fn build(self) -> ItemFn {
        ItemFn {
            vis: self.vis,
            sig: self.sig.build(),
            block: self.block,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `StmtBuilder` to construct statements.
pub fn stmt() -> StmtBuilder {
    StmtBuilder
}

/// A builder for constructing `Stmt` AST nodes.
#[derive(Clone, Copy)]
pub struct StmtBuilder;

impl StmtBuilder {
    /// Creates a local (`let`) binding statement.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern for the `let` binding.
    pub fn local(self, pat: impl Into<Pat>) -> LocalBuilder {
        LocalBuilder::new(pat)
    }

    /// Creates an item statement.
    ///
    /// # Parameters
    ///
    /// - `item`: The `Item` to be used as a statement.
    pub fn item(self, item: impl Into<Item>) -> Stmt {
        Stmt::Item(item.into())
    }

    /// Creates an expression statement.
    ///
    /// # Parameters
    ///
    /// - `expr`: The `Expr` to be used as a statement.
    pub fn expr(self, expr: Expr) -> Stmt {
        Stmt::Expr(expr)
    }

    /// Creates a macro call statement.
    ///
    /// # Parameters
    ///
    /// - `mac`: The `ExprMacroCall` to be used as a statement.
    pub fn mac_call(self, mac: ExprMacroCall) -> Stmt {
        Stmt::Expr(Expr::MacroCall(mac))
    }
}

/// A builder for constructing a `Local` (let) AST node.
pub struct LocalBuilder {
    pat: Pat,
    ty: Option<Type>,
    expr: Option<Expr>,
    else_block: Option<Block>,
}

impl LocalBuilder {
    /// Creates a new `LocalBuilder` with the given pattern.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern for the `let` binding.
    pub fn new(pat: impl Into<Pat>) -> Self {
        Self {
            pat: pat.into(),
            ty: None,
            expr: None,
            else_block: None,
        }
    }

    /// Sets the type of the variable.
    ///
    /// # Parameters
    ///
    /// - `ty`: The `Type` of the variable.
    pub fn ty(mut self, ty: impl Into<Type>) -> Self {
        self.ty = Some(ty.into());
        self
    }

    /// Sets the expression to initialize the variable.
    ///
    /// # Parameters
    ///
    /// - `expr`: The initialization `Expr`.
    pub fn expr(mut self, expr: impl Into<Expr>) -> Self {
        self.expr = Some(expr.into());
        self
    }

    /// Sets the `else` block for a `let-else` statement.
    ///
    /// # Parameters
    ///
    /// - `block`: The `Block` to be executed if the pattern does not match.
    pub fn else_block(mut self, block: BlockBuilder) -> Self {
        self.else_block = Some(block.build());
        self
    }

    /// Builds the `Stmt::Local` AST node.
    ///
    /// # Panics
    ///
    /// Panics if an `else` block is provided without an initializer expression.
    ///
    /// # Returns
    ///
    /// A `Stmt` instance representing the `let` binding.
    pub fn build(self) -> Stmt {
        if self.else_block.is_some() && self.expr.is_none() {
            panic!("let-else statements must have an initializer expression");
        }
        Stmt::Local(Local {
            pat: self.pat,
            ty: self.ty,
            expr: self.expr,
            else_block: self.else_block.map(Box::new),
        })
    }
}

/// Creates a new `FieldValueBuilder` to construct a field-value pair.
pub fn field_value(member: impl Into<Ident>, value: impl Into<Expr>) -> FieldValue {
    FieldValue {
        member: member.into(),
        value: value.into(),
    }
}

/// Creates a new `TraitItemFnBuilder` to construct a trait item function.
pub fn trait_item_fn(name: impl Into<Ident>) -> TraitItemFnBuilder {
    TraitItemFnBuilder::new(name)
}

/// A builder for constructing a `TraitItemFn` AST node.
pub struct TraitItemFnBuilder {
    sig: SignatureBuilder,
    block: Option<Block>,
    md: MdBuilder,
}

impl TraitItemFnBuilder {
    /// Creates a new `TraitItemFnBuilder` with the given function name.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            sig: signature(name),
            block: None,
            md: MdBuilder::new(),
        }
    }

    /// Sets the function as `const`.
    pub fn const_(mut self) -> Self {
        self.sig = self.sig.const_();
        self
    }

    /// Sets the function as `async`.
    pub fn async_(mut self) -> Self {
        self.sig = self.sig.async_();
        self
    }

    /// Sets the function as `unsafe`.
    pub fn unsafe_(mut self) -> Self {
        self.sig = self.sig.unsafe_();
        self
    }

    /// Sets the ABI of the function.
    pub fn abi(mut self, abi: Abi) -> Self {
        self.sig = self.sig.abi(abi);
        self
    }

    /// Adds a generic parameter to the function.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.sig = self.sig.generic(param);
        self
    }

    /// Adds an input parameter to the function.
    pub fn input(mut self, pat: impl Into<Pat>) -> Self {
        self.sig = self.sig.input(pat);
        self
    }

    /// Adds a typed input parameter to the function.
    ///
    /// This is a convenience method for creating a `Pat::Type` pattern.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the input parameter.
    /// - `ty`: The type of the input parameter.
    pub fn input_typed(mut self, name: impl Into<Ident>, ty: impl Into<Type>) -> Self {
        self.sig = self.sig.input_typed(name, ty);
        self
    }

    /// Sets whether the function is variadic.
    pub fn variadic(mut self, is_variadic: bool) -> Self {
        self.sig = self.sig.variadic(is_variadic);
        self
    }

    /// Sets the return type of the function.
    pub fn output(mut self, ty: impl Into<Type>) -> Self {
        self.sig = self.sig.output(ty);
        self
    }

    /// Sets the `where` clause of the function.
    pub fn where_clause(mut self, where_clause: WhereClause) -> Self {
        self.sig = self.sig.where_clause(where_clause);
        self
    }

    /// Sets the block of statements for the function.
    pub fn block(mut self, block: BlockBuilder) -> Self {
        self.block = Some(block.build());
        self
    }

    /// Builds the `TraitItemFn` AST node.
    pub fn build(self) -> TraitItemFn {
        TraitItemFn {
            sig: self.sig.build(),
            block: self.block,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `PatBuilder` to construct patterns.
pub fn pat() -> PatBuilder {
    PatBuilder::default()
}

/// A builder for constructing `Pat` AST nodes.
#[derive(Clone, Copy, Default)]
pub struct PatBuilder {
    mutability: bool,
}

impl PatBuilder {
    /// Creates a new `PatWildBuilder`.
    pub fn build(self) -> PatWildBuilder {
        PatWildBuilder::new()
    }

    /// Creates a wildcard pattern (`_`).
    pub fn wild(self) -> Pat {
        Pat::Wild(PatWild)
    }

    /// Sets the pattern to be mutable (e.g., `mut ident`).
    pub fn mutable(mut self) -> Self {
        self.mutability = true;
        self
    }

    /// Creates an identifier pattern.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the identifier.
    pub fn ident(self, name: impl Into<Ident>) -> Pat {
        Pat::Ident(PatIdent {
            ident: name.into(),
            is_mut: self.mutability,
        })
    }

    /// Creates a tuple pattern.
    ///
    /// # Parameters
    ///
    /// - `pats`: An iterator of patterns for the tuple elements.
    pub fn tuple(self, pats: impl IntoIterator<Item = impl Into<Pat>>) -> Pat {
        Pat::Tuple(PatTuple {
            pats: pats.into_iter().map(Into::into).collect(),
        })
    }

    /// Creates a rest pattern (`..`).
    pub fn rest(self) -> Pat {
        Pat::Rest(PatRest)
    }

    /// Creates a literal pattern.
    ///
    /// # Parameters
    ///
    /// - `lit`: The literal value.
    pub fn lit(self, lit: impl Into<Lit>) -> Pat {
        Pat::Lit(PatLit {
            lit: Box::new(lit.into()),
        })
    }

    /// Creates a path pattern.
    ///
    /// # Parameters
    ///
    /// - `path`: The path.
    pub fn path(self, path: impl Into<Path>) -> Pat {
        Pat::Path(PatPath { path: path.into() })
    }

    /// Creates a struct pattern.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the struct.
    pub fn struct_(self, path: impl Into<Path>) -> PatStructBuilder {
        PatStructBuilder::new(path)
    }

    /// Creates a tuple struct pattern.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the tuple struct.
    pub fn tuple_struct(self, path: impl Into<Path>) -> PatTupleStructBuilder {
        PatTupleStructBuilder::new(path)
    }

    /// Creates a slice pattern.
    ///
    /// # Parameters
    ///
    /// - `pats`: An iterator of patterns for the slice elements.
    pub fn slice(self, pats: impl IntoIterator<Item = impl Into<Pat>>) -> Pat {
        Pat::Slice(PatSlice {
            pats: pats.into_iter().map(Into::into).collect(),
        })
    }

    /// Creates a range pattern.
    ///
    /// # Parameters
    ///
    /// - `start`: The optional start of the range.
    /// - `limits`: The type of range (`..` or `..=`).
    /// - `end`: The optional end of the range.
    pub fn range(self, start: Option<Expr>, limits: RangeLimits, end: Option<Expr>) -> Pat {
        Pat::Range(PatRange {
            start: start.map(Box::new),
            end: end.map(Box::new),
            limits,
        })
    }

    /// Creates a reference pattern.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern to reference.
    pub fn reference(self, pat: impl Into<Pat>) -> PatReferenceBuilder {
        PatReferenceBuilder::new(pat)
    }

    /// Creates a parenthesized pattern.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern to parenthesize.
    pub fn paren(self, pat: impl Into<Pat>) -> Pat {
        Pat::Paren(PatParen {
            pat: Box::new(pat.into()),
        })
    }

    /// Creates an "or" pattern.
    ///
    /// # Parameters
    ///
    /// - `pats`: An iterator of patterns for the alternatives.
    pub fn or(self, pats: impl IntoIterator<Item = impl Into<Pat>>) -> Pat {
        Pat::Or(PatOr {
            pats: pats.into_iter().map(Into::into).collect(),
        })
    }

    /// Creates a macro pattern.
    ///
    /// # Parameters
    ///
    /// - `mac`: The macro call.
    pub fn mac(self, mac: impl Into<ExprMacroCall>) -> Pat {
        Pat::Macro(PatMacro { mac: mac.into() })
    }

    /// Creates a type pattern.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern.
    /// - `ty`: The type.
    pub fn type_(self, pat: impl Into<Pat>, ty: impl Into<Type>) -> Pat {
        Pat::Type(PatType {
            pat: Box::new(pat.into()),
            ty: Box::new(ty.into()),
        })
    }

    /// Creates a const pattern.
    ///
    /// # Parameters
    ///
    /// - `expr`: The const expression.
    pub fn const_(self, expr: impl Into<Expr>) -> Pat {
        Pat::Const(PatConst {
            expr: Box::new(expr.into()),
        })
    }
}

/// A builder for constructing a `PatWild` AST node.
pub struct PatWildBuilder;

impl PatWildBuilder {
    /// Creates a new `PatWildBuilder`.
    pub fn new() -> Self {
        Self {}
    }

    /// Builds the `PatWild` AST node.
    pub fn build(self) -> Pat {
        Pat::Wild(PatWild)
    }
}

impl Default for PatWildBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A builder for constructing a `PatStruct` AST node.
pub struct PatStructBuilder {
    path: Path,
    fields: ThinVec<FieldPat>,
    has_rest: bool,
}

impl PatStructBuilder {
    /// Creates a new `PatStructBuilder`.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the struct.
    pub fn new(path: impl Into<Path>) -> Self {
        Self {
            path: path.into(),
            fields: thin_vec![],
            has_rest: false,
        }
    }

    /// Adds a field to the struct pattern.
    ///
    /// # Parameters
    ///
    /// - `member`: The name of the field.
    /// - `pat`: The pattern for the field.
    pub fn field(mut self, member: impl Into<Ident>, pat: impl Into<Pat>) -> Self {
        self.fields.push(FieldPat {
            member: member.into(),
            pat: Box::new(pat.into()),
        });
        self
    }

    /// Adds a rest pattern (`..`) to the struct pattern.
    pub fn rest(mut self) -> Self {
        self.has_rest = true;
        self
    }

    /// Builds the `PatStruct` AST node.
    pub fn build(self) -> Pat {
        Pat::Struct(PatStruct {
            path: self.path,
            fields: self.fields,
            has_rest: self.has_rest,
        })
    }
}

/// A builder for constructing a `PatTupleStruct` AST node.
pub struct PatTupleStructBuilder {
    path: Path,
    pats: ThinVec<Pat>,
}

impl PatTupleStructBuilder {
    /// Creates a new `PatTupleStructBuilder`.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the tuple struct.
    pub fn new(path: impl Into<Path>) -> Self {
        Self {
            path: path.into(),
            pats: thin_vec![],
        }
    }

    /// Adds a sub-pattern to the tuple struct pattern.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern to add.
    pub fn pat(mut self, pat: impl Into<Pat>) -> Self {
        self.pats.push(pat.into());
        self
    }

    /// Builds the `PatTupleStruct` AST node.
    pub fn build(self) -> Pat {
        Pat::TupleStruct(PatTupleStruct {
            path: self.path,
            pats: self.pats,
        })
    }
}

/// A builder for constructing a `PatReference` AST node.
pub struct PatReferenceBuilder {
    pat: Box<Pat>,
    is_mut: bool,
}

impl PatReferenceBuilder {
    /// Creates a new `PatReferenceBuilder`.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern to reference.
    pub fn new(pat: impl Into<Pat>) -> Self {
        Self {
            pat: Box::new(pat.into()),
            is_mut: false,
        }
    }

    /// Marks the reference as mutable.
    pub fn mutable(mut self) -> Self {
        self.is_mut = true;
        self
    }

    /// Builds the `PatReference` AST node.
    pub fn build(self) -> Pat {
        Pat::Reference(PatReference {
            pat: self.pat,
            is_mut: self.is_mut,
        })
    }
}

/// Creates a new `TypeBuilder` to construct types.
pub fn type_() -> TypeBuilder {
    TypeBuilder
}

/// A builder for constructing `Type` AST nodes.
#[derive(Clone, Copy)]
pub struct TypeBuilder;

impl TypeBuilder {
    /// Creates an array type.
    ///
    /// # Parameters
    ///
    /// - `elem`: The element type.
    /// - `len`: The length of the array.
    pub fn array(self, elem: impl Into<Type>, len: impl Into<Expr>) -> Type {
        Type::Array(TypeArray {
            elem: Box::new(elem.into()),
            len: Box::new(len.into()),
        })
    }

    /// Creates a bare function type.
    ///
    /// # Parameters
    ///
    /// - `inputs`: An iterator of types for the function's input parameters.
    /// - `output`: The optional return type.
    pub fn bare_fn(
        self,
        inputs: impl IntoIterator<Item = impl Into<Type>>,
        output: Option<impl Into<Type>>,
    ) -> Type {
        Type::BareFn(TypeBareFn {
            inputs: inputs.into_iter().map(|t| t.into()).collect(),
            output: output.map(|t| Box::new(t.into())),
        })
    }

    /// Creates a grouped type.
    ///
    /// # Parameters
    ///
    /// - `ty`: The type to group.
    pub fn group(self, ty: impl Into<Type>) -> Type {
        Type::Group(Box::new(ty.into()))
    }

    /// Creates an `impl Trait` type.
    pub fn impl_trait(self) -> Type {
        Type::ImplTrait
    }

    /// Creates an inferred type (`_`).
    pub fn infer(self) -> Type {
        Type::Infer
    }

    /// Creates a macro type.
    ///
    /// # Parameters
    ///
    /// - `mac`: The macro call.
    pub fn mac(self, mac: impl Into<ExprMacroCall>) -> Type {
        Type::Macro(ItemMacro {
            expr: Box::new(Expr::MacroCall(mac.into())),
            md: None,
        })
    }

    /// Creates a never type (`!`).
    pub fn never(self) -> Type {
        Type::Never
    }

    /// Creates a parenthesized type.
    ///
    /// # Parameters
    ///
    /// - `ty`: The type to parenthesize.
    pub fn paren(self, ty: impl Into<Type>) -> Type {
        Type::Paren(Box::new(ty.into()))
    }

    /// Creates a path type.
    ///
    /// # Parameters
    ///
    /// - `path`: The path.
    pub fn path(self, path: impl Into<Path>) -> Type {
        Type::Path(TypePath { path: path.into() })
    }

    /// Creates a pointer type.
    ///
    /// # Parameters
    ///
    /// - `is_mut`: Whether the pointer is mutable.
    /// - `ty`: The type being pointed to.
    pub fn ptr(self, is_mut: bool, ty: impl Into<Type>) -> Type {
        Type::Ptr(TypePtr {
            mutable: is_mut,
            elem: Box::new(ty.into()),
        })
    }

    /// Creates a reference type.
    ///
    /// # Parameters
    ///
    /// - `is_mut`: Whether the reference is mutable.
    /// - `ty`: The type being referenced.
    pub fn reference(self, is_mut: bool, ty: impl Into<Type>) -> TypeReferenceBuilder {
        TypeReferenceBuilder::new(is_mut, ty)
    }

    /// Creates a slice type.
    ///
    /// # Parameters
    ///
    /// - `ty`: The element type of the slice.
    pub fn slice(self, ty: impl Into<Type>) -> Type {
        Type::Slice(Box::new(ty.into()))
    }

    /// Creates a `dyn Trait` type.
    pub fn trait_object(self) -> Type {
        Type::TraitObject
    }

    /// Creates a tuple type.
    ///
    /// # Parameters
    ///
    /// - `tys`: An iterator of types for the tuple elements.
    pub fn tuple(self, tys: impl IntoIterator<Item = impl Into<Type>>) -> Type {
        Type::Tuple(tys.into_iter().map(|t| t.into()).collect())
    }
}

/// A builder for constructing a `TypeReference` AST node.
pub struct TypeReferenceBuilder {
    is_mut: bool,
    ty: Type,
    lifetime: Option<Ident>,
}

impl TypeReferenceBuilder {
    /// Creates a new `TypeReferenceBuilder`.
    ///
    /// # Parameters
    ///
    /// - `is_mut`: Whether the reference is mutable.
    /// - `ty`: The type being referenced.
    pub fn new(is_mut: bool, ty: impl Into<Type>) -> Self {
        Self {
            is_mut,
            ty: ty.into(),
            lifetime: None,
        }
    }

    /// Sets the lifetime of the reference.
    ///
    /// # Parameters
    ///
    /// - `lifetime`: The lifetime to set.
    pub fn lifetime(mut self, lifetime: impl Into<Ident>) -> Self {
        self.lifetime = Some(lifetime.into());
        self
    }

    /// Builds the `TypeReference` AST node.
    pub fn build(self) -> Type {
        Type::Reference(TypeReference {
            mutable: self.is_mut,
            elem: Box::new(self.ty),
            lifetime: self.lifetime,
        })
    }
}

impl From<TypeReferenceBuilder> for Type {
    fn from(builder: TypeReferenceBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new `MdBuilder` to construct metadata.
pub fn md() -> MdBuilder {
    MdBuilder::new()
}

/// A builder for constructing `Md` (metadata) AST nodes.
#[derive(Default)]
pub struct MdBuilder {
    attrs: ThinVec<Attribute>,
    comments: ThinVec<Comment>,
    trailing_comments: ThinVec<Comment>,
}

impl MdBuilder {
    /// Creates a new `MdBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an attribute to the metadata.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.attrs.push(attr.into());
        self
    }

    /// Adds a comment to the metadata.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.comments.push(comment.into());
        self
    }

    /// Adds a trailing comment to the metadata.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    /// Returns true if no metadata has been added.
    pub fn is_empty(&self) -> bool {
        self.attrs.is_empty() && self.comments.is_empty() && self.trailing_comments.is_empty()
    }

    /// Builds the `Md` AST node.
    pub fn build(self) -> Md {
        Md {
            attrs: self.attrs,
            comments: self.comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

impl From<MdBuilder> for Md {
    fn from(builder: MdBuilder) -> Self {
        builder.build()
    }
}

impl From<MdBuilder> for Option<Box<Md>> {
    fn from(builder: MdBuilder) -> Self {
        Some(Box::new(builder.build()))
    }
}

impl From<ItemExternCrateBuilder> for Item {
    /// Converts an `ItemExternCrateBuilder` into an `Item::ExternCrate` variant.
    fn from(builder: ItemExternCrateBuilder) -> Self {
        Item::ExternCrate(builder.build())
    }
}

impl From<ItemExternBlockBuilder> for Item {
    /// Converts an `ItemExternBlockBuilder` into an `Item::ExternBlock` variant.
    fn from(builder: ItemExternBlockBuilder) -> Self {
        Item::ExternBlock(builder.build())
    }
}

/// Creates a new `ItemExternTypeBuilder` to construct an `extern type` item.
pub fn extern_type_item(name: impl Into<Ident>) -> ItemExternTypeBuilder {
    ItemExternTypeBuilder::new(name)
}

/// A builder for constructing an `ItemExternType` AST node.
pub struct ItemExternTypeBuilder {
    ident: Ident,
    vis: Visibility,
    md: MdBuilder,
}

impl ItemExternTypeBuilder {
    /// Creates a new `ItemExternTypeBuilder`.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the extern type item.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a comment to the `extern type` item.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the `extern type` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemExternType` AST node.
    pub fn build(self) -> ItemExternType {
        ItemExternType {
            vis: self.vis,
            ident: self.ident,
            md: Some(Box::new(self.md.build())),
        }
    }
}

impl From<ItemExternTypeBuilder> for Item {
    /// Converts an `ItemExternTypeBuilder` into an `Item::ExternType` variant.
    fn from(builder: ItemExternTypeBuilder) -> Self {
        Item::ExternType(builder.build())
    }
}

impl From<ItemExternType> for ExternalItem {
    /// Converts an `ItemExternType` into an `ExternalItem::Type` variant.
    fn from(item: ItemExternType) -> Self {
        ExternalItem::Type(item)
    }
}

impl From<ItemForeignModBuilder> for Item {
    /// Converts an `ItemForeignModBuilder` into an `Item::ForeignMod` variant.
    fn from(builder: ItemForeignModBuilder) -> Self {
        Item::ForeignMod(builder.build())
    }
}

impl From<ItemMacroBuilder> for Item {
    /// Converts an `ItemMacroBuilder` into an `Item::Macro` variant.
    fn from(builder: ItemMacroBuilder) -> Self {
        Item::Macro(builder.build())
    }
}

impl From<ItemModBuilder> for Item {
    /// Converts an `ItemModBuilder` into an `Item::Mod` variant.
    fn from(builder: ItemModBuilder) -> Self {
        Item::Mod(builder.build())
    }
}

impl From<ItemTraitAliasBuilder> for Item {
    /// Converts an `ItemTraitAliasBuilder` into an `Item::TraitAlias` variant.
    fn from(builder: ItemTraitAliasBuilder) -> Self {
        Item::TraitAlias(builder.build())
    }
}

impl From<ItemUnionBuilder> for Item {
    /// Converts an `ItemUnionBuilder` into an `Item::Union` variant.
    fn from(builder: ItemUnionBuilder) -> Self {
        Item::Union(builder.build())
    }
}

impl From<ItemUseBuilder> for Item {
    /// Converts an `ItemUseBuilder` into an `Item::Use` variant.
    fn from(builder: ItemUseBuilder) -> Self {
        Item::Use(builder.build())
    }
}

impl From<AsmBuilder> for Item {
    /// Converts an `AsmBuilder` into an `Item::Asm` variant.
    fn from(builder: AsmBuilder) -> Self {
        Item::Asm(builder.build())
    }
}

impl From<AssociatedConstBuilder> for ImplItem {
    /// Converts an `AssociatedConstBuilder` into an `ImplItem::Const` variant.
    fn from(builder: AssociatedConstBuilder) -> Self {
        ImplItem::Const(builder.build())
    }
}

impl From<AssociatedTypeBuilder> for ImplItem {
    /// Converts an `AssociatedTypeBuilder` into an `ImplItem::Type` variant.
    fn from(builder: AssociatedTypeBuilder) -> Self {
        ImplItem::Type(builder.build())
    }
}

impl From<FnBuilder> for ImplItem {
    /// Converts a `FnBuilder` into an `ImplItem::Fn` variant.
    fn from(builder: FnBuilder) -> Self {
        ImplItem::Fn(builder.build())
    }
}

impl From<TraitBuilder> for Item {
    /// Converts a `TraitBuilder` into an `Item::Trait` variant.
    fn from(builder: TraitBuilder) -> Self {
        Item::Trait(builder.build())
    }
}

impl From<EnumBuilder> for Item {
    /// Converts an `EnumBuilder` into an `Item::Enum` variant.
    fn from(builder: EnumBuilder) -> Self {
        Item::Enum(builder.build())
    }
}

impl From<StructBuilder> for Item {
    /// Converts a `StructBuilder` into an `Item::Struct` variant.
    fn from(builder: StructBuilder) -> Self {
        Item::Struct(builder.build())
    }
}

/// Creates a new `StaticItemBuilder` to construct a static item.
///
/// # Parameters
///
/// - `name`: The name of the static item.
/// - `ty`: The type of the static item.
/// - `expr`: The expression of the static item.
///
/// # Returns
///
/// A `StaticItemBuilder` instance.
pub fn static_item(
    name: impl Into<Ident>,
    ty: impl Into<Type>,
    expr: impl Into<Expr>,
) -> StaticItemBuilder {
    StaticItemBuilder::new(name, ty, expr)
}

/// A builder for constructing an `ItemStatic` (static item) AST node.
pub struct StaticItemBuilder {
    ident: Ident,
    vis: Visibility,
    is_mut: bool,
    ty: Type,
    expr: Box<Expr>,
    md: MdBuilder,
}

impl StaticItemBuilder {
    /// Creates a new `StaticItemBuilder` with the given name, type and expression.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the static item.
    /// - `ty`: The type of the static item.
    /// - `expr`: The expression of the static item.
    pub fn new(name: impl Into<Ident>, ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            is_mut: false,
            ty: ty.into(),
            expr: Box::new(expr.into()),
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the static item.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Sets the static item as mutable.
    pub fn mutable(mut self) -> Self {
        self.is_mut = true;
        self
    }

    /// Adds a comment to the static item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the static item.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemStatic` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemStatic` instance.
    pub fn build(self) -> ItemStatic {
        ItemStatic {
            vis: self.vis,
            ident: self.ident,
            is_mut: self.is_mut,
            ty: self.ty,
            expr: self.expr,
            md: Some(Box::new(self.md.build())),
        }
    }
}

impl From<StaticItemBuilder> for Item {
    /// Converts a `StaticItemBuilder` into an `Item::Static` variant.
    fn from(builder: StaticItemBuilder) -> Self {
        Item::Static(builder.build())
    }
}

impl From<i32> for Expr {
    /// Converts an `i32` into an `Expr::Lit` variant.
    fn from(val: i32) -> Self {
        Expr::Lit(val.into())
    }
}

impl From<u128> for Lit {
    /// Converts a `u128` into a `Lit::Int` variant.
    fn from(val: u128) -> Self {
        Lit::Int(LitInt::new(val))
    }
}

impl From<u128> for Expr {
    /// Converts a `u128` into an `Expr::Lit` variant.
    fn from(val: u128) -> Self {
        Expr::Lit(val.into())
    }
}

impl From<bool> for Expr {
    /// Converts a `bool` into an `Expr::Lit` variant.
    fn from(val: bool) -> Self {
        Expr::Lit(val.into())
    }
}

impl From<&str> for Expr {
    /// Converts a `&str` into an `Expr::Lit` variant.
    fn from(val: &str) -> Self {
        Expr::Lit(val.into())
    }
}

impl From<String> for Expr {
    /// Converts a `String` into an `Expr::Lit` variant.
    fn from(val: String) -> Self {
        Expr::Lit(val.into())
    }
}

/// Creates a new `PathBuilder` to construct a path.
pub fn path(segment: impl Into<Ident>) -> PathBuilder {
    PathBuilder::new(segment)
}

/// A builder for constructing a `Path` AST node.
pub struct PathBuilder {
    segments: ThinVec<PathSegment>,
}

impl PathBuilder {
    /// Creates a new `PathBuilder` with the given segment.
    ///
    /// # Parameters
    ///
    /// - `segment`: The first segment of the path.
    pub fn new(segment: impl Into<Ident>) -> Self {
        Self {
            segments: thin_vec![PathSegment {
                ident: segment.into(),
                args: None,
            }],
        }
    }

    /// Adds a segment to the path.
    ///
    /// # Parameters
    ///
    /// - `segment`: The segment to add.
    pub fn segment(mut self, segment: impl Into<Ident>) -> Self {
        self.segments.push(PathSegment {
            ident: segment.into(),
            args: None,
        });
        self
    }

    /// Builds the `Path` AST node.
    ///
    /// # Returns
    ///
    /// A `Path` instance.
    pub fn build(self) -> Path {
        Path {
            segments: self.segments,
        }
    }

    /// Adds a generic argument to the last segment.
    ///
    /// # Parameters
    ///
    /// - `arg`: The generic argument to add.
    pub fn generic(mut self, arg: impl Into<GenericArg>) -> Self {
        let segment = self.segments.last_mut().unwrap();
        let args = segment.args.get_or_insert_with(Default::default);
        args.args.push(arg.into());
        self
    }

    /// Builds a `Type::Path` from the `PathBuilder`.
    ///
    /// # Returns
    ///
    /// A `Type` instance representing the path.
    pub fn build_type(self) -> Type {
        Type::Path(TypePath { path: self.build() })
    }
}

/// Creates a new `ExprBuilder` to construct expressions.
pub fn expr() -> ExprBuilder {
    ExprBuilder
}

/// A builder for constructing `Expr` (expression) AST nodes.
#[derive(Clone, Copy)]
pub struct ExprBuilder;

impl ExprBuilder {
    /// Creates an array expression.
    ///
    /// # Parameters
    ///
    /// - `elems`: An iterator of expressions for the array elements.
    pub fn array(self, elems: impl IntoIterator<Item = Expr>) -> Expr {
        Expr::Array(ExprArray {
            elems: elems.into_iter().collect(),
        })
    }

    /// Creates a unary operation expression.
    ///
    /// # Parameters
    ///
    /// - `op`: The unary operator.
    /// - `expr`: The expression.
    pub fn unary(self, op: UnOp, expr: Expr) -> Expr {
        Expr::Unary(ExprUnary {
            op,
            expr: Box::new(expr),
        })
    }

    /// Creates a raw reference expression.
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to reference.
    pub fn raw_ref(self, expr: Expr) -> ExprRawRefBuilder {
        ExprRawRefBuilder::new(expr)
    }

    /// Creates an assignment expression.
    ///
    /// # Parameters
    ///
    /// - `left`: The expression on the left-hand side of the assignment.
    /// - `right`: The expression on the right-hand side of the assignment.
    pub fn assign(self, left: Expr, right: Expr) -> Expr {
        Expr::Assign(ExprAssign {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /// Creates an `async` block expression.
    ///
    /// # Parameters
    ///
    /// - `block`: The block of statements inside the `async` block.
    pub fn async_block(self, block: impl Into<Block>) -> Expr {
        Expr::Async(ExprAsync {
            block: block.into(),
        })
    }

    /// Creates an `await` expression.
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to `await`.
    pub fn await_expr(self, expr: Expr) -> Expr {
        Expr::Await(ExprAwait {
            expr: Box::new(expr),
        })
    }

    /// Creates a binary operation expression.
    ///
    /// # Parameters
    ///
    /// - `left`: The left-hand side expression.
    /// - `op`: The binary operator.
    /// - `right`: The right-hand side expression.
    pub fn binary(self, left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::Binary(ExprBinary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    /// Creates a block expression.
    ///
    /// # Parameters
    ///
    /// - `block`: The block of statements.
    pub fn block(self, block: impl Into<Block>) -> Expr {
        Expr::Block(ExprBlock {
            block: block.into(),
        })
    }

    /// Creates a `break` expression.
    pub fn break_expr(self) -> Expr {
        Expr::Break(ExprBreak)
    }

    /// Creates a function call expression.
    ///
    /// # Parameters
    ///
    /// - `func`: The function to call.
    /// - `args`: An iterator of expressions for the function arguments.
    pub fn call(self, func: Expr, args: impl IntoIterator<Item = Expr>) -> Expr {
        Expr::Call(ExprCall {
            func: Box::new(func),
            args: args.into_iter().collect(),
        })
    }

    /// Creates a type cast expression.
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to cast.
    /// - `ty`: The type to cast to.
    pub fn cast(self, expr: Expr, ty: impl Into<Type>) -> Expr {
        Expr::Cast(ExprCast {
            expr: Box::new(expr),
            ty: ty.into(),
        })
    }

    /// Creates a closure expression.
    ///
    /// # Parameters
    ///
    /// - `inputs`: An iterator of patterns for the closure's input parameters.
    /// - `body`: The body of the closure.
    pub fn closure(self, inputs: impl IntoIterator<Item = impl Into<Pat>>, body: Expr) -> Expr {
        Expr::Closure(ExprClosure {
            inputs: inputs.into_iter().map(Into::into).collect(),
            body: Box::new(body),
        })
    }

    /// Creates a `const` block expression.
    ///
    /// # Parameters
    ///
    /// - `block`: The block of statements inside the `const` block.
    pub fn const_block(self, block: impl Into<Block>) -> Expr {
        Expr::Const(ExprConst {
            block: block.into(),
        })
    }

    /// Creates a `continue` expression.
    pub fn continue_expr(self) -> Expr {
        Expr::Continue(ExprContinue)
    }

    /// Creates a field access expression.
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to access the field from.
    /// - `member`: The name of the field.
    pub fn field(self, expr: Expr, member: impl Into<Ident>) -> Expr {
        Expr::Field(ExprField {
            expr: Box::new(expr),
            member: member.into(),
        })
    }

    /// Creates a `for` loop expression.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern to bind the elements of the iterator.
    /// - `expr`: The expression to iterate over.
    /// - `body`: The body of the loop.
    pub fn for_loop(self, pat: impl Into<Pat>, expr: Expr, body: impl Into<Block>) -> Expr {
        Expr::For(ExprFor {
            pat: pat.into(),
            expr: Box::new(expr),
            body: body.into(),
        })
    }

    /// Creates a `gen` block expression.
    ///
    /// # Parameters
    ///
    /// - `block`: The block of statements inside the `gen` block.
    pub fn gen_block(self, block: impl Into<Block>) -> Expr {
        Expr::Gen(ExprGen {
            block: block.into(),
        })
    }

    /// Creates an `if` expression.
    ///
    /// # Parameters
    ///
    /// - `cond`: The condition expression.
    /// - `then_branch`: The block to execute if the condition is true.
    /// - `else_branch`: An optional `else` branch.
    pub fn if_expr(
        self,
        cond: Expr,
        then_branch: impl Into<Block>,
        else_branch: Option<Expr>,
    ) -> Expr {
        Expr::If(ExprIf {
            cond: Box::new(cond),
            then_branch: then_branch.into(),
            else_branch: else_branch.map(Box::new),
        })
    }

    /// Creates an index expression (e.g., `array[index]`).
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to index into.
    /// - `index`: The index expression.
    pub fn index(self, expr: Expr, index: Expr) -> Expr {
        Expr::Index(ExprIndex {
            expr: Box::new(expr),
            index: Box::new(index),
        })
    }

    /// Creates an inferred expression (`_`).
    pub fn infer(self) -> Expr {
        Expr::Infer(ExprInfer)
    }

    /// Creates a literal expression.
    ///
    /// # Parameters
    ///
    /// - `lit`: The literal value.
    pub fn lit(self, lit: impl Into<Lit>) -> Expr {
        Expr::Lit(lit.into())
    }

    /// Creates an integer literal expression with a specific suffix.
    ///
    /// # Parameters
    ///
    /// - `value`: The integer value.
    /// - `suffix`: The integer suffix (e.g., `u32`, `i64`).
    pub fn int_lit_with_suffix(self, value: i32, suffix: IntSuffix) -> Expr {
        Expr::Lit(Lit::Int(LitInt::with_suffix(value as u128, suffix)))
    }

    /// Creates a float literal expression with a specific suffix.
    ///
    /// # Parameters
    ///
    /// - `value`: The float value as a string.
    /// - `suffix`: The float suffix (e.g., `f32`, `f64`).
    pub fn float_lit_with_suffix(self, value: &str, suffix: FloatSuffix) -> Expr {
        Expr::Lit(Lit::Float(LitFloat::with_suffix(value, suffix)))
    }

    /// Creates a `loop` expression.
    ///
    /// # Parameters
    ///
    /// - `body`: The body of the loop.
    pub fn loop_expr(self, body: impl Into<Block>) -> Expr {
        Expr::Loop(ExprLoop { body: body.into() })
    }

    /// Creates a macro call expression.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the macro.
    /// - `delimiter`: The delimiter of the macro's input.
    /// - `tokens`: The token stream passed to the macro.
    pub fn macro_call(
        self,
        path: impl Into<Path>,
        delimiter: Delimiter,
        tokens: impl Into<TokenStream>,
    ) -> Expr {
        Expr::MacroCall(ExprMacroCall {
            path: path.into(),
            delimiter,
            tokens: tokens.into(),
        })
    }

    /// Creates a `match` expression.
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to match on.
    /// - `arms`: An iterator of `Arm`s for the match expression.
    pub fn match_expr(self, expr: Expr, arms: impl IntoIterator<Item = Arm>) -> Expr {
        Expr::Match(ExprMatch {
            expr: Box::new(expr),
            arms: arms.into_iter().collect(),
        })
    }

    /// Creates a new `ArmBuilder` to construct a match arm.
    ///
    /// # Parameters
    ///
    /// - `pat`: The pattern for the arm.
    pub fn arm(self, pat: impl Into<Pat>) -> ArmBuilder {
        ArmBuilder::new(pat)
    }

    /// Creates a method call expression.
    ///
    /// # Parameters
    ///
    /// - `receiver`: The expression to call the method on.
    /// - `method`: The name of the method.
    /// - `args`: An iterator of expressions for the method arguments.
    pub fn method_call(
        self,
        receiver: Expr,
        method: impl Into<Ident>,
        args: impl IntoIterator<Item = Expr>,
    ) -> Expr {
        Expr::MethodCall(ExprMethodCall {
            receiver: Box::new(receiver),
            method: method.into(),
            args: args.into_iter().collect(),
        })
    }

    /// Creates a parenthesized expression.
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to wrap in parentheses.
    pub fn paren(self, expr: Expr) -> Expr {
        Expr::Paren(ExprParen {
            expr: Box::new(expr),
        })
    }

    /// Creates a path expression.
    ///
    /// # Parameters
    ///
    /// - `path`: The path.
    pub fn path(self, path: impl Into<Path>) -> Expr {
        Expr::Path(ExprPath { path: path.into() })
    }

    /// Creates a range expression.
    ///
    /// # Parameters
    ///
    /// - `start`: The optional start of the range.
    /// - `limits`: The type of range (`..` or `..=`).
    /// - `end`: The optional end of the range.
    pub fn range(self, start: Option<Expr>, limits: RangeLimits, end: Option<Expr>) -> Expr {
        Expr::Range(ExprRange {
            start: start.map(Box::new),
            limits,
            end: end.map(Box::new),
        })
    }

    /// Creates a reference expression.
    ///
    /// # Parameters
    ///
    /// - `is_mut`: Whether the reference is mutable.
    /// - `expr`: The expression to reference.
    pub fn reference(self, is_mut: bool, expr: Expr) -> Expr {
        Expr::Reference(ExprRef {
            is_mut,
            expr: Box::new(expr),
        })
    }

    /// Creates a `return` expression.
    ///
    /// # Parameters
    ///
    /// - `expr`: The optional expression to return.
    pub fn return_expr(self, expr: Option<Expr>) -> Expr {
        Expr::Return(ExprReturn {
            expr: expr.map(Box::new),
        })
    }

    /// Creates a struct instantiation expression.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the struct.
    /// - `fields`: An iterator of `FieldValue`s for the struct fields.
    pub fn struct_expr(
        self,
        path_str: impl Into<String>,
        fields: impl IntoIterator<Item = FieldValue>,
    ) -> Expr {
        Expr::Struct(ExprStruct {
            path: path(path_str.into()).build(),
            fields: fields.into_iter().collect(),
        })
    }

    /// Creates a `try` block expression.
    ///
    /// # Parameters
    ///
    /// - `block`: The block of statements inside the `try` block.
    pub fn try_block(self, block: impl Into<Block>) -> Expr {
        Expr::Try(ExprTry {
            block: block.into(),
        })
    }

    /// Creates a tuple expression.
    ///
    /// # Parameters
    ///
    /// - `elems`: An iterator of expressions for the tuple elements.
    pub fn tuple(self, elems: impl IntoIterator<Item = Expr>) -> Expr {
        Expr::Tuple(ExprTuple {
            elems: elems.into_iter().collect(),
        })
    }

    /// Creates a `while` loop expression.
    ///
    /// # Parameters
    ///
    /// - `cond`: The condition expression.
    /// - `body`: The body of the loop.
    pub fn while_loop(self, cond: Expr, body: impl Into<Block>) -> Expr {
        Expr::While(ExprWhile {
            cond: Box::new(cond),
            body: body.into(),
        })
    }
}

/// A builder for constructing a raw reference expression.
pub struct ExprRawRefBuilder {
    expr: Expr,
    is_mut: bool,
}

impl ExprRawRefBuilder {
    /// Creates a new `ExprRawRefBuilder`.
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression to be referenced.
    pub fn new(expr: Expr) -> Self {
        Self {
            expr,
            is_mut: false,
        }
    }

    /// Marks the raw reference as mutable.
    pub fn mutable(mut self) -> Self {
        self.is_mut = true;
        self
    }

    /// Builds the `Expr::RawRef`.
    ///
    /// # Returns
    ///
    /// An `Expr` instance representing the raw reference.
    pub fn build(self) -> Expr {
        Expr::RawRef(ExprRawRef {
            expr: Box::new(self.expr),
            is_mut: self.is_mut,
        })
    }
}

impl From<ExprRawRefBuilder> for Expr {
    /// Converts an `ExprRawRefBuilder` into an `Expr::RawRef` variant.
    fn from(builder: ExprRawRefBuilder) -> Self {
        builder.build()
    }
}

impl From<Expr> for Stmt {
    /// Converts an `Expr` into a `Stmt::Expr` variant.
    fn from(value: Expr) -> Stmt {
        Stmt::Expr(value)
    }
}

/// Creates a new `ItemExternCrateBuilder` to construct an `extern crate` item.
pub fn extern_crate_item(name: impl Into<Ident>) -> ItemExternCrateBuilder {
    ItemExternCrateBuilder::new(name)
}

/// Creates a new `ItemExternBlockBuilder` to construct an `extern` block item.
pub fn extern_block_item() -> ItemExternBlockBuilder {
    ItemExternBlockBuilder::new()
}

/// A builder for constructing an `ItemExternCrate` AST node.
pub struct ItemExternCrateBuilder {
    ident: Ident,
    md: MdBuilder,
}

impl ItemExternCrateBuilder {
    /// Creates a new `ItemExternCrateBuilder`.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the crate.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            md: MdBuilder::new(),
        }
    }

    /// Adds a comment to the `extern crate` item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the `extern crate` item.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemExternCrate` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemExternCrate` instance.
    pub fn build(self) -> ItemExternCrate {
        ItemExternCrate {
            ident: self.ident,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemForeignModBuilder` to construct a foreign module.
pub fn foreign_mod_item(abi: impl Into<String>) -> ItemForeignModBuilder {
    ItemForeignModBuilder::new(abi)
}

/// A builder for constructing an `ItemForeignMod` AST node.
pub struct ItemForeignModBuilder {
    abi: String,
    items: ThinVec<Item>,
    md: MdBuilder,
}

impl ItemForeignModBuilder {
    /// Creates a new `ItemForeignModBuilder`.
    ///
    /// # Parameters
    ///
    /// - `abi`: The ABI of the foreign module (e.g., "C").
    pub fn new(abi: impl Into<String>) -> Self {
        Self {
            abi: abi.into(),
            items: thin_vec![],
            md: MdBuilder::new(),
        }
    }

    /// Adds an item to the foreign module.
    ///
    /// # Parameters
    ///
    /// - `item`: The `Item` to add.
    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds a comment to the foreign module.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the foreign module.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemForeignMod` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemForeignMod` instance.
    pub fn build(self) -> ItemForeignMod {
        ItemForeignMod {
            abi: self.abi,
            items: self.items,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemMacroBuilder` to construct a macro item.
pub fn macro_item(expr: impl Into<Expr>) -> ItemMacroBuilder {
    ItemMacroBuilder::new(expr)
}

/// A builder for constructing an `ItemMacro` AST node.
pub struct ItemMacroBuilder {
    expr: Expr,
    md: MdBuilder,
}

impl ItemMacroBuilder {
    /// Creates a new `ItemMacroBuilder`.
    ///
    /// # Parameters
    ///
    /// - `expr`: The macro invocation `Expr`.
    pub fn new(expr: impl Into<Expr>) -> Self {
        Self {
            expr: expr.into(),
            md: MdBuilder::new(),
        }
    }

    /// Adds a comment to the macro item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the macro item.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemMacro` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemMacro` instance.
    pub fn build(self) -> ItemMacro {
        ItemMacro {
            expr: Box::new(self.expr),
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemModBuilder` to construct a module item.
pub fn mod_item(name: impl Into<Ident>) -> ItemModBuilder {
    ItemModBuilder::new(name)
}

/// Create a new `EmptyItemModBuilder` to construct a module item in separated file.
pub fn empty_mod_item(name: impl Into<Ident>) -> EmptyItemModBuilder {
    EmptyItemModBuilder::new(name)
}

/// A builder for constructing an `ItemMod` AST node.
#[derive(Default)]
pub struct ItemModBuilder {
    ident: Ident,
    vis: Visibility,
    content: ThinVec<Item>,
    md: MdBuilder,
}

impl ItemModBuilder {
    /// Creates a new `ItemModBuilder`.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            ..Default::default()
        }
    }

    /// Sets the visibility of the module.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Sets the content of the module.
    ///
    /// # Parameters
    ///
    /// - `content`: A `ThinVec<Item>` containing the module's content.
    pub fn content(mut self, content: ThinVec<Item>) -> Self {
        self.content = content;
        self
    }

    /// Adds an item to the module's content.
    ///
    /// If the module was previously defined without a block (e.g., `mod my_mod;`),
    /// this will initialize an empty content block before adding the item.
    ///
    /// # Parameters
    ///
    /// - `item`: The item to add to the module.
    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.content.push(item.into());
        self
    }

    /// Adds a comment to the module item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the module item.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemMod` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemMod` instance.
    pub fn build(self) -> ItemMod {
        ItemMod {
            vis: self.vis,
            ident: self.ident,
            content: Some(self.content),
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// A builder for constructing an `ItemMod` AST node.
#[derive(Default)]
pub struct EmptyItemModBuilder {
    ident: Ident,
    vis: Visibility,
    md: MdBuilder,
}

impl EmptyItemModBuilder {
    /// Creates a new `EmptyItemModBuilder`.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            ..Default::default()
        }
    }

    /// Sets the visibility of the module.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a comment to the module item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the module item.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemMod` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemMod` instance.
    pub fn build(self) -> ItemMod {
        ItemMod {
            vis: self.vis,
            ident: self.ident,
            content: None,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemTraitAliasBuilder` to construct a trait alias.
pub fn trait_alias_item(name: impl Into<Ident>, bounds: ThinVec<String>) -> ItemTraitAliasBuilder {
    ItemTraitAliasBuilder::new(name, bounds)
}

/// A builder for constructing an `ItemTraitAlias` AST node.
pub struct ItemTraitAliasBuilder {
    ident: Ident,
    bounds: ThinVec<String>,
    md: MdBuilder,
}

impl ItemTraitAliasBuilder {
    /// Creates a new `ItemTraitAliasBuilder`.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the trait alias.
    /// - `bounds`: The bounds of the trait alias.
    pub fn new(name: impl Into<Ident>, bounds: ThinVec<String>) -> Self {
        Self {
            ident: name.into(),
            bounds,
            md: MdBuilder::new(),
        }
    }

    /// Adds a comment to the trait alias.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the trait alias.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemTraitAlias` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemTraitAlias` instance.
    pub fn build(self) -> ItemTraitAlias {
        ItemTraitAlias {
            ident: self.ident,
            bounds: self.bounds,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemUnionBuilder` to construct a `union` item.
pub fn union_item(name: impl Into<Ident>) -> ItemUnionBuilder {
    ItemUnionBuilder::new(name)
}

/// A builder for constructing an `ItemUnion` AST node.
pub struct ItemUnionBuilder {
    ident: Ident,
    vis: Visibility,
    fields: ThinVec<Field>,
    generics: GenericParams,
    md: MdBuilder,
}

impl ItemUnionBuilder {
    /// Creates a new `ItemUnionBuilder`.
    pub fn new(name: impl Into<Ident>) -> Self {
        Self {
            ident: name.into(),
            vis: Visibility::Default,
            generics: GenericParams::new(),
            fields: thin_vec![],
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the union.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a generic parameter to the union.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Adds a field to the `union`.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the field.
    /// - `ty`: The `Type` of the field.
    pub fn field(mut self, name: impl Into<Ident>, ty: impl Into<Type>) -> Self {
        self.fields.push(Field {
            ident: name.into(),
            ty: ty.into(),
            md: None,
        });
        self
    }

    /// Adds a comment to the `union` item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the `union` item.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemUnion` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemUnion` instance.
    pub fn build(self) -> ItemUnion {
        ItemUnion {
            vis: self.vis,
            ident: self.ident,
            generics: self.generics,
            fields: self.fields,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemUseBuilder` to construct a `use` item.
pub fn use_item(path: impl Into<String>) -> ItemUseBuilder {
    ItemUseBuilder::new(path)
}

/// A builder for constructing an `ItemUse` AST node.
pub struct ItemUseBuilder {
    path: String,
    vis: Visibility,
    md: MdBuilder,
}

impl ItemUseBuilder {
    /// Creates a new `ItemUseBuilder`.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            vis: Visibility::Default,
            md: MdBuilder::new(),
        }
    }

    /// Sets the visibility of the use item.
    ///
    /// # Parameters
    ///
    /// - `vis`: The `Visibility` to set.
    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    /// Adds a comment to the `use` item.
    ///
    /// # Parameters
    ///
    /// - `comment`: The `Comment` to add.
    pub fn comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.comment(comment.into());
        self
    }

    /// Adds an attribute to the `use` item.
    ///
    /// # Parameters
    ///
    /// - `attr`: The `Attribute` to add.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemUse` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemUse` instance.
    pub fn build(self) -> ItemUse {
        ItemUse {
            vis: self.vis,
            path: self.path,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `AttributeBuilder` to construct an attribute.
pub fn attr() -> AttributeBuilder {
    AttributeBuilder::new()
}

/// A builder for constructing an `Attribute` AST node.
#[derive(Default)]
pub struct AttributeBuilder {
    is_inner: bool,
    meta: Option<Meta>,
}

impl AttributeBuilder {
    /// Creates a new `AttributeBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the attribute as an inner attribute (e.g., `#![foo]`).
    pub fn inner(mut self) -> Self {
        self.is_inner = true;
        self
    }

    /// Sets the meta item for the attribute.
    ///
    /// # Parameters
    ///
    /// - `meta`: The `Meta` item to set.
    pub fn meta(mut self, meta: impl Into<Meta>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    /// Builds the `Attribute` AST node.
    ///
    /// # Panics
    ///
    /// Panics if the meta item has not been set.
    pub fn build(self) -> Attribute {
        let meta = self.meta.expect("meta is required");
        if self.is_inner {
            Attribute::Inner(meta)
        } else {
            Attribute::Outer(meta)
        }
    }
}

impl From<AttributeBuilder> for Attribute {
    fn from(value: AttributeBuilder) -> Self {
        value.build()
    }
}

/// Creates a new `MetaBuilder` to construct a meta item.
pub fn meta() -> MetaBuilder {
    MetaBuilder
}

/// A builder for constructing `Meta` AST nodes.
#[derive(Clone, Copy)]
pub struct MetaBuilder;

impl MetaBuilder {
    /// Creates a meta list, e.g., `path(meta1, meta2)`.
    ///
    /// # Parameters
    ///
    /// - `path`: The path of the meta list.
    /// - `metas`: An iterator of `Meta` items for the list.
    pub fn list(
        self,
        path: impl Into<Ident>,
        metas: impl IntoIterator<Item = impl Into<Meta>>,
    ) -> Meta {
        Meta::List(MetaList {
            path: path.into(),
            metas: metas.into_iter().map(Into::into).collect(),
        })
    }

    /// Creates a meta path, e.g., `path`.
    ///
    /// # Parameters
    ///
    /// - `path`: The path of the meta item.
    pub fn path(self, path: impl Into<Ident>) -> Meta {
        Meta::Path(path.into())
    }

    /// Creates a meta name-value pair, e.g., `path = "value"`.
    ///
    /// # Parameters
    ///
    /// - `path`: The path of the meta item.
    /// - `value`: The `Lit` value of the meta item.
    pub fn name_value(self, path: impl Into<Ident>, value: impl Into<Lit>) -> Meta {
        Meta::NameValue(MetaNameValue {
            path: path.into(),
            value: value.into(),
        })
    }
}

/// Creates a new `TokenTreeBuilder` to construct `TokenTree` nodes.
pub fn tt() -> TokenTreeBuilder {
    TokenTreeBuilder {}
}

/// A builder for constructing `TokenTree` AST nodes.
#[derive(Clone, Copy, Default)]
pub struct TokenTreeBuilder;

impl TokenTreeBuilder {
    /// Creates a literal token tree.
    ///
    /// # Parameters
    ///
    /// - `value`: The literal value.
    pub fn lit(self, value: impl Into<Lit>) -> TokenTree {
        TokenTree::Literal(value.into())
    }

    /// Creates an identifier token tree.
    ///
    /// # Parameters
    ///
    /// - `value`: The identifier string.
    pub fn ident(self, value: impl Into<String>) -> TokenTree {
        TokenTree::Ident(value.into())
    }

    /// Creates a punctuation token tree.
    ///
    /// # Parameters
    ///
    /// - `ch`: The punctuation character.
    /// - `spacing`: The spacing of the punctuation.
    pub fn punct(self, ch: char, spacing: Spacing) -> TokenTree {
        TokenTree::Punct(Punct { ch, spacing })
    }
}

impl From<LocalBuilder> for Stmt {
    /// Converts a `LocalBuilder` into a `Stmt::Local` variant.
    fn from(value: LocalBuilder) -> Self {
        value.build()
    }
}

impl From<PathBuilder> for Path {
    /// Converts a `PathBuilder` into a `Path`.
    fn from(builder: PathBuilder) -> Self {
        builder.build()
    }
}

impl From<&str> for Path {
    /// Converts a `&str` into a `Path`.
    fn from(value: &str) -> Self {
        path(value).build()
    }
}

impl<const N: usize> From<&[&str; N]> for Path {
    /// Converts an array of `&str` into a `Path`.
    fn from(array: &[&str; N]) -> Self {
        let array: ThinVec<PathSegment> = array
            .iter()
            .map(|x| PathSegment {
                ident: (*x).into(),
                args: None,
            })
            .collect();
        Path { segments: array }
    }
}

impl From<&str> for Pat {
    /// Converts a `&str` into a `Pat::Ident` variant.
    fn from(val: &str) -> Self {
        Pat::Ident(PatIdent {
            ident: val.into(),
            is_mut: false,
        })
    }
}

impl From<BlockBuilder> for Block {
    /// Converts a `BlockBuilder` into a `Block`.
    fn from(val: BlockBuilder) -> Self {
        val.build()
    }
}

impl From<FnBuilder> for ItemFn {
    /// Converts a `FnBuilder` into an `ItemFn`.
    fn from(val: FnBuilder) -> Self {
        val.build()
    }
}

impl From<FnBuilder> for Item {
    /// Converts a `FnBuilder` into an `Item::Fn` variant.
    fn from(val: FnBuilder) -> Self {
        Item::Fn(val.into())
    }
}

impl From<TraitItemFnBuilder> for TraitItem {
    /// Converts a `TraitItemFnBuilder` into a `TraitItem::Fn` variant.
    fn from(val: TraitItemFnBuilder) -> Self {
        TraitItem::Fn(val.build())
    }
}

impl From<AssociatedTypeBuilder> for AssociatedType {
    /// Converts an `AssociatedTypeBuilder` into an `AssociatedType`.
    fn from(val: AssociatedTypeBuilder) -> Self {
        val.build()
    }
}

impl From<Vec<Stmt>> for Block {
    /// Converts a `Vec<Stmt>` into a `Block`.
    fn from(array: Vec<Stmt>) -> Self {
        Block {
            stmts: array.into(),
            ..Default::default()
        }
    }
}

impl From<Vec<Expr>> for Block {
    /// Converts a `Vec<Expr>` into a `Block`.
    fn from(array: Vec<Expr>) -> Self {
        Block {
            stmts: array.into_iter().map(Stmt::Expr).collect(),
            ..Default::default()
        }
    }
}

impl<const N: usize> From<[Expr; N]> for Block {
    /// Converts an array of `Expr` into a `Block`.
    fn from(array: [Expr; N]) -> Self {
        Block {
            stmts: array.into_iter().map(Stmt::Expr).collect(),
            ..Default::default()
        }
    }
}

/// Creates a new `AsmBuilder` to construct an `asm!` item.
pub fn asm_item(template: impl Into<LitStr>) -> AsmBuilder {
    AsmBuilder::new(template)
}

/// A builder for constructing an `ItemAsm` AST node.
pub struct AsmBuilder {
    template: ThinVec<LitStr>,
    operands: ThinVec<AsmOperand>,
    options: Option<AsmOptions>,
}

impl AsmBuilder {
    /// Creates a new `AsmBuilder` with the given template.
    ///
    /// # Parameters
    ///
    /// - `template`: The initial template string for the `asm!` item.
    pub fn new(template: impl Into<LitStr>) -> Self {
        Self {
            template: thin_vec![template.into()],
            operands: thin_vec![],
            options: None,
        }
    }

    /// Adds a template string to the `asm!` item.
    ///
    /// # Parameters
    ///
    /// - `template`: The template string to add.
    pub fn template(mut self, template: impl Into<LitStr>) -> Self {
        self.template.push(template.into());
        self
    }

    /// Adds an operand to the `asm!` item.
    ///
    /// # Parameters
    ///
    /// - `operand`: The `AsmOperand` to add.
    pub fn operand(mut self, operand: impl Into<AsmOperand>) -> Self {
        self.operands.push(operand.into());
        self
    }

    /// Sets the options for the `asm!` item.
    ///
    /// # Parameters
    ///
    /// - `options`: The `AsmOptions` to set.
    pub fn options(mut self, options: AsmOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Builds the `ItemAsm` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemAsm` instance.
    pub fn build(self) -> ItemAsm {
        ItemAsm {
            template: self.template,
            operands: self.operands,
            options: self.options,
        }
    }
}

/// Creates a new `AsmOperandBuilder` to construct an `AsmOperand`.
pub fn asm_operand() -> AsmOperandBuilder {
    AsmOperandBuilder
}

/// A builder for constructing `AsmOperand` AST nodes.
#[derive(Clone, Copy)]
pub struct AsmOperandBuilder;

impl AsmOperandBuilder {
    /// Creates a register operand.
    ///
    /// # Parameters
    ///
    /// - `direction`: The direction of the operand (`in`, `out`, etc.).
    /// - `reg`: The register specifier.
    /// - `expr`: The expression for the operand.
    pub fn reg(self, direction: AsmDirection, reg: RegSpec, expr: Expr) -> RegOperandBuilder {
        RegOperandBuilder::new(direction, reg, expr)
    }

    /// Creates a `sym` operand.
    ///
    /// # Parameters
    ///
    /// - `path`: The `Path` to the symbol.
    pub fn sym(self, path: Path) -> AsmOperand {
        AsmOperand::Sym(path)
    }

    /// Creates a `const` operand.
    ///
    /// # Parameters
    ///
    /// - `expr`: The `Expr` for the constant value.
    pub fn const_(self, expr: Expr) -> AsmOperand {
        AsmOperand::Const(expr)
    }

    /// Creates a `clobber_abi` operand.
    ///
    /// # Parameters
    ///
    /// - `abi`: The ABI to clobber.
    pub fn clobber_abi(self, abi: impl Into<LitStr>) -> ClobberAbiBuilder {
        ClobberAbiBuilder::new(abi)
    }
}

/// A builder for constructing a `RegOperand` AST node.
pub struct RegOperandBuilder {
    direction: AsmDirection,
    reg: RegSpec,
    expr: Expr,
    out_expr: Option<Expr>,
}

impl RegOperandBuilder {
    /// Creates a new `RegOperandBuilder`.
    ///
    /// # Parameters
    ///
    /// - `direction`: The direction of the operand.
    /// - `reg`: The register specifier.
    /// - `expr`: The expression for the operand.
    pub fn new(direction: AsmDirection, reg: RegSpec, expr: Expr) -> Self {
        Self {
            direction,
            reg,
            expr,
            out_expr: None,
        }
    }

    /// Sets the output expression for an `inout` operand.
    ///
    /// # Parameters
    ///
    /// - `expr`: The output `Expr`.
    pub fn out_expr(mut self, expr: Expr) -> Self {
        self.out_expr = Some(expr);
        self
    }

    /// Builds the `RegOperand` AST node.
    ///
    /// # Returns
    ///
    /// An `AsmOperand` instance representing the register operand.
    pub fn build(self) -> AsmOperand {
        AsmOperand::Reg(RegOperand {
            direction: self.direction,
            reg: self.reg,
            expr: self.expr,
            out_expr: self.out_expr,
        })
    }
}

impl From<RegOperandBuilder> for AsmOperand {
    fn from(builder: RegOperandBuilder) -> Self {
        builder.build()
    }
}

/// A builder for constructing a `ClobberAbi` AST node.
pub struct ClobberAbiBuilder {
    abis: ThinVec<LitStr>,
}

impl ClobberAbiBuilder {
    /// Creates a new `ClobberAbiBuilder`.
    ///
    /// # Parameters
    ///
    /// - `abi`: The initial ABI to clobber.
    pub fn new(abi: impl Into<LitStr>) -> Self {
        Self {
            abis: thin_vec![abi.into()],
        }
    }

    /// Adds an ABI to the list of clobbered ABIs.
    ///
    /// # Parameters
    ///
    /// - `abi`: The ABI to add.
    pub fn abi(mut self, abi: impl Into<LitStr>) -> Self {
        self.abis.push(abi.into());
        self
    }

    /// Builds the `ClobberAbi` AST node.
    ///
    /// # Returns
    ///
    /// An `AsmOperand` instance representing the `clobber_abi`.
    pub fn build(self) -> AsmOperand {
        AsmOperand::ClobberAbi(ClobberAbi { abis: self.abis })
    }
}

impl From<ClobberAbiBuilder> for AsmOperand {
    fn from(builder: ClobberAbiBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new `AsmOptionsBuilder` to construct an `AsmOptions` AST node.
pub fn asm_options() -> AsmOptionsBuilder {
    AsmOptionsBuilder::new()
}

/// A builder for constructing an `AsmOptions` AST node.
#[derive(Default)]
pub struct AsmOptionsBuilder {
    options: ThinVec<AsmOption>,
}

impl AsmOptionsBuilder {
    /// Creates a new `AsmOptionsBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an option to the `asm!` options.
    ///
    /// # Parameters
    ///
    /// - `option`: The `AsmOption` to add.
    pub fn option(mut self, option: AsmOption) -> Self {
        self.options.push(option);
        self
    }

    /// Builds the `AsmOptions` AST node.
    ///
    /// # Returns
    ///
    /// An `AsmOptions` instance.
    pub fn build(self) -> AsmOptions {
        AsmOptions {
            options: self.options,
        }
    }
}
