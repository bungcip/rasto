//! Provides a builder API for constructing AST nodes programmatically.
//!
//! The builder pattern allows for a more fluent and readable way of creating complex AST structures.
//! Each builder corresponds to a specific AST node and provides methods for setting its properties.
//!
//! # Examples
//!
//! ```
//! use rasto::ast::builder::*;
//! use rasto::ast::*;
//!
//! let file_ast = file()
//!     .item(
//!         fn_def("my_function")
//!             .block(Block {
//!                 leading_comments: vec![],
//!                 stmts: vec![Stmt::Expr(expr().lit(Lit::Int(42)))],
//!                 trailing_comments: vec![],
//!             })
//!             .build(),
//!     )
//!     .build();
//! ```

use crate::ast::*;

/// Creates a new `FileBuilder` to construct a `File` AST node.
///
/// # Returns
///
/// A `FileBuilder` instance.
pub fn file() -> FileBuilder {
    FileBuilder::new()
}

/// A builder for constructing a `File` AST node.
pub struct FileBuilder {
    items: Vec<Item>,
}

impl FileBuilder {
    /// Creates a new, empty `FileBuilder`.
    pub fn new() -> Self {
        Self { items: vec![] }
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
    inputs: Vec<Type>,
    output: Option<Type>,
    block: Option<Block>,
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
        }
    }

    /// Adds an input parameter to the function.
    ///
    /// # Parameters
    ///
    /// - `ty`: The type of the input parameter.
    pub fn input(mut self, ty: impl Into<Type>) -> Self {
        self.inputs.push(ty.into());
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
            leading_comments: vec![],
            sig: Signature {
                ident: self.ident,
                inputs: self.inputs,
                output: self.output,
            },
            block,
            trailing_comments: vec![],
        }
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
    /// - `inputs`: An iterator of strings for the closure's input parameters.
    /// - `body`: The body of the closure.
    pub fn closure(
        self,
        inputs: impl IntoIterator<Item = impl Into<String>>,
        body: Expr,
    ) -> Expr {
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
    pub fn for_loop(self, pat: impl Into<String>, expr: Expr, body: Block) -> Expr {
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

#[allow(missing_docs)]
pub fn const_item(name: impl Into<String>, ty: impl Into<Type>, expr: impl Into<Expr>) -> ItemConstBuilder {
    ItemConstBuilder::new(name, ty, expr)
}

#[allow(missing_docs)]
pub struct ItemConstBuilder {
    ident: String,
    ty: Type,
    expr: Box<Expr>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemConstBuilder {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>, ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            expr: Box::new(expr.into()),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemConst {
        ItemConst {
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn extern_crate_item(name: impl Into<String>) -> ItemExternCrateBuilder {
    ItemExternCrateBuilder::new(name)
}

#[allow(missing_docs)]
pub struct ItemExternCrateBuilder {
    ident: String,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemExternCrateBuilder {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemExternCrate {
        ItemExternCrate {
            ident: self.ident,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn foreign_mod_item(abi: impl Into<String>) -> ItemForeignModBuilder {
    ItemForeignModBuilder::new(abi)
}

#[allow(missing_docs)]
pub struct ItemForeignModBuilder {
    abi: String,
    items: Vec<Item>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemForeignModBuilder {
    #[allow(missing_docs)]
    pub fn new(abi: impl Into<String>) -> Self {
        Self {
            abi: abi.into(),
            items: vec![],
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemForeignMod {
        ItemForeignMod {
            abi: self.abi,
            items: self.items,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn macro_item(expr: impl Into<Expr>) -> ItemMacroBuilder {
    ItemMacroBuilder::new(expr)
}

#[allow(missing_docs)]
pub struct ItemMacroBuilder {
    expr: Expr,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemMacroBuilder {
    #[allow(missing_docs)]
    pub fn new(expr: impl Into<Expr>) -> Self {
        Self {
            expr: expr.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemMacro {
        ItemMacro {
            expr: Box::new(self.expr),
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn mod_item(name: impl Into<String>) -> ItemModBuilder {
    ItemModBuilder::new(name)
}

#[allow(missing_docs)]
pub struct ItemModBuilder {
    ident: String,
    content: Option<Vec<Item>>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemModBuilder {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            content: None,
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn content(mut self, content: Vec<Item>) -> Self {
        self.content = Some(content);
        self
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemMod {
        ItemMod {
            ident: self.ident,
            content: self.content,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn static_item(name: impl Into<String>, ty: impl Into<Type>, expr: impl Into<Expr>) -> ItemStaticBuilder {
    ItemStaticBuilder::new(name, ty, expr)
}

#[allow(missing_docs)]
pub struct ItemStaticBuilder {
    ident: String,
    ty: Type,
    expr: Box<Expr>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemStaticBuilder {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>, ty: impl Into<Type>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            expr: Box::new(expr.into()),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemStatic {
        ItemStatic {
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn trait_alias_item(name: impl Into<String>, bounds: Vec<String>) -> ItemTraitAliasBuilder {
    ItemTraitAliasBuilder::new(name, bounds)
}

#[allow(missing_docs)]
pub struct ItemTraitAliasBuilder {
    ident: String,
    bounds: Vec<String>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemTraitAliasBuilder {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>, bounds: Vec<String>) -> Self {
        Self {
            ident: name.into(),
            bounds,
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemTraitAlias {
        ItemTraitAlias {
            ident: self.ident,
            bounds: self.bounds,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn type_item(name: impl Into<String>, ty: impl Into<Type>) -> ItemTypeBuilder {
    ItemTypeBuilder::new(name, ty)
}

#[allow(missing_docs)]
pub struct ItemTypeBuilder {
    ident: String,
    ty: Type,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemTypeBuilder {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>, ty: impl Into<Type>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemType {
        ItemType {
            ident: self.ident,
            ty: self.ty,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn union_item(name: impl Into<String>) -> ItemUnionBuilder {
    ItemUnionBuilder::new(name)
}

#[allow(missing_docs)]
pub struct ItemUnionBuilder {
    ident: String,
    fields: Vec<Field>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemUnionBuilder {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            fields: vec![],
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemUnion {
        ItemUnion {
            ident: self.ident,
            fields: self.fields,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

#[allow(missing_docs)]
pub fn use_item(path: impl Into<String>) -> ItemUseBuilder {
    ItemUseBuilder::new(path)
}

#[allow(missing_docs)]
pub struct ItemUseBuilder {
    path: String,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemUseBuilder {
    #[allow(missing_docs)]
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    #[allow(missing_docs)]
    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    #[allow(missing_docs)]
    pub fn build(self) -> ItemUse {
        ItemUse {
            path: self.path,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}
