//! Defines the AST nodes for statements.
//!
//! Statements are instructions that perform an action but do not produce a value.
//! They are the building blocks of function bodies and other code blocks.

use crate::ast::expressions::{Expr, ExprMacroCall};
use crate::ast::items::Item;
use crate::ast::patterns::Pat;
use crate::ast::types::Type;
use crate::ast::Md;
use thin_vec::ThinVec;

/// A block of code, enclosed in curly braces: `{ ... }`.
///
/// A block contains a sequence of statements and is also an expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The statements within the block.
    pub stmts: ThinVec<Stmt>,
    /// Whether the last statement in the block has a trailing semicolon.
    pub has_trailing_semicolon: bool,
    ///
    pub md: Option<Box<Md>>,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            stmts: Default::default(),
            has_trailing_semicolon: true,
            md: Default::default(),
        }
    }
}

/// A statement in a block.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// A local (let) binding.
    Local(Local),
    /// An item definition.
    Item(Item),
    /// An expression statement.
    Expr(Expr),
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
