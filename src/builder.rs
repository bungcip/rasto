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

use crate::ast::*;
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

    /// Builds the `File` AST node.
    ///
    /// # Returns
    ///
    /// A `File` instance.
    pub fn build(self) -> File {
        File { items: self.items }
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
    /// Creates a line comment.
    pub fn line<S: Into<String>>(self, content: S) -> Comment {
        Comment::Line(content.into())
    }

    /// Creates a block comment.
    pub fn block<S: Into<String>>(self, content: S) -> Comment {
        Comment::Block(content.into())
    }

    /// Creates a doc comment.
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
pub fn trait_def(name: impl Into<String>) -> TraitBuilder {
    TraitBuilder::new(name)
}

/// A builder for constructing an `ItemTrait` (trait definition) AST node.
pub struct TraitBuilder {
    ident: String,
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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            generics: GenericParams::new(),
            associated_types: thin_vec![],
            items: thin_vec![],
            md: MdBuilder::new(),
        }
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

    /// Adds a leading comment to the trait.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the trait.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the trait.
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
            ident: self.ident,
            generics: self.generics,
            associated_types: self.associated_types,
            items: self.items,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `AssociatedTypeBuilder` to construct an associated type for traits.
pub fn associated_type(ident: impl Into<String>) -> AssociatedTypeBuilder {
    AssociatedTypeBuilder::new(ident)
}

/// A builder for constructing an `AssociatedType` AST node.
pub struct AssociatedTypeBuilder {
    ident: String,
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
    pub fn new(ident: impl Into<String>) -> Self {
        Self {
            ident: ident.into(),
            generics: GenericParams::new(),
            bounds: thin_vec![],
            default: None,
            md: None,
        }
    }

    /// Add a generic parameter to the associated type.
    pub fn generic(mut self, g: impl Into<GenericParam>) -> Self {
        self.generics.params.push(g.into());
        self
    }

    /// Add a bound type to the associated type.
    pub fn bound(mut self, t: impl Into<Type>) -> Self {
        self.bounds.push(t.into());
        self
    }

    /// Set a default type for the associated type.
    pub fn default(mut self, t: impl Into<Type>) -> Self {
        self.default = Some(t.into());
        self
    }

    /// Set metadata for the associated type.
    pub fn md(mut self, md: impl Into<Md>) -> Self {
        self.md = Some(Box::new(md.into()));
        self
    }

    /// Build the `AssociatedType` instance.
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
    leading_comments: ThinVec<Comment>,
    trailing_comments: ThinVec<Comment>,
}

impl BlockBuilder {
    /// Creates a new, empty `BlockBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a leading comment to the block.
    ///
    /// # Parameters
    ///
    /// - `comment`: The comment to add.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    /// Adds a statement to the block.
    ///
    /// # Parameters
    ///
    /// - `stmt`: The statement to add.
    pub fn statement(mut self, stmt: impl Into<Stmt>) -> Self {
        self.stmts.push(stmt.into());
        self
    }

    /// Sets whether the block has a trailing semicolon.
    ///
    /// # Parameters
    ///
    /// - `has_trailing_semicolon`: Whether the block has a trailing semicolon.
    pub fn has_trailing_semicolon(mut self, has_trailing_semicolon: bool) -> Self {
        self.has_trailing_semicolon = has_trailing_semicolon;
        self
    }

    /// Adds a trailing comment to the block.
    ///
    /// # Parameters
    ///
    /// - `comment`: The comment to add.
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
        let md = if !self.leading_comments.is_empty() || !self.trailing_comments.is_empty() {
            let mut md_builder = MdBuilder::new();
            for comment in self.leading_comments {
                md_builder = md_builder.leading_comment(comment);
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
            leading_comments: Default::default(),
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
    fns: ThinVec<ItemFn>,
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
            fns: thin_vec![],
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
    pub fn unsafe_(mut self) -> Self {
        self.is_unsafe = true;
        self
    }

    /// Marks the impl block as negative.
    pub fn negative(mut self) -> Self {
        self.is_negative = true;
        self
    }

    /// Adds a function to the impl block.
    ///
    /// # Parameters
    ///
    /// - `func`: The function to add.
    pub fn function(mut self, func: ItemFn) -> Self {
        self.fns.push(func);
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
            fns: self.fns,
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
pub fn enum_def(name: impl Into<String>) -> EnumBuilder {
    EnumBuilder::new(name)
}

/// A builder for constructing an `ItemEnum` (enum definition) AST node.
pub struct EnumBuilder {
    ident: String,
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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            generics: GenericParams::new(),
            variants: thin_vec![],
            md: MdBuilder::new(),
        }
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
    pub fn variant(mut self, name: impl Into<String>) -> Self {
        self.variants.push(Variant {
            ident: name.into(),
            md: None,
        });
        self
    }

    /// Adds a leading comment to the enum.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the enum.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the enum.
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
pub fn struct_def(name: impl Into<String>) -> StructBuilder {
    StructBuilder::new(name)
}

/// A builder for constructing an `ItemStruct` (struct definition) AST node.
pub struct StructBuilder {
    ident: String,
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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            generics: GenericParams::new(),
            fields: thin_vec![],
            md: MdBuilder::new(),
        }
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
    pub fn field(mut self, name: impl Into<String>, ty: impl Into<Type>) -> Self {
        self.fields.push(Field {
            ident: name.into(),
            ty: ty.into(),
            md: None,
        });
        self
    }

    /// Adds a leading comment to the struct.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the struct.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the struct.
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
            ident: self.ident,
            generics: self.generics,
            fields: self.fields,
            md: Some(Box::new(self.md.build())),
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
pub fn fn_def(name: impl Into<String>) -> FnBuilder {
    FnBuilder::new(name)
}

/// A builder for constructing an `ItemFn` (function definition) AST node.
#[derive(Default)]
pub struct FnBuilder {
    ident: String,
    generics: GenericParams,
    inputs: ThinVec<Pat>,
    output: Option<Type>,
    block: Block,
    md: MdBuilder,
}

impl FnBuilder {
    /// Creates a new `FnBuilder` with the given function name.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the function.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            ..Default::default()
        }
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

    /// Sets the return type of the function.
    ///
    /// # Parameters
    ///
    /// - `ty`: The return type.
    pub fn output(mut self, ty: impl Into<Type>) -> Self {
        self.output = Some(ty.into());
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

    /// has trailing semicolon
    pub fn has_trailing_semicolon(mut self, value: bool) -> Self {
        self.block.has_trailing_semicolon = value;
        self
    }

    /// insert statement
    pub fn statement(mut self, stmt: impl Into<Stmt>) -> Self {
        self.block.stmts.push(stmt.into());
        self
    }

    /// Adds an attribute to the function.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Adds a leading comment to the function.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the function.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
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
            sig: Signature {
                ident: self.ident,
                generics: self.generics,
                inputs: self.inputs,
                output: self.output,
            },
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
    pub fn local(self, pat: impl Into<Pat>) -> LocalBuilder {
        LocalBuilder::new(pat)
    }

    /// Creates an item statement.
    pub fn item(self, item: impl Into<Item>) -> Stmt {
        Stmt::Item(item.into())
    }

    /// Creates an expression statement.
    pub fn expr(self, expr: Expr) -> Stmt {
        Stmt::Expr(expr)
    }

    /// Creates a macro call statement.
    pub fn mac_call(self, mac: ExprMacroCall) -> Stmt {
        Stmt::MacCall(mac)
    }
}

/// A builder for constructing a `Local` (let) AST node.
pub struct LocalBuilder {
    pat: Pat,
    ty: Option<Type>,
    expr: Option<Expr>,
}

impl LocalBuilder {
    /// Creates a new `LocalBuilder` with the given pattern.
    pub fn new(pat: impl Into<Pat>) -> Self {
        Self {
            pat: pat.into(),
            ty: None,
            expr: None,
        }
    }

    /// Sets the type of the variable.
    pub fn ty(mut self, ty: impl Into<Type>) -> Self {
        self.ty = Some(ty.into());
        self
    }

    /// Sets the expression to initialize the variable.
    pub fn expr(mut self, expr: impl Into<Expr>) -> Self {
        self.expr = Some(expr.into());
        self
    }

    /// Builds the `Stmt::Local` AST node.
    pub fn build(self) -> Stmt {
        Stmt::Local(Local {
            pat: self.pat,
            ty: self.ty,
            expr: self.expr,
        })
    }
}

/// Creates a new `FieldValueBuilder` to construct a field-value pair.
pub fn field_value(member: impl Into<String>, value: impl Into<Expr>) -> FieldValue {
    FieldValue {
        member: member.into(),
        value: value.into(),
    }
}

/// Creates a new `TraitItemFnBuilder` to construct a trait item function.
pub fn trait_item_fn(name: impl Into<String>) -> TraitItemFn {
    TraitItemFn {
        sig: Signature {
            ident: name.into(),
            generics: Default::default(),
            inputs: thin_vec![],
            output: None,
        },
        block: None,
        md: None,
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
    /// Creates a wildcard pattern (`_`).
    pub fn wild(self) -> Pat {
        Pat::Wild
    }

    /// set mutability of pattern
    pub fn mutable(mut self) -> Self {
        self.mutability = true;
        self
    }

    /// Creates an identifier pattern.
    pub fn ident(self, name: impl Into<String>) -> Pat {
        Pat::Ident(PatIdent {
            ident: name.into(),
            is_mut: self.mutability,
        })
    }

    /// Creates a tuple pattern.
    pub fn tuple(self, pats: impl IntoIterator<Item = Pat>) -> Pat {
        Pat::Tuple(pats.into_iter().collect())
    }

    /// Creates a rest pattern (`..`).
    pub fn rest(self) -> Pat {
        Pat::Rest
    }
}

/// Creates a new `PathBuilder` to construct a path.
pub fn path(segment: impl Into<String>) -> PathBuilder {
    PathBuilder::new(segment)
}

/// A builder for constructing a `Path` AST node.
pub struct PathBuilder {
    segments: ThinVec<PathSegment>,
}

impl PathBuilder {
    /// Creates a new `PathBuilder` with the given segment.
    pub fn new(segment: impl Into<String>) -> Self {
        Self {
            segments: thin_vec![PathSegment {
                ident: segment.into(),
                args: None,
            }],
        }
    }

    /// Adds a segment to the path.
    pub fn segment(mut self, segment: impl Into<String>) -> Self {
        self.segments.push(PathSegment {
            ident: segment.into(),
            args: None,
        });
        self
    }

    /// Builds the `Path` AST node.
    pub fn build(self) -> Path {
        Path {
            segments: self.segments,
        }
    }

    /// Adds a generic argument to the last segment.
    pub fn generic(mut self, arg: impl Into<GenericArg>) -> Self {
        let segment = self.segments.last_mut().unwrap();
        let args = segment.args.get_or_insert_with(Default::default);
        args.args.push(arg.into());
        self
    }

    /// Builds a `Type::Path` from the `PathBuilder`.
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
    pub fn async_block(self, block: Block) -> Expr {
        Expr::Async(ExprAsync { block })
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
    pub fn block(self, block: Block) -> Expr {
        Expr::Block(ExprBlock { block })
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
    pub fn const_block(self, block: Block) -> Expr {
        Expr::Const(ExprConst { block })
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
    pub fn field(self, expr: Expr, member: impl Into<String>) -> Expr {
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
    pub fn for_loop(self, pat: impl Into<Pat>, expr: Expr, body: Block) -> Expr {
        Expr::For(ExprFor {
            pat: pat.into(),
            expr: Box::new(expr),
            body,
        })
    }

    /// Creates an `if` expression.
    ///
    /// # Parameters
    ///
    /// - `cond`: The condition expression.
    /// - `then_branch`: The block to execute if the condition is true.
    /// - `else_branch`: An optional `else` branch.
    pub fn if_expr(self, cond: Expr, then_branch: Block, else_branch: Option<Expr>) -> Expr {
        Expr::If(ExprIf {
            cond: Box::new(cond),
            then_branch,
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

    /// Creates a literal expression.
    ///
    /// # Parameters
    ///
    /// - `lit`: The literal value.
    pub fn lit(self, lit: impl Into<Lit>) -> Expr {
        Expr::Lit(lit.into())
    }

    /// create int literal with suffix
    pub fn int_lit_with_suffix(self, value: i32, suffix: IntSuffix) -> Expr {
        Expr::Lit(Lit::Int(LitInt::with_suffix(value as u128, suffix)))
    }

    /// create float literal with suffix
    pub fn float_lit_with_suffix(self, value: &str, suffix: FloatSuffix) -> Expr {
        Expr::Lit(Lit::Float(LitFloat::with_suffix(value, suffix)))
    }

    /// Creates a `loop` expression.
    ///
    /// # Parameters
    ///
    /// - `body`: The body of the loop.
    pub fn loop_expr(self, body: Block) -> Expr {
        Expr::Loop(ExprLoop { body })
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
        method: impl Into<String>,
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
        path: impl Into<String>,
        fields: impl IntoIterator<Item = FieldValue>,
    ) -> Expr {
        Expr::Struct(ExprStruct {
            path: path.into(),
            fields: fields.into_iter().collect(),
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
    pub fn while_loop(self, cond: Expr, body: Block) -> Expr {
        Expr::While(ExprWhile {
            cond: Box::new(cond),
            body,
        })
    }
}

impl From<Expr> for Stmt {
    fn from(value: Expr) -> Stmt {
        Stmt::Expr(value)
    }
}

/// Creates a new `ItemDefBuilder` to construct a `const`, `static`, or `type` item.
pub fn def_item(name: impl Into<String>, kind: impl Into<ItemDefKind>) -> ItemDefBuilder {
    ItemDefBuilder::new(name, kind)
}

/// A builder for constructing an `ItemDef` AST node.
pub struct ItemDefBuilder {
    ident: String,
    kind: ItemDefKind,
    md: MdBuilder,
}

impl ItemDefBuilder {
    /// Creates a new `ItemDefBuilder`.
    pub fn new(name: impl Into<String>, kind: impl Into<ItemDefKind>) -> Self {
        Self {
            ident: name.into(),
            kind: kind.into(),
            md: MdBuilder::new(),
        }
    }

    /// Adds a leading comment to the item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemDef` AST node.
    pub fn build(self) -> ItemDef {
        ItemDef {
            ident: self.ident,
            kind: self.kind,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ConstKindBuilder` to construct an `ItemDefKind::Const`.
pub fn const_kind(ty: impl Into<Type>, expr: impl Into<Expr>) -> ConstKindBuilder {
    ConstKindBuilder::new(ty, expr)
}

/// A builder for `ItemDefKind::Const`.
pub struct ConstKindBuilder {
    ty: Type,
    expr: Box<Expr>,
}

impl ConstKindBuilder {
    /// Creates a new `ConstKindBuilder`.
    pub fn new(ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ty: ty.into(),
            expr: Box::new(expr.into()),
        }
    }

    /// Builds the `ItemDefKind::Const`.
    pub fn build(self) -> ItemDefKind {
        ItemDefKind::Const {
            ty: self.ty,
            expr: self.expr,
        }
    }
}

impl From<ConstKindBuilder> for ItemDefKind {
    fn from(builder: ConstKindBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new `StaticKindBuilder` to construct an `ItemDefKind::Static`.
pub fn static_kind(ty: impl Into<Type>, expr: impl Into<Expr>) -> StaticKindBuilder {
    StaticKindBuilder::new(ty, expr)
}

/// A builder for `ItemDefKind::Static`.
pub struct StaticKindBuilder {
    ty: Type,
    expr: Box<Expr>,
}

impl StaticKindBuilder {
    /// Creates a new `StaticKindBuilder`.
    pub fn new(ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ty: ty.into(),
            expr: Box::new(expr.into()),
        }
    }

    /// Builds the `ItemDefKind::Static`.
    pub fn build(self) -> ItemDefKind {
        ItemDefKind::Static {
            ty: self.ty,
            expr: self.expr,
        }
    }
}

impl From<StaticKindBuilder> for ItemDefKind {
    fn from(builder: StaticKindBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new `TypeAliasKindBuilder` to construct an `ItemDefKind::TypeAlias`.
pub fn type_alias_kind(ty: impl Into<Type>) -> TypeAliasKindBuilder {
    TypeAliasKindBuilder::new(ty)
}

/// A builder for `ItemDefKind::TypeAlias`.
pub struct TypeAliasKindBuilder {
    generics: GenericParams,
    ty: Type,
}

impl TypeAliasKindBuilder {
    /// Creates a new `TypeAliasKindBuilder`.
    pub fn new(ty: impl Into<Type>) -> Self {
        Self {
            generics: GenericParams::new(),
            ty: ty.into(),
        }
    }

    /// Adds a generic parameter to the type alias.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Builds the `ItemDefKind::TypeAlias`.
    pub fn build(self) -> ItemDefKind {
        ItemDefKind::TypeAlias {
            generics: self.generics,
            ty: self.ty,
        }
    }
}

impl From<TypeAliasKindBuilder> for ItemDefKind {
    fn from(builder: TypeAliasKindBuilder) -> Self {
        builder.build()
    }
}

/// Creates a new `ItemExternCrateBuilder` to construct an `extern crate` item.
pub fn extern_crate_item(name: impl Into<String>) -> ItemExternCrateBuilder {
    ItemExternCrateBuilder::new(name)
}

/// A builder for constructing an `ItemExternCrate` AST node.
pub struct ItemExternCrateBuilder {
    ident: String,
    md: MdBuilder,
}

impl ItemExternCrateBuilder {
    /// Creates a new `ItemExternCrateBuilder`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            md: MdBuilder::new(),
        }
    }

    /// Adds a leading comment to the `extern crate` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the `extern crate` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the `extern crate` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemExternCrate` AST node.
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
    pub fn new(abi: impl Into<String>) -> Self {
        Self {
            abi: abi.into(),
            items: thin_vec![],
            md: MdBuilder::new(),
        }
    }

    /// Adds an item to the foreign module.
    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds a leading comment to the foreign module.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the foreign module.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the foreign module.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemForeignMod` AST node.
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
    pub fn new(expr: impl Into<Expr>) -> Self {
        Self {
            expr: expr.into(),
            md: MdBuilder::new(),
        }
    }

    /// Adds a leading comment to the macro item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the macro item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the macro item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemMacro` AST node.
    pub fn build(self) -> ItemMacro {
        ItemMacro {
            expr: Box::new(self.expr),
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemModBuilder` to construct a module item.
pub fn mod_item(name: impl Into<String>) -> ItemModBuilder {
    ItemModBuilder::new(name)
}

/// A builder for constructing an `ItemMod` AST node.
pub struct ItemModBuilder {
    ident: String,
    content: Option<ThinVec<Item>>,
    md: MdBuilder,
}

impl ItemModBuilder {
    /// Creates a new `ItemModBuilder`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            content: None,
            md: MdBuilder::new(),
        }
    }

    /// Sets the content of the module.
    pub fn content(mut self, content: ThinVec<Item>) -> Self {
        self.content = Some(content);
        self
    }

    /// Adds a leading comment to the module item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the module item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the module item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemMod` AST node.
    pub fn build(self) -> ItemMod {
        ItemMod {
            ident: self.ident,
            content: self.content,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemTraitAliasBuilder` to construct a trait alias.
pub fn trait_alias_item(name: impl Into<String>, bounds: ThinVec<String>) -> ItemTraitAliasBuilder {
    ItemTraitAliasBuilder::new(name, bounds)
}

/// A builder for constructing an `ItemTraitAlias` AST node.
pub struct ItemTraitAliasBuilder {
    ident: String,
    bounds: ThinVec<String>,
    md: MdBuilder,
}

impl ItemTraitAliasBuilder {
    /// Creates a new `ItemTraitAliasBuilder`.
    pub fn new(name: impl Into<String>, bounds: ThinVec<String>) -> Self {
        Self {
            ident: name.into(),
            bounds,
            md: MdBuilder::new(),
        }
    }

    /// Adds a leading comment to the trait alias.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the trait alias.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the trait alias.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemTraitAlias` AST node.
    pub fn build(self) -> ItemTraitAlias {
        ItemTraitAlias {
            ident: self.ident,
            bounds: self.bounds,
            md: Some(Box::new(self.md.build())),
        }
    }
}

/// Creates a new `ItemUnionBuilder` to construct a `union` item.
pub fn union_item(name: impl Into<String>) -> ItemUnionBuilder {
    ItemUnionBuilder::new(name)
}

/// A builder for constructing an `ItemUnion` AST node.
pub struct ItemUnionBuilder {
    ident: String,
    fields: ThinVec<Field>,
    generics: GenericParams,
    md: MdBuilder,
}

impl ItemUnionBuilder {
    /// Creates a new `ItemUnionBuilder`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            generics: GenericParams::new(),
            fields: thin_vec![],
            md: MdBuilder::new(),
        }
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
    pub fn field(mut self, name: impl Into<String>, ty: impl Into<Type>) -> Self {
        self.fields.push(Field {
            ident: name.into(),
            ty: ty.into(),
            md: None,
        });
        self
    }

    /// Adds a leading comment to the `union` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the `union` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the `union` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemUnion` AST node.
    pub fn build(self) -> ItemUnion {
        ItemUnion {
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
    md: MdBuilder,
}

impl ItemUseBuilder {
    /// Creates a new `ItemUseBuilder`.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            md: MdBuilder::new(),
        }
    }

    /// Adds a leading comment to the `use` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.leading_comment(comment.into());
        self
    }

    /// Adds a trailing comment to the `use` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md = self.md.trailing_comment(comment.into());
        self
    }

    /// Adds an attribute to the `use` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md = self.md.attr(attr.into());
        self
    }

    /// Builds the `ItemUse` AST node.
    pub fn build(self) -> ItemUse {
        ItemUse {
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

    /// Sets the attribute as an inner attribute.
    pub fn inner(mut self) -> Self {
        self.is_inner = true;
        self
    }

    /// Sets the meta item for the attribute.
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
    /// Creates a meta list.
    pub fn list(self, path: impl Into<String>, metas: impl IntoIterator<Item = Meta>) -> Meta {
        Meta::List(MetaList {
            path: path.into(),
            metas: metas.into_iter().collect(),
        })
    }

    /// Creates a meta path.
    pub fn path(self, path: impl Into<String>) -> Meta {
        Meta::Path(path.into())
    }

    /// Creates a meta name-value pair.
    pub fn name_value(self, path: impl Into<String>, value: impl Into<Lit>) -> Meta {
        Meta::NameValue(MetaNameValue {
            path: path.into(),
            value: value.into(),
        })
    }
}

/// token tree builder
pub fn tt() -> TokenTreeBuilder {
    TokenTreeBuilder {}
}

/// A builder for constructing `TokenTree` AST nodes.
#[derive(Clone, Copy, Default)]
pub struct TokenTreeBuilder;

impl TokenTreeBuilder {
    /// create TokenTree::Literal
    pub fn lit(self, value: impl Into<Lit>) -> TokenTree {
        TokenTree::Literal(value.into())
    }

    /// create TokenTree::Ident
    pub fn ident(self, value: impl Into<String>) -> TokenTree {
        TokenTree::Ident(value.into())
    }

    /// create TokenTree::Punct
    pub fn punct(self, ch: char, spacing: Spacing) -> TokenTree {
        TokenTree::Punct(Punct { ch, spacing })
    }
}

impl From<LocalBuilder> for Stmt {
    fn from(value: LocalBuilder) -> Self {
        value.build()
    }
}

impl From<PathBuilder> for Path {
    fn from(builder: PathBuilder) -> Self {
        builder.build()
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        path(value).build()
    }
}

impl<const N: usize> From<&[&str; N]> for Path {
    fn from(array: &[&str; N]) -> Self {
        let array: ThinVec<PathSegment> = array
            .iter()
            .map(|x| PathSegment {
                ident: x.to_string(),
                args: None,
            })
            .collect();
        Path { segments: array }
    }
}

impl From<&str> for Pat {
    fn from(val: &str) -> Self {
        Pat::Ident(PatIdent {
            ident: val.into(),
            is_mut: false,
        })
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
    pub fn new(template: impl Into<LitStr>) -> Self {
        Self {
            template: thin_vec![template.into()],
            operands: thin_vec![],
            options: None,
        }
    }

    /// Adds a template string to the `asm!` item.
    pub fn template(mut self, template: impl Into<LitStr>) -> Self {
        self.template.push(template.into());
        self
    }

    /// Adds an operand to the `asm!` item.
    pub fn operand(mut self, operand: impl Into<AsmOperand>) -> Self {
        self.operands.push(operand.into());
        self
    }

    /// Sets the options for the `asm!` item.
    pub fn options(mut self, options: AsmOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Builds the `ItemAsm` AST node.
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
    pub fn reg(self, direction: AsmDirection, reg: RegSpec, expr: Expr) -> RegOperandBuilder {
        RegOperandBuilder::new(direction, reg, expr)
    }

    /// Creates a `sym` operand.
    pub fn sym(self, path: Path) -> AsmOperand {
        AsmOperand::Sym(path)
    }

    /// Creates a `const` operand.
    pub fn const_(self, expr: Expr) -> AsmOperand {
        AsmOperand::Const(expr)
    }

    /// Creates a `clobber_abi` operand.
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
    pub fn new(direction: AsmDirection, reg: RegSpec, expr: Expr) -> Self {
        Self {
            direction,
            reg,
            expr,
            out_expr: None,
        }
    }

    /// Sets the output expression for an `inout` operand.
    pub fn out_expr(mut self, expr: Expr) -> Self {
        self.out_expr = Some(expr);
        self
    }

    /// Builds the `RegOperand` AST node.
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
    pub fn new(abi: impl Into<LitStr>) -> Self {
        Self {
            abis: thin_vec![abi.into()],
        }
    }

    /// Adds an ABI to the list of clobbered ABIs.
    pub fn abi(mut self, abi: impl Into<LitStr>) -> Self {
        self.abis.push(abi.into());
        self
    }

    /// Builds the `ClobberAbi` AST node.
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

    /// Adds an option.
    pub fn option(mut self, option: AsmOption) -> Self {
        self.options.push(option);
        self
    }

    /// Builds the `AsmOptions` AST node.
    pub fn build(self) -> AsmOptions {
        AsmOptions {
            options: self.options,
        }
    }
}
