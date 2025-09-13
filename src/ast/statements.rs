//! Defines the AST nodes for statements.
//!
//! Statements are instructions that perform an action but do not produce a value.
//! They are the building blocks of function bodies and other code blocks.

use crate::ast::comments::Comment;
use crate::ast::expressions::{Expr, ExprMacroCall};
use crate::ast::items::Item;
use crate::ast::patterns::Pat;
use crate::ast::types::Type;
use thin_vec::ThinVec;

/// A block of code, enclosed in curly braces: `{ ... }`.
///
/// A block contains a sequence of statements and is also an expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Comments that appear at the beginning of the block, before any statements.
    pub leading_comments: ThinVec<Comment>,
    /// The statements within the block.
    pub stmts: ThinVec<Stmt>,
    /// Comments that appear at the end of the block, after all statements.
    pub trailing_comments: ThinVec<Comment>,
}

/// A statement in a block.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// A local (let) binding.
    Local(Local),
    /// An item definition.
    Item(Item),
    /// An expression statement, with an optional trailing semicolon.
    Expr(Expr, bool),
    /// A macro call.
    MacCall(ExprMacroCall),
}

/// A `let` statement: `let x = 1;`.
#[derive(Debug, Clone, PartialEq)]
pub struct Local {
    /// The pattern to bind.
    pub pat: Pat,
    /// The optional type annotation of the variable.
    pub ty: Option<Type>,
    /// The optional expression to initialize the variable.
    pub expr: Option<Expr>,
}
