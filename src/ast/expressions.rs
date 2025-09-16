//! Defines the AST nodes for expressions.
//!
//! Expressions are a core part of the Rust language and represent computations that produce a value.
//! This module provides the data structures for all kinds of expressions, such as binary operations,
//! function calls, and control flow expressions like `if` and `match`.

use crate::ast::{Pat, TokenStream, generics::GenericArgs, literals::Lit, statements::Block};
use thin_vec::ThinVec;

/// Represents a Rust expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Expressions sorted alphabetically
    /// An array expression: `[a, b, c]`.
    Array(ExprArray),
    /// An assignment expression: `x = y`.
    Assign(ExprAssign),
    /// An `async` block: `async { ... }`.
    Async(ExprAsync),
    /// An `await` expression: `future.await`.
    Await(ExprAwait),
    /// A binary operation: `a + b`.
    Binary(ExprBinary),
    /// A block expression: `{ ... }`.
    Block(ExprBlock),
    /// A `break` expression.
    Break(ExprBreak),
    /// A function call expression: `foo(a, b)`.
    Call(ExprCall),
    /// A type cast expression: `x as u32`.
    Cast(ExprCast),
    /// A closure expression: `|a, b| a + b`.
    Closure(ExprClosure),
    /// A `const` block: `const { ... }`.
    Const(ExprConst),
    /// A `continue` expression.
    Continue(ExprContinue),
    /// A field access expression: `stru.field`.
    Field(ExprField),
    /// A `for` loop expression: `for pat in iter { ... }`.
    For(ExprFor),
    /// An `if` expression: `if x { y } else { z }`.
    If(ExprIf),
    /// An index expression: `arr[i]`.
    Index(ExprIndex),
    /// A literal expression, like `1` or `"hello"`.
    Lit(Lit),
    /// A `loop` expression: `loop { ... }`.
    Loop(ExprLoop),
    /// A macro call expression: `foo!(...)`.
    MacroCall(ExprMacroCall),
    /// A `match` expression: `match x { ... }`.
    Match(ExprMatch),
    /// A method call expression: `obj.method(a, b)`.
    MethodCall(ExprMethodCall),
    /// A parenthesized expression: `(a + b)`.
    Paren(ExprParen),
    /// A range expression: `a..b`, `a..=b`, `..b`, `a..`.
    Range(ExprRange),
    /// A reference expression: `&x` or `&mut x`.
    Reference(ExprRef),
    /// A raw reference expression: `&raw const x` or `&raw mut x`.
    RawRef(ExprRawRef),
    /// A `return` expression: `return x`.
    Return(ExprReturn),
    /// A struct instantiation expression: `Foo { a: 1, b: 2 }`.
    Struct(ExprStruct),
    /// A `try` block: `try { ... }`.
    Try(ExprTry),
    /// A tuple expression: `(a, b, c)`.
    Tuple(ExprTuple),
    /// A unary operation: `!x` or `-x`.
    Unary(ExprUnary),
    /// A `while` loop expression: `while x { ... }`.
    While(ExprWhile),
}

/// A unary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnOp {
    /// The `!` operator (logical negation).
    Not,
    /// The `-` operator (negation).
    Neg,
}

/// A unary expression, such as `!x` or `-x`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprUnary {
    /// The unary operator, e.g., `!` or `-`.
    pub op: UnOp,
    /// The expression the operator is applied to.
    pub expr: Box<Expr>,
}

/// An array expression, such as `[a, b, c]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprArray {
    /// The elements of the array.
    pub elems: ThinVec<Expr>,
}

/// An `async` block, such as `async { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprAsync {
    /// The block of statements inside the `async` block.
    pub block: Block,
}

/// An `await` expression, such as `future.await`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprAwait {
    /// The future to `await`.
    pub expr: Box<Expr>,
}

/// A binary operation, such as `a + b`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    /// The left-hand side of the operation.
    pub left: Box<Expr>,
    /// The binary operator.
    pub op: BinOp,
    /// The right-hand side of the operation.
    pub right: Box<Expr>,
}

/// A `break` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBreak;

/// A function call expression, such as `foo(a, b)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    /// The function to call. This can be a path to a function or a closure.
    pub func: Box<Expr>,
    /// The arguments to the function.
    pub args: ThinVec<Expr>,
}

use crate::ast::types::Type;
/// A type cast expression, such as `x as u32`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprCast {
    /// The expression to cast.
    pub expr: Box<Expr>,
    /// The type to cast to.
    pub ty: Type,
}

/// A closure expression, such as `|a, b| a + b`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprClosure {
    /// The input parameters of the closure.
    pub inputs: ThinVec<Pat>,
    /// The body of the closure.
    pub body: Box<Expr>,
}

/// A `const` block, such as `const { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprConst {
    /// The block of statements inside the `const` block.
    pub block: Block,
}

/// A `continue` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprContinue;

/// A field access expression, such as `stru.field`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprField {
    /// The expression to access the field from.
    pub expr: Box<Expr>,
    /// The name of the field.
    pub member: String,
}

/// An index expression, such as `arr[i]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprIndex {
    /// The expression to index into.
    pub expr: Box<Expr>,
    /// The index expression.
    pub index: Box<Expr>,
}

/// A `match` expression, such as `match x { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprMatch {
    /// The expression to match on.
    pub expr: Box<Expr>,
    /// The arms of the `match` expression.
    pub arms: ThinVec<Arm>,
}

/// An arm of a `match` expression, e.g., `pat if guard => body`.
#[derive(Debug, Clone, PartialEq)]
pub struct Arm {
    /// The pattern to match against.
    pub pat: Pat,
    /// An optional guard expression.
    pub guard: Option<Box<Expr>>,
    /// The body of the arm.
    pub body: Box<Expr>,
}

/// A method call expression, such as `obj.method(a, b)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprMethodCall {
    /// The receiver of the method call.
    pub receiver: Box<Expr>,
    /// The name of the method.
    pub method: String,
    /// The arguments to the method.
    pub args: ThinVec<Expr>,
}

/// A parenthesized expression, such as `(a + b)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprParen {
    /// The expression inside the parentheses.
    pub expr: Box<Expr>,
}

/// A range expression, such as `a..b`, `a..=b`, `..b`, `a..`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprRange {
    /// The optional start of the range.
    pub start: Option<Box<Expr>>,
    /// The type of range (`..` or `..=`).
    pub limits: RangeLimits,
    /// The optional end of the range.
    pub end: Option<Box<Expr>>,
}

/// The limits of a range expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeLimits {
    /// A half-open range (`..`).
    HalfOpen,
    /// A closed range (`..=`).
    Closed,
}

/// A reference expression, such as `&x` or `&mut x`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprRef {
    /// `true` if the reference is mutable (`&mut`).
    pub is_mut: bool,
    /// The expression being referenced.
    pub expr: Box<Expr>,
}

/// A raw reference expression, such as `&raw const x` or `&raw mut x`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprRawRef {
    /// `true` if the reference is mutable (`&raw mut`).
    pub is_mut: bool,
    /// The expression being referenced.
    pub expr: Box<Expr>,
}

/// A `return` expression, such as `return x`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprReturn {
    /// The optional expression to return.
    pub expr: Option<Box<Expr>>,
}

/// A struct instantiation expression, such as `Foo { a: 1, b: 2 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprStruct {
    /// The path to the struct type.
    pub path: String,
    /// The fields to initialize.
    pub fields: ThinVec<FieldValue>,
}

/// A `try` block, such as `try { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprTry {
    /// The block of statements inside the `try` block.
    pub block: Block,
}

/// A field-value pair in a struct expression.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldValue {
    /// The name of the field.
    pub member: String,
    /// The value of the field.
    pub value: Expr,
}

/// A tuple expression, such as `(a, b, c)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprTuple {
    /// The elements of the tuple.
    pub elems: ThinVec<Expr>,
}

/// A binary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    /// The `+` operator (addition).
    Add,
    /// The `-` operator (subtraction).
    Sub,
    /// The `*` operator (multiplication).
    Mul,
    /// The `/` operator (division).
    Div,
}

/// An `if` expression, such as `if x { y } else { z }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprIf {
    /// The condition of the `if` expression.
    pub cond: Box<Expr>,
    /// The block to execute if the condition is true.
    pub then_branch: Block,
    /// The optional `else` branch.
    pub else_branch: Option<Box<Expr>>,
}

/// A block expression, such as `{ ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBlock {
    /// The block of statements.
    pub block: Block,
}

/// A `loop` expression, such as `loop { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprLoop {
    /// The body of the loop.
    pub body: Block,
}

/// A `while` loop expression, such as `while x { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprWhile {
    /// The condition of the `while` loop.
    pub cond: Box<Expr>,
    /// The body of the loop.
    pub body: Block,
}

/// A `for` loop expression, such as `for pat in iter { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprFor {
    /// The pattern to bind the elements of the iterator.
    pub pat: Pat,
    /// The expression to iterate over.
    pub expr: Box<Expr>,
    /// The body of the loop.
    pub body: Block,
}

/// An assignment expression, such as `x = y`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprAssign {
    /// The left-hand side of the assignment.
    pub left: Box<Expr>,
    /// The right-hand side of the assignment.
    pub right: Box<Expr>,
}

impl From<ExprTry> for Expr {
    /// Converts an `ExprTry` into an `Expr::Try`.
    fn from(expr: ExprTry) -> Self {
        Expr::Try(expr)
    }
}

use crate::ast::tokens::Delimiter;
/// A macro call expression, such as `foo!(...)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprMacroCall {
    /// The path to the macro being called.
    pub path: Path,
    /// The delimiter of the macro’s input.
    pub delimiter: Delimiter,
    /// The token stream passed to the macro.
    pub tokens: TokenStream,
}

/// A path expression, such as `foo::bar::baz`.
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    /// The segments of the path.
    pub segments: ThinVec<PathSegment>,
}

/// A segment of a path, such as `foo` or `bar`.
#[derive(Debug, Clone, PartialEq)]
pub struct PathSegment {
    /// The identifier of the segment.
    pub ident: String,
    /// The generic arguments of the segment, such as `<T>`.
    pub args: Option<GenericArgs>,
}
