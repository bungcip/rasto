//! Defines the AST nodes for patterns.
//!
//! Patterns are used in `let` bindings, function parameters, and `match` expressions to destructure
//! data. They are a powerful feature of Rust that allows for expressive and concise code.

use crate::ast::{ident::Ident, *};
use thin_vec::ThinVec;

/// Represents a pattern in a `let` binding, function parameter, or `match` arm.
#[derive(Debug, Clone, PartialEq)]
pub enum Pat {
    /// A const pattern, e.g., `const FOO`.
    Const(PatConst),
    /// An identifier pattern, e.g., `x` or `mut x`.
    Ident(PatIdent),
    /// A literal pattern, e.g., `1`, `"a"`, `true`.
    Lit(PatLit),
    /// A macro pattern, e.g., `mac!(...)`.
    Macro(PatMacro),
    /// An "or" pattern, e.g., `p | q`.
    Or(PatOr),
    /// A parenthesized pattern, e.g., `(p)`.
    Paren(PatParen),
    /// A path pattern, e.g., `Some(x)`, `Color::Red`.
    Path(PatPath),
    /// A range pattern, e.g., `1..=5`, `'a'..='z'`.
    Range(PatRange),
    /// A reference pattern, e.g., `&x`, `&mut y`.
    Reference(PatReference),
    /// A rest pattern, e.g., `..`.
    Rest(PatRest),
    /// A slice pattern, e.g., `[a, b, c]`.
    Slice(PatSlice),
    /// A struct pattern, e.g., `Point { x, y }`.
    Struct(PatStruct),
    /// A tuple pattern, e.g., `(a, b)`.
    Tuple(PatTuple),
    /// A tuple struct pattern, e.g., `Point(x, y)`.
    TupleStruct(PatTupleStruct),
    /// A type pattern, e.g., `x: T`.
    Type(PatType),
    /// A wildcard pattern, e.g., `_`.
    Wild(PatWild),
}

/// A const pattern: `const FOO`
#[derive(Debug, Clone, PartialEq)]
pub struct PatConst {
    /// The constant expression.
    pub expr: Box<Expr>,
}

/// An identifier pattern: `x`, `mut x`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatIdent {
    /// The identifier.
    pub ident: Ident,
    /// Whether the pattern is mutable.
    pub is_mut: bool,
}

/// A literal pattern: `1`, `"a"`, `true`
#[derive(Debug, Clone, PartialEq)]
pub struct PatLit {
    /// The literal.
    pub lit: Box<Lit>,
}

/// A macro pattern: `mac!(...)`
#[derive(Debug, Clone, PartialEq)]
pub struct PatMacro {
    /// The macro call.
    pub mac: ExprMacroCall,
}

/// An "or" pattern: `p | q`
#[derive(Debug, Clone, PartialEq)]
pub struct PatOr {
    /// The sub-patterns.
    pub pats: ThinVec<Pat>,
}

/// A parenthesized pattern: `(p)`
#[derive(Debug, Clone, PartialEq)]
pub struct PatParen {
    /// The sub-pattern.
    pub pat: Box<Pat>,
}

/// A path pattern: `Some(x)`, `Color::Red`
#[derive(Debug, Clone, PartialEq)]
pub struct PatPath {
    /// The path.
    pub path: Path,
}

/// A range pattern: `1..=5`, `'a'..='z'`
#[derive(Debug, Clone, PartialEq)]
pub struct PatRange {
    /// The start of the range.
    pub start: Option<Box<Expr>>,
    /// The end of the range.
    pub end: Option<Box<Expr>>,
    /// The limits of the range.
    pub limits: RangeLimits,
}

/// A reference pattern: `&x`, `&mut y`
#[derive(Debug, Clone, PartialEq)]
pub struct PatReference {
    /// The sub-pattern.
    pub pat: Box<Pat>,
    /// Whether the reference is mutable.
    pub is_mut: bool,
}

/// A rest pattern: `..`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PatRest;

/// A slice pattern: `[a, b, c]`
#[derive(Debug, Clone, PartialEq)]
pub struct PatSlice {
    /// The sub-patterns.
    pub pats: ThinVec<Pat>,
}

/// A struct pattern: `Point { x, y }`
#[derive(Debug, Clone, PartialEq)]
pub struct PatStruct {
    /// The path to the struct.
    pub path: Path,
    /// The fields of the struct.
    pub fields: ThinVec<FieldPat>,
    /// Whether the struct has a rest pattern.
    pub has_rest: bool,
}

/// A field in a struct pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldPat {
    /// The name of the field.
    pub member: Ident,
    /// The pattern for the field.
    pub pat: Box<Pat>,
}

/// A tuple pattern: `(a, b)`
#[derive(Debug, Clone, PartialEq)]
pub struct PatTuple {
    /// The sub-patterns.
    pub pats: ThinVec<Pat>,
}

/// A tuple struct pattern: `Point(x, y)`
#[derive(Debug, Clone, PartialEq)]
pub struct PatTupleStruct {
    /// The path to the tuple struct.
    pub path: Path,
    /// The sub-patterns.
    pub pats: ThinVec<Pat>,
}

/// A type pattern: `x: T`
#[derive(Debug, Clone, PartialEq)]
pub struct PatType {
    /// The sub-pattern.
    pub pat: Box<Pat>,
    /// The type annotation.
    pub ty: Box<Type>,
}

/// A wildcard pattern: `_`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PatWild;
