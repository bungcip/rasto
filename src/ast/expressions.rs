use crate::ast::{literals::Lit, statements::Block};

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
