//! The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes
//! that represent Rust `where` clauses.

use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `where` clause, such as `where T: Trait`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WhereClause {
    /// The predicates in the `where` clause.
    pub predicates: Vec<WherePredicate>,
}

impl WhereClause {
    /// Creates a new, empty `where` clause.
    pub fn new() -> Self {
        Self { predicates: vec![] }
    }
}

/// A single predicate in a `where` clause.
#[derive(Debug, Clone, PartialEq)]
pub enum WherePredicate {
    /// A lifetime predicate, such as `'a: 'b`.
    Lifetime(LifetimePredicate),
    /// A type-bound predicate, such as `T: Trait`.
    Type(TypePredicate),
}

/// A lifetime predicate, such as `'a: 'b`.
#[derive(Debug, Clone, PartialEq)]
pub struct LifetimePredicate {
    /// The lifetime being bounded.
    pub lifetime: String,
    /// The bounds on the lifetime.
    pub bounds: Vec<String>,
}

/// A type-bound predicate, such as `T: Trait`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypePredicate {
    /// The type being bounded.
    pub ty: Type,
    /// The bounds on the type.
    pub bounds: Vec<Type>,
}

impl PrettyPrinter for WhereClause {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if !self.predicates.is_empty() {
            printer.string(" where ");
            for (i, predicate) in self.predicates.iter().enumerate() {
                if i > 0 {
                    printer.string(", ");
                }
                predicate.pretty_print(printer)?;
            }
        }
        Ok(())
    }
}

impl PrettyPrinter for WherePredicate {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            WherePredicate::Lifetime(p) => p.pretty_print(printer),
            WherePredicate::Type(p) => p.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for LifetimePredicate {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("'");
        printer.string(&self.lifetime);
        if !self.bounds.is_empty() {
            printer.string(": ");
            for (i, bound) in self.bounds.iter().enumerate() {
                if i > 0 {
                    printer.string(" + ");
                }
                printer.string("'");
                printer.string(bound);
            }
        }
        Ok(())
    }
}

impl PrettyPrinter for TypePredicate {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.ty.pretty_print(printer)?;
        if !self.bounds.is_empty() {
            printer.string(": ");
            for (i, bound) in self.bounds.iter().enumerate() {
                if i > 0 {
                    printer.string(" + ");
                }
                bound.pretty_print(printer)?;
            }
        }
        Ok(())
    }
}
