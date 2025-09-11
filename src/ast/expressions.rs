use crate::ast::{literals::Lit, statements::Block, TokenStream};

/// An expression.
#[derive(Debug, Clone)]
pub enum Expr {
    // Expressions sorted alphabetically
    /// An array expression: `[a, b, c]`
    Array(ExprArray),
    /// An assignment expression: `x = y`
    Assign(ExprAssign),
    /// An async block: `async { ... }`
    Async(ExprAsync),
    /// An await expression: `future.await`
    Await(ExprAwait),
    /// A binary operation: `1 + 2`
    Binary(ExprBinary),
    /// A block expression: `{ ... }`
    Block(ExprBlock),
    /// A break expression: `break`
    Break(ExprBreak),
    /// A call expression: `foo(a, b)`
    Call(ExprCall),
    /// A cast expression: `x as u32`
    Cast(ExprCast),
    /// A closure expression: `|a, b| a + b`
    Closure(ExprClosure),
    /// A const block: `const { ... }`
    Const(ExprConst),
    /// A continue expression: `continue`
    Continue(ExprContinue),
    /// A field access expression: `stru.field`
    Field(ExprField),
    /// A for expression: `for pat in iter { ... }`
    For(ExprFor),
    /// An if expression: `if x { y } else { z }`
    If(ExprIf),
    /// An index expression: `arr[i]`
    Index(ExprIndex),
    /// A literal expression, like `1`, `"hello"`.
    Lit(Lit),
    /// A loop expression: `loop { ... }`
    Loop(ExprLoop),
    /// A macro call expression: `foo!(...)`
    MacroCall(ExprMacroCall),
    /// A match expression: `match x { ... }`
    Match(ExprMatch),
    /// A method call expression: `obj.method(a, b)`
    MethodCall(ExprMethodCall),
    /// A parenthesized expression: `(a + b)`
    Paren(ExprParen),
    /// A range expression: `a..b`, `a..=b`, `..b`, `a..`
    Range(ExprRange),
    /// A reference expression: `&x` or `&mut x`
    Reference(ExprRef),
    /// A return expression: `return x`
    Return(ExprReturn),
    /// A struct expression: `Foo { a: 1, b: 2 }`
    Struct(ExprStruct),
    /// A tuple expression: `(a, b, c)`
    Tuple(ExprTuple),
    /// A while expression: `while x { ... }`
    While(ExprWhile),
}

/// An array expression: `[a, b, c]`
#[derive(Debug, Clone)]
pub struct ExprArray {
    pub elems: Vec<Expr>,
}

/// An async block: `async { ... }`
#[derive(Debug, Clone)]
pub struct ExprAsync {
    pub block: Block,
}

/// An await expression: `future.await`
#[derive(Debug, Clone)]
pub struct ExprAwait {
    pub expr: Box<Expr>,
}

/// A binary operation: `1 + 2`
#[derive(Debug, Clone)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

/// A break expression: `break`
#[derive(Debug, Clone)]
pub struct ExprBreak;

/// A call expression: `foo(a, b)`
#[derive(Debug, Clone)]
pub struct ExprCall {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

/// A cast expression: `x as u32`
#[derive(Debug, Clone)]
pub struct ExprCast {
    pub expr: Box<Expr>,
    pub ty: String,
}

/// A closure expression: `|a, b| a + b`
#[derive(Debug, Clone)]
pub struct ExprClosure {
    pub inputs: Vec<String>,
    pub body: Box<Expr>,
}

/// A const block: `const { ... }`
#[derive(Debug, Clone)]
pub struct ExprConst {
    pub block: Block,
}

/// A continue expression: `continue`
#[derive(Debug, Clone)]
pub struct ExprContinue;

/// A field access expression: `stru.field`
#[derive(Debug, Clone)]
pub struct ExprField {
    pub expr: Box<Expr>,
    pub member: String,
}

/// An index expression: `arr[i]`
#[derive(Debug, Clone)]
pub struct ExprIndex {
    pub expr: Box<Expr>,
    pub index: Box<Expr>,
}

/// A match expression: `match x { ... }`
#[derive(Debug, Clone)]
pub struct ExprMatch {
    pub expr: Box<Expr>,
    pub arms: Vec<Arm>,
}

/// An arm of a match expression.
#[derive(Debug, Clone)]
pub struct Arm {
    pub pat: String,
    pub guard: Option<Box<Expr>>,
    pub body: Box<Expr>,
}

/// A method call expression: `obj.method(a, b)`
#[derive(Debug, Clone)]
pub struct ExprMethodCall {
    pub receiver: Box<Expr>,
    pub method: String,
    pub args: Vec<Expr>,
}

/// A parenthesized expression: `(a + b)`
#[derive(Debug, Clone)]
pub struct ExprParen {
    pub expr: Box<Expr>,
}

/// A range expression: `a..b`, `a..=b`, `..b`, `a..`
#[derive(Debug, Clone)]
pub struct ExprRange {
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}

/// The limits of a range expression.
#[derive(Debug, Clone)]
pub enum RangeLimits {
    /// `..`
    HalfOpen,
    /// `..=`
    Closed,
}

/// A reference expression: `&x` or `&mut x`
#[derive(Debug, Clone)]
pub struct ExprRef {
    pub is_mut: bool,
    pub expr: Box<Expr>,
}

/// A return expression: `return x`
#[derive(Debug, Clone)]
pub struct ExprReturn {
    pub expr: Option<Box<Expr>>,
}

/// A struct expression: `Foo { a: 1, b: 2 }`
#[derive(Debug, Clone)]
pub struct ExprStruct {
    pub path: String,
    pub fields: Vec<FieldValue>,
}

/// A field-value pair in a struct expression.
#[derive(Debug, Clone)]
pub struct FieldValue {
    pub member: String,
    pub value: Expr,
}

/// A tuple expression: `(a, b, c)`
#[derive(Debug, Clone)]
pub struct ExprTuple {
    pub elems: Vec<Expr>,
}

/// A binary operator.
#[derive(Debug, Clone)]
pub enum BinOp {
    /// The `+` operator.
    Add,
    /// The `-` operator.
    Sub,
    /// The `*` operator.
    Mul,
    /// The `/` operator.
    Div,
}

/// An if expression: `if x { y } else { z }`
#[derive(Debug, Clone)]
pub struct ExprIf {
    pub cond: Box<Expr>,
    pub then_branch: Block,
    pub else_branch: Option<Box<Expr>>,
}

/// A block expression: `{ ... }`
#[derive(Debug, Clone)]
pub struct ExprBlock {
    pub block: Block,
}

/// A loop expression: `loop { ... }`
#[derive(Debug, Clone)]
pub struct ExprLoop {
    pub body: Block,
}

/// A while expression: `while x { ... }`
#[derive(Debug, Clone)]
pub struct ExprWhile {
    pub cond: Box<Expr>,
    pub body: Block,
}

/// A for expression: `for pat in iter { ... }`
#[derive(Debug, Clone)]
pub struct ExprFor {
    pub pat: String,
    pub expr: Box<Expr>,
    pub body: Block,
}

/// An assignment expression: `x = y`
#[derive(Debug, Clone)]
pub struct ExprAssign {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

/// A macro call expression: `foo!(...)`
#[derive(Debug, Clone)]
pub struct ExprMacroCall {
    pub ident: String,
    pub tokens: TokenStream,
}
