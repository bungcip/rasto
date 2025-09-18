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
    /// A path expression.
    Path(ExprPath),
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

/// Represents a unary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnOp {
    /// The logical negation operator `!`.
    Not,
    /// The arithmetic negation operator `-`.
    Neg,
}

/// Represents a unary operation, which consists of an operator applied to a
/// single expression.
///
/// For example, `-x` or `!y`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprUnary {
    /// The unary operator to be applied, such as `!` or `-`.
    pub op: UnOp,
    /// The expression that the operator is applied to.
    pub expr: Box<Expr>,
}

/// Represents an array expression, which creates an array with a fixed size.
///
/// For example, `[1, 2, 3]` or `[0; 10]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprArray {
    /// The list of expressions that initialize the elements of the array.
    pub elems: ThinVec<Expr>,
}

/// Represents an `async` block, which creates a `Future` that can be awaited.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprAsync {
    /// The block of statements that will be executed asynchronously.
    pub block: Block,
}

/// Represents an `await` expression, which is used to pause the execution of an
/// `async` function until a `Future` is resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprAwait {
    /// The expression that evaluates to a `Future` to be awaited.
    pub expr: Box<Expr>,
}

/// Represents a binary operation, which combines two expressions with an operator.
///
/// For example, `a + b` or `x * y`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    /// The expression on the left-hand side of the operator.
    pub left: Box<Expr>,
    /// The binary operator, such as `+`, `-`, `*`, or `/`.
    pub op: BinOp,
    /// The expression on the right-hand side of the operator.
    pub right: Box<Expr>,
}

/// Represents a `break` expression, which is used to exit a loop prematurely.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBreak;

/// Represents a function call expression.
///
/// This includes calls to named functions, as well as calls to closures or
/// other values that implement the `Fn` traits.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    /// The expression that evaluates to the function being called.
    pub func: Box<Expr>,
    /// The list of arguments passed to the function.
    pub args: ThinVec<Expr>,
}

use crate::ast::types::Type;
/// Represents a type cast expression, which converts a value from one type
/// to another.
///
/// For example, `x as i64`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprCast {
    /// The expression whose value is being cast.
    pub expr: Box<Expr>,
    /// The type that the expression is being cast to.
    pub ty: Type,
}

/// Represents a closure expression, which is an anonymous function that can
/// capture its environment.
///
/// For example, `|x| x * 2`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprClosure {
    /// The list of input parameters for the closure.
    pub inputs: ThinVec<Pat>,
    /// The body of the closure, which is the code that gets executed.
    pub body: Box<Expr>,
}

/// Represents a `const` block, which is a block of code that is evaluated at
/// compile time.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprConst {
    /// The block of statements that is executed at compile time.
    pub block: Block,
}

/// Represents a `continue` expression, which skips the rest of the current
/// loop iteration and proceeds to the next one.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprContinue;

/// Represents a field access expression, which is used to access a field of a
/// struct or a tuple.
///
/// For example, `my_struct.field` or `my_tuple.0`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprField {
    /// The expression that evaluates to the struct or tuple.
    pub expr: Box<Expr>,
    /// The name of the field being accessed.
    pub member: String,
}

/// Represents an index expression, which is used to access an element of an
/// array, slice, or other collection.
///
/// For example, `my_array[i]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprIndex {
    /// The expression that evaluates to the collection being indexed.
    pub expr: Box<Expr>,
    /// The expression that evaluates to the index.
    pub index: Box<Expr>,
}

/// Represents a `match` expression, which allows for branching based on
/// pattern matching.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprMatch {
    /// The expression whose value is being matched against the patterns in the arms.
    pub expr: Box<Expr>,
    /// The list of arms, each containing a pattern and a corresponding expression.
    pub arms: ThinVec<Arm>,
}

/// Represents an arm of a `match` expression.
///
/// An arm has the form `pattern if guard => body`, where the `if guard` part
/// is optional.
#[derive(Debug, Clone, PartialEq)]
pub struct Arm {
    /// The pattern that the `match` expression's input is tested against.
    pub pat: Pat,
    /// An optional guard expression that provides additional conditions for
    /// the arm to be selected.
    pub guard: Option<Box<Expr>>,
    /// The expression that is executed if the pattern matches and the guard
    /// (if present) evaluates to true.
    pub body: Box<Expr>,
}

/// Represents a method call expression, like `object.method(arg1, arg2)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprMethodCall {
    /// The expression that the method is being called on (the "receiver").
    pub receiver: Box<Expr>,
    /// The name of the method being called.
    pub method: String,
    /// The list of arguments passed to the method.
    pub args: ThinVec<Expr>,
}

/// Represents a parenthesized expression, which is an expression enclosed in `()`.
///
/// Parentheses are used to control the order of operations.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprParen {
    /// The expression that is enclosed within the parentheses.
    pub expr: Box<Expr>,
}

/// A path expression, e.g. `foo` or `foo::bar`
#[derive(Debug, Clone, PartialEq)]
pub struct ExprPath {
    /// The path to the item
    pub path: Path,
}

impl From<ExprPath> for Expr {
    fn from(expr: ExprPath) -> Self {
        Expr::Path(expr)
    }
}

/// Represents a range expression, which can be used for slicing or iteration.
///
/// Ranges can be bounded on both ends (`1..10`), have only a start (`1..`),
/// only an end (`..10`), or be unbounded (`..`).
#[derive(Debug, Clone, PartialEq)]
pub struct ExprRange {
    /// The optional expression that defines the start of the range.
    /// If `None`, the range is open on the left.
    pub start: Option<Box<Expr>>,
    /// The type of range, either half-open (`..`) or closed (`..=`).
    pub limits: RangeLimits,
    /// The optional expression that defines the end of the range.
    /// If `None`, the range is open on the right.
    pub end: Option<Box<Expr>>,
}

/// Defines the bounds of a range expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeLimits {
    /// A half-open range, specified with `..`. The end is exclusive.
    HalfOpen,
    /// A closed range, specified with `..=`. The end is inclusive.
    Closed,
}

/// Represents a reference expression, which creates a pointer to a value.
///
/// References can be either shared (`&x`) or mutable (`&mut x`).
#[derive(Debug, Clone, PartialEq)]
pub struct ExprRef {
    /// `true` if the reference is mutable (`&mut`), `false` for a shared reference (`&`).
    pub is_mut: bool,
    /// The expression that is being referenced.
    pub expr: Box<Expr>,
}

/// Represents a raw reference expression, like `&raw const x` or `&raw mut x`.
///
/// Raw references are unsafe and are primarily used in FFI contexts.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprRawRef {
    /// `true` if the raw reference is mutable (`&raw mut`), `false` otherwise (`&raw const`).
    pub is_mut: bool,
    /// The expression that is being referenced.
    pub expr: Box<Expr>,
}

/// Represents a `return` expression, which exits a function and optionally
/// returns a value.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprReturn {
    /// The optional expression whose value is returned from the function.
    /// If `None`, the function returns the unit type `()`.
    pub expr: Option<Box<Expr>>,
}

/// Represents a struct instantiation expression.
///
/// This is used to create a new instance of a struct, for example:
/// `MyStruct { field1: 42, field2: "hello" }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprStruct {
    /// The path to the struct being instantiated, e.g., `my_module::MyStruct`.
    pub path: String,
    /// The list of fields and their initial values.
    pub fields: ThinVec<FieldValue>,
}

/// Represents a `try` block, which is used for error handling.
///
/// A `try` block executes its statements and returns a `Result`. If any
/// operation within the block returns an `Err`, the block immediately
/// returns that `Err`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprTry {
    /// The block of statements to be executed within the `try` context.
    pub block: Block,
}

/// Represents a field-value pair in a struct instantiation expression.
///
/// For example, in `Foo { bar: 42 }`, `bar: 42` is a `FieldValue`.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldValue {
    /// The name of the field being initialized.
    pub member: String,
    /// The expression that provides the value for the field.
    pub value: Expr,
}

/// Represents a tuple expression, such as `(a, b, c)`.
///
/// A tuple is a fixed-size, ordered list of elements of potentially different types.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprTuple {
    /// The expressions that make up the elements of the tuple.
    pub elems: ThinVec<Expr>,
}

/// Represents a binary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    /// The addition operator `+`.
    Add,
    /// The subtraction operator `-`.
    Sub,
    /// The multiplication operator `*`.
    Mul,
    /// The division operator `/`.
    Div,
}

/// Represents an `if` expression, which allows for conditional execution.
///
/// An `if` expression can optionally have an `else` branch.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprIf {
    /// The condition that is evaluated.
    pub cond: Box<Expr>,
    /// The block of code that is executed if the condition is true.
    pub then_branch: Block,
    /// The optional `else` branch, which is executed if the condition is false.
    ///
    /// This can be another `if` expression for `else if` chains.
    pub else_branch: Option<Box<Expr>>,
}

/// Represents a block expression, which is a collection of statements enclosed
/// in curly braces `{}`.
///
/// The last expression in the block, if it is not followed by a semicolon,
/// determines the value of the block.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBlock {
    /// The block containing the statements.
    pub block: Block,
}

/// Represents a `loop` expression, which creates an infinite loop.
///
/// A `loop` can be exited using `break`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprLoop {
    /// The block of code that is executed repeatedly.
    pub body: Block,
}

/// Represents a `while` loop expression, which continues to execute as long as
/// a condition is true.
///
/// A `while` loop has the structure `while condition { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprWhile {
    /// The condition expression that is evaluated before each iteration of the loop.
    pub cond: Box<Expr>,
    /// The block of code that is executed as long as the condition is true.
    pub body: Block,
}

/// Represents a `for` loop expression, which iterates over an iterator.
///
/// A `for` loop has the structure `for pattern in iterator { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprFor {
    /// The pattern that binds to the elements of the iterator on each iteration.
    pub pat: Pat,
    /// The expression that evaluates to an iterator.
    pub expr: Box<Expr>,
    /// The block of code that is executed for each iteration of the loop.
    pub body: Block,
}

/// Represents an assignment expression, such as `x = y`.
///
/// This is used to assign a value to a variable or a memory location.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprAssign {
    /// The expression on the left-hand side of the assignment, which is
    /// being assigned to.
    pub left: Box<Expr>,
    /// The expression on the right-hand side of the assignment, which is
    /// the value being assigned.
    pub right: Box<Expr>,
}

impl From<ExprTry> for Expr {
    fn from(expr: ExprTry) -> Self {
        Expr::Try(expr)
    }
}

use crate::ast::tokens::Delimiter;
/// Represents a macro invocation expression.
///
/// For example, `println!("Hello, {}!", name)` or `vec![1, 2, 3]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprMacroCall {
    /// The path to the macro being invoked.
    pub path: Path,
    /// The type of delimiter used for the macro's input.
    ///
    /// This can be parentheses `()`, brackets `[]`, or braces `{}`.
    pub delimiter: Delimiter,
    /// The token stream that is passed as input to the macro.
    pub tokens: TokenStream,
}

/// Represents a path, which is a sequence of identifiers separated by `::`.
///
/// Paths are used to refer to items, such as functions, structs, and modules.
/// For example, `std::collections::HashMap`.
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    /// The list of segments that make up the path.
    pub segments: ThinVec<PathSegment>,
}

/// Represents a single segment of a path.
///
/// A path segment is an identifier, optionally followed by generic arguments.
/// For example, in `std::collections::HashMap<K, V>`, `std`, `collections`, and
/// `HashMap<K, V>` are all path segments.
#[derive(Debug, Clone, PartialEq)]
pub struct PathSegment {
    /// The identifier of the path segment.
    pub ident: String,
    /// The optional generic arguments associated with this path segment.
    ///
    /// For example, in `Vec<i32>`, the arguments would be `<i32>`.
    pub args: Option<GenericArgs>,
}
