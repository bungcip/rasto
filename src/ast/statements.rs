use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;

/// A block of code: `{ ... }`
#[derive(Debug, Clone)]
pub struct Block {
    /// Comments that appear at the beginning of the block, before any statements.
    pub leading_comments: Vec<Comment>,
    /// The statements within the block.
    pub stmts: Vec<Stmt>,
    /// Comments that appear at the end of the block, after all statements.
    pub trailing_comments: Vec<Comment>,
}

/// A statement.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// An expression statement, like `2 + 2;`.
    /// For now, we assume all expression statements are terminated by a semicolon.
    Expr(Expr),
    /// A let statement: `let x = 1;`
    Let(StmtLet),
}

/// A let statement: `let x = 1;`
#[derive(Debug, Clone)]
pub struct StmtLet {
    /// The name of the variable being bound.
    pub ident: String,
    /// The type of the variable.
    pub ty: Option<String>,
    /// The expression the variable is bound to.
    pub expr: Option<Expr>,
}
