use crate::ast::{literals::Lit, statements::Block, TokenStream};

/// An expression.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A literal expression, like `1`, `"hello"`.
    Lit(Lit),
    /// A binary operation: `1 + 2`
    Binary(ExprBinary),
    /// An if expression: `if x { y } else { z }`
    If(ExprIf),
    /// A block expression: `{ ... }`
    Block(ExprBlock),
    /// A loop expression: `loop { ... }`
    Loop(ExprLoop),
    /// A while expression: `while x { ... }`
    While(ExprWhile),
    /// A for expression: `for pat in iter { ... }`
    For(ExprFor),
    /// An assignment expression: `x = y`
    Assign(ExprAssign),
    /// A macro call expression: `foo!(...)`
    MacroCall(ExprMacroCall),
}

/// A binary operation: `1 + 2`
#[derive(Debug, Clone)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
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
