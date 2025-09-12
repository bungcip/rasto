//! Defines the AST nodes for statements.
//!
//! Statements are instructions that perform an action but do not produce a value.
//! They are the building blocks of function bodies and other code blocks.

use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;

/// A block of code, enclosed in curly braces: `{ ... }`.
///
/// A block contains a sequence of statements and is also an expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Comments that appear at the beginning of the block, before any statements.
    pub leading_comments: Vec<Comment>,
    /// The statements within the block.
    pub stmts: Vec<Stmt>,
    /// Comments that appear at the end of the block, after all statements.
    pub trailing_comments: Vec<Comment>,
}

/// A statement in a block.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// An expression statement, such as `2 + 2;`.
    /// For now, we assume all expression statements are terminated by a semicolon.
    Expr(Expr),
    /// A `let` statement, which binds a variable: `let x = 1;`.
    Let(StmtLet),
}

/// A `let` statement: `let x = 1;`.
#[derive(Debug, Clone, PartialEq)]
pub struct StmtLet {
    /// The name of the variable being bound.
    pub ident: String,
    /// The optional type annotation of the variable.
    pub ty: Option<String>,
    /// The optional expression to initialize the variable.
    pub expr: Option<Expr>,
}
