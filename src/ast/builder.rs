//! Provides a builder API for constructing AST nodes programmatically.
//!
//! The builder pattern allows for a more fluent and readable way of creating complex AST structures.
//! Each builder corresponds to a specific AST node and provides methods for setting its properties.
//!
//! # Examples
//!
//! ```
//! use rasto::ast::builder::*;
//! use rasto::ast::{*, Lit, LitInt};
//! use thin_vec::thin_vec;
//!
//! let file_ast = file()
//!     .item(
//!         fn_def("my_function")
//!             .block(Block {
//!                 leading_comments: thin_vec![],
//!                 stmts: thin_vec![Stmt::Expr(expr().lit(Lit::Int(LitInt::new(42))), true)],
//!                 trailing_comments: thin_vec![],
//!             })
//!             .build(),
//!     )
//!     .build();
//! ```

use crate::ast::*;
use thin_vec::{thin_vec, ThinVec};

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
    items: Vec<TraitItem>,
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
            items: vec![],
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

    /// Builds the `ItemTrait` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemTrait` instance.
    pub fn build(self) -> ItemTrait {
        ItemTrait {
            attrs: vec![],
            leading_comments: vec![],
            ident: self.ident,
            generics: self.generics,
            items: self.items,
            trailing_comments: vec![],
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
    fns: Vec<ItemFn>,
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
            fns: vec![],
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
            attrs: vec![],
            leading_comments: vec![],
            generics: self.generics,
            ty: self.ty,
            fns: self.fns,
            trailing_comments: vec![],
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
    variants: Vec<Variant>,
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
            variants: vec![],
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
            attrs: vec![],
            ident: name.into(),
        });
        self
    }

    /// Builds the `ItemEnum` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemEnum` instance.
    pub fn build(self) -> ItemEnum {
        ItemEnum {
            attrs: vec![],
            leading_comments: vec![],
            ident: self.ident,
            generics: self.generics,
            variants: self.variants,
            trailing_comments: vec![],
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
    fields: Vec<Field>,
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
            fields: vec![],
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
            attrs: vec![],
            ident: name.into(),
            ty: ty.into(),
        });
        self
    }

    /// Builds the `ItemStruct` AST node.
    ///
    /// # Returns
    ///
    /// An `ItemStruct` instance.
    pub fn build(self) -> ItemStruct {
        ItemStruct {
            attrs: vec![],
            leading_comments: vec![],
            ident: self.ident,
            generics: self.generics,
            fields: self.fields,
            trailing_comments: vec![],
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
pub struct FnBuilder {
    ident: String,
    generics: GenericParams,
    inputs: ThinVec<Pat>,
    output: Option<Type>,
    block: Option<Block>,
    md: Option<Box<Md>>,
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
            inputs: vec![],
            output: None,
            block: None,
            md: None,
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
    pub fn block(mut self, block: Block) -> Self {
        self.block = Some(block);
        self
    }

    /// Adds an attribute to the function.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Adds a leading comment to the function.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the function.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
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
        let block = self.block.expect("block is required");

        ItemFn {
            sig: Signature {
                ident: self.ident,
                generics: self.generics,
                inputs: self.inputs,
                output: self.output,
            },
            block,
            md: self.md,
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
    pub fn expr(self, expr: Expr, semi: bool) -> Stmt {
        Stmt::Expr(expr, semi)
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

/// Creates a new `PatBuilder` to construct patterns.
pub fn pat() -> PatBuilder {
    PatBuilder
}

/// A builder for constructing `Pat` AST nodes.
#[derive(Clone, Copy)]
pub struct PatBuilder;

impl PatBuilder {
    /// Creates a wildcard pattern (`_`).
    pub fn wild(self) -> Pat {
        Pat::Wild
    }

    /// Creates an identifier pattern.
    pub fn ident(self, name: impl Into<String>, is_mut: bool) -> Pat {
        Pat::Ident(PatIdent {
            ident: name.into(),
            is_mut,
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
    /// - `ident`: The name of the macro.
    /// - `tokens`: The token stream passed to the macro.
    pub fn macro_call(self, ident: impl Into<String>, tokens: TokenStream) -> Expr {
        Expr::MacroCall(ExprMacroCall {
            ident: ident.into(),
            tokens,
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

/// Creates a new `ItemConstBuilder` to construct a `const` item.
pub fn const_item(
    name: impl Into<String>,
    ty: impl Into<Type>,
    expr: impl Into<Expr>,
) -> ItemConstBuilder {
    ItemConstBuilder::new(name, ty, expr)
}

/// A builder for constructing an `ItemConst` AST node.
pub struct ItemConstBuilder {
    ident: String,
    ty: Type,
    expr: Box<Expr>,
    md: Option<Box<Md>>,
}

impl ItemConstBuilder {
    /// Creates a new `ItemConstBuilder`.
    pub fn new(name: impl Into<String>, ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            expr: Box::new(expr.into()),
            md: None,
        }
    }

    /// Adds a leading comment to the `const` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the `const` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the `const` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemConst` AST node.
    pub fn build(self) -> ItemConst {
        ItemConst {
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            md: self.md,
        }
    }
}

/// Creates a new `ItemExternCrateBuilder` to construct an `extern crate` item.
pub fn extern_crate_item(name: impl Into<String>) -> ItemExternCrateBuilder {
    ItemExternCrateBuilder::new(name)
}

/// A builder for constructing an `ItemExternCrate` AST node.
pub struct ItemExternCrateBuilder {
    ident: String,
    md: Option<Box<Md>>,
}

impl ItemExternCrateBuilder {
    /// Creates a new `ItemExternCrateBuilder`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            md: None,
        }
    }

    /// Adds a leading comment to the `extern crate` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the `extern crate` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the `extern crate` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemExternCrate` AST node.
    pub fn build(self) -> ItemExternCrate {
        ItemExternCrate {
            ident: self.ident,
            md: self.md,
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
    md: Option<Box<Md>>,
}

impl ItemForeignModBuilder {
    /// Creates a new `ItemForeignModBuilder`.
    pub fn new(abi: impl Into<String>) -> Self {
        Self {
            abi: abi.into(),
            items: thin_vec![],
            md: None,
        }
    }

    /// Adds an item to the foreign module.
    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Adds a leading comment to the foreign module.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the foreign module.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the foreign module.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemForeignMod` AST node.
    pub fn build(self) -> ItemForeignMod {
        ItemForeignMod {
            abi: self.abi,
            items: self.items,
            md: self.md,
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
    md: Option<Box<Md>>,
}

impl ItemMacroBuilder {
    /// Creates a new `ItemMacroBuilder`.
    pub fn new(expr: impl Into<Expr>) -> Self {
        Self {
            expr: expr.into(),
            md: None,
        }
    }

    /// Adds a leading comment to the macro item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the macro item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the macro item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemMacro` AST node.
    pub fn build(self) -> ItemMacro {
        ItemMacro {
            expr: Box::new(self.expr),
            md: self.md,
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
    md: Option<Box<Md>>,
}

impl ItemModBuilder {
    /// Creates a new `ItemModBuilder`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            content: None,
            md: None,
        }
    }

    /// Sets the content of the module.
    pub fn content(mut self, content: ThinVec<Item>) -> Self {
        self.content = Some(content);
        self
    }

    /// Adds a leading comment to the module item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the module item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the module item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemMod` AST node.
    pub fn build(self) -> ItemMod {
        ItemMod {
            ident: self.ident,
            content: self.content,
            md: self.md,
        }
    }
}

/// Creates a new `ItemStaticBuilder` to construct a `static` item.
pub fn static_item(
    name: impl Into<String>,
    ty: impl Into<Type>,
    expr: impl Into<Expr>,
) -> ItemStaticBuilder {
    ItemStaticBuilder::new(name, ty, expr)
}

/// A builder for constructing an `ItemStatic` AST node.
pub struct ItemStaticBuilder {
    ident: String,
    ty: Type,
    expr: Box<Expr>,
    md: Option<Box<Md>>,
}

impl ItemStaticBuilder {
    /// Creates a new `ItemStaticBuilder`.
    pub fn new(name: impl Into<String>, ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            expr: Box::new(expr.into()),
            md: None,
        }
    }

    /// Adds a leading comment to the `static` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the `static` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the `static` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemStatic` AST node.
    pub fn build(self) -> ItemStatic {
        ItemStatic {
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            md: self.md,
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
    md: Option<Box<Md>>,
}

impl ItemTraitAliasBuilder {
    /// Creates a new `ItemTraitAliasBuilder`.
    pub fn new(name: impl Into<String>, bounds: ThinVec<String>) -> Self {
        Self {
            ident: name.into(),
            bounds,
            md: None,
        }
    }

    /// Adds a leading comment to the trait alias.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the trait alias.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the trait alias.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemTraitAlias` AST node.
    pub fn build(self) -> ItemTraitAlias {
        ItemTraitAlias {
            ident: self.ident,
            bounds: self.bounds,
            md: self.md,
        }
    }
}

/// Creates a new `ItemTypeBuilder` to construct a type alias.
pub fn type_item(name: impl Into<String>, ty: impl Into<Type>) -> ItemTypeBuilder {
    ItemTypeBuilder::new(name, ty)
}

/// A builder for constructing an `ItemType` AST node.
pub struct ItemTypeBuilder {
    ident: String,
    generics: GenericParams,
    ty: Type,
    md: Option<Box<Md>>,
}

impl ItemTypeBuilder {
    /// Creates a new `ItemTypeBuilder`.
    pub fn new(name: impl Into<String>, ty: impl Into<Type>) -> Self {
        Self {
            ident: name.into(),
            generics: GenericParams::new(),
            ty: ty.into(),
            md: None,
        }
    }

    /// Adds a generic parameter to the type alias.
    ///
    /// # Parameters
    ///
    /// - `param`: The generic parameter to add.
    pub fn generic(mut self, param: impl Into<GenericParam>) -> Self {
        self.generics.params.push(param.into());
        self
    }

    /// Adds a leading comment to the type alias.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the type alias.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the type alias.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemType` AST node.
    pub fn build(self) -> ItemType {
        ItemType {
            ident: self.ident,
            generics: self.generics,
            ty: self.ty,
            md: self.md,
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
    fields: Vec<Field>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemUnionBuilder {
    /// Creates a new `ItemUnionBuilder`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            generics: GenericParams::new(),
            fields: vec![],
            md: None,
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
            attrs: vec![],
            ident: name.into(),
            ty: ty.into(),
        });
        self
    }

    /// Adds a leading comment to the `union` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the `union` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the `union` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemUnion` AST node.
    pub fn build(self) -> ItemUnion {
        ItemUnion {
            ident: self.ident,
            generics: self.generics,
            fields: self.fields,
            md: self.md,
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
    md: Option<Box<Md>>,
}

impl ItemUseBuilder {
    /// Creates a new `ItemUseBuilder`.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            md: None,
        }
    }

    /// Adds a leading comment to the `use` item.
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .leading_comments
            .push(comment.into());
        self
    }

    /// Adds a trailing comment to the `use` item.
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .trailing_comments
            .push(comment.into());
        self
    }

    /// Adds an attribute to the `use` item.
    pub fn attr(mut self, attr: impl Into<Attribute>) -> Self {
        self.md
            .get_or_insert_with(Default::default)
            .attrs
            .push(attr.into());
        self
    }

    /// Builds the `ItemUse` AST node.
    pub fn build(self) -> ItemUse {
        ItemUse {
            path: self.path,
            md: self.md,
        }
    }
}

impl Into<Pat> for &str {
    fn into(self) -> Pat {
        Pat::Ident(PatIdent { 
            ident: self.into(),
            is_mut: false,
        })
    }
}
