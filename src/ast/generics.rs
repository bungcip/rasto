//! The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes
//! that represent Rust generics.

use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// Creates a new `GenericParamBuilder`.
pub fn generic_param() -> GenericParamBuilder {
    GenericParamBuilder
}

/// A builder for creating `GenericParam`s.
#[derive(Clone, Copy)]
pub struct GenericParamBuilder;

impl GenericParamBuilder {
    /// Creates a new `TypeParam`.
    ///
    /// # Parameters
    ///
    /// - `ident`: The name of the type parameter.
    pub fn ty(self, ident: impl Into<String>) -> TypeParam {
        TypeParam {
            ident: ident.into(),
            bounds: vec![],
        }
    }

    /// Creates a new `LifetimeParam`.
    ///
    /// # Parameters
    ///
    /// - `ident`: The name of the lifetime parameter.
    pub fn lifetime(self, ident: impl Into<String>) -> LifetimeParam {
        LifetimeParam {
            ident: ident.into(),
        }
    }

    /// Creates a new `ConstParam`.
    ///
    /// # Parameters
    ///
    /// - `ident`: The name of the const parameter.
    /// - `ty`: The `Type` of the const parameter.
    pub fn const_(self, ident: impl Into<String>, ty: impl Into<Type>) -> ConstParam {
        ConstParam {
            ident: ident.into(),
            ty: ty.into(),
        }
    }
}

/// A set of generic parameters, such as `<'a, T: Trait, const N: usize>`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GenericParams {
    /// The generic parameters.
    pub params: Vec<GenericParam>,
}

impl GenericParams {
    /// Creates a new, empty set of generic parameters.
    pub fn new() -> Self {
        Self { params: vec![] }
    }
}

/// A single generic parameter.
#[derive(Debug, Clone, PartialEq)]
pub enum GenericParam {
    /// A lifetime parameter: `'a`.
    Lifetime(LifetimeParam),
    /// A type parameter: `T: Trait`.
    Type(TypeParam),
    /// A const parameter: `const N: usize`.
    Const(ConstParam),
}

impl From<LifetimeParam> for GenericParam {
    /// Converts a `LifetimeParam` into a `GenericParam::Lifetime` variant.
    fn from(param: LifetimeParam) -> Self {
        GenericParam::Lifetime(param)
    }
}

impl From<TypeParam> for GenericParam {
    /// Converts a `TypeParam` into a `GenericParam::Type` variant.
    fn from(param: TypeParam) -> Self {
        GenericParam::Type(param)
    }
}

impl From<ConstParam> for GenericParam {
    /// Converts a `ConstParam` into a `GenericParam::Const` variant.
    fn from(param: ConstParam) -> Self {
        GenericParam::Const(param)
    }
}

/// A lifetime parameter, such as `'a`.
#[derive(Debug, Clone, PartialEq)]
pub struct LifetimeParam {
    /// The name of the lifetime, without the leading apostrophe.
    pub ident: String,
}

/// A type parameter, such as `T: Trait`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeParam {
    /// The name of the type parameter.
    pub ident: String,
    /// The bounds on the type parameter.
    pub bounds: Vec<Type>,
}

/// A const parameter, such as `const N: usize`.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstParam {
    /// The name of the const parameter.
    pub ident: String,
    /// The type of the const parameter.
    pub ty: Type,
}

impl PrettyPrinter for GenericParams {
    /// Pretty-prints the `GenericParams` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if !self.params.is_empty() {
            printer.string("<");
            for (i, param) in self.params.iter().enumerate() {
                if i > 0 {
                    printer.string(", ");
                }
                param.pretty_print(printer)?;
            }
            printer.string(">");
        }
        Ok(())
    }
}

impl PrettyPrinter for GenericParam {
    /// Pretty-prints the `GenericParam` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            GenericParam::Lifetime(p) => p.pretty_print(printer),
            GenericParam::Type(p) => p.pretty_print(printer),
            GenericParam::Const(p) => p.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for LifetimeParam {
    /// Pretty-prints the `LifetimeParam` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("'");
        printer.string(&self.ident);
        Ok(())
    }
}

impl PrettyPrinter for TypeParam {
    /// Pretty-prints the `TypeParam` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.ident);
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

impl PrettyPrinter for ConstParam {
    /// Pretty-prints the `ConstParam` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("const ");
        printer.string(&self.ident);
        printer.string(": ");
        self.ty.pretty_print(printer)
    }
}

/// A set of generic arguments, such as `<'a, T, 42>`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GenericArgs {
    /// The generic arguments.
    pub args: Vec<GenericArg>,
}

impl GenericArgs {
    /// Creates a new, empty set of generic arguments.
    pub fn new() -> Self {
        Self { args: vec![] }
    }
}

/// A single generic argument.
#[derive(Debug, Clone, PartialEq)]
pub enum GenericArg {
    /// A lifetime argument: `'a`.
    Lifetime(String),
    /// A type argument: `T`.
    Type(Type),
    /// A const argument: `N`.
    Const(crate::ast::Expr),
}

impl From<&str> for GenericArg {
    fn from(s: &str) -> Self {
        GenericArg::Type(Type::from(s))
    }
}
