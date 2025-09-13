//! The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes
//! that represent Rust types.

use crate::ast::Expr;
use crate::ast::expressions::{Path, PathSegment};
use crate::ast::item_macro::ItemMacro;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::{ThinVec, thin_vec};

/// A Rust type.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// A fixed size array type: `[T; n]`.
    Array(TypeArray),

    /// A bare function type: `fn(usize) -> bool`.
    BareFn(TypeBareFn),

    /// A type contained within invisible delimiters.
    Group(Box<Type>),

    /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
    /// a lifetime.
    ImplTrait,

    /// Indication that a type should be inferred by the compiler: `_`.
    Infer,

    /// A macro in the type position.
    Macro(ItemMacro),

    /// The never type: `!`.
    Never,

    /// A parenthesized type equivalent to the inner type.
    Paren(Box<Type>),

    /// A path like `std::slice::Iter`, optionally qualified with a
    /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
    Path(TypePath),

    /// A raw pointer type: `*const T` or `*mut T`.
    Ptr(TypePtr),

    /// A reference type: `&'a T` or `&'a mut T`.
    Reference(TypeReference),

    /// A dynamically sized slice type: `[T]`.
    Slice(Box<Type>),

    /// A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
    /// trait or a lifetime.
    TraitObject,

    /// A tuple type: `(A, B, C, String)`.
    Tuple(ThinVec<Type>),
}

/// A fixed size array type: `[T; n]`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeArray {
    /// The element type.
    pub elem: Box<Type>,
    /// The length of the array.
    pub len: Box<Expr>,
}

/// A bare function type: `fn(usize) -> bool`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeBareFn {
    /// The input types.
    pub inputs: ThinVec<Type>,
    /// The output type.
    pub output: Option<Box<Type>>,
}

/// A path like `std::slice::Iter`, optionally qualified with a
/// self-type as in `<Vec<T> as SomeTrait>::Associated`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypePath {
    /// The path itself.
    pub path: Path,
    // Note: We are not including generics for now for simplicity.
}

/// A raw pointer type: `*const T` or `*mut T`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypePtr {
    /// The pointed-to type.
    pub elem: Box<Type>,
    /// Whether the pointer is mutable.
    pub mutable: bool,
}

/// A reference type: `&'a T` or `&'a mut T`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    /// The lifetime of the reference.
    pub lifetime: Option<String>,
    /// The referenced type.
    pub elem: Box<Type>,
    /// Whether the reference is mutable.
    pub mutable: bool,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        Type::Path(TypePath {
            path: Path {
                segments: thin_vec![PathSegment {
                    ident: s.to_string(),
                    args: None,
                }],
            },
        })
    }
}

impl PrettyPrinter for Type {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Type::Array(array) => array.pretty_print(printer),
            Type::BareFn(bare_fn) => bare_fn.pretty_print(printer),
            Type::Group(group) => group.pretty_print(printer),
            Type::ImplTrait => {
                printer.string("impl Trait");
                Ok(())
            }
            Type::Infer => {
                printer.string("_");
                Ok(())
            }
            Type::Macro(mac) => mac.pretty_print(printer),
            Type::Never => {
                printer.string("!");
                Ok(())
            }
            Type::Paren(paren) => {
                printer.string("(");
                paren.pretty_print(printer)?;
                printer.string(")");
                Ok(())
            }
            Type::Path(path) => path.pretty_print(printer),
            Type::Ptr(ptr) => ptr.pretty_print(printer),
            Type::Reference(reference) => reference.pretty_print(printer),
            Type::Slice(slice) => {
                printer.string("[");
                slice.pretty_print(printer)?;
                printer.string("]");
                Ok(())
            }
            Type::TraitObject => {
                printer.string("dyn Trait");
                Ok(())
            }
            Type::Tuple(tuple) => {
                printer.string("(");
                for (i, ty) in tuple.iter().enumerate() {
                    if i > 0 {
                        printer.string(", ");
                    }
                    ty.pretty_print(printer)?;
                }
                if tuple.len() == 1 {
                    printer.string(",");
                }
                printer.string(")");
                Ok(())
            }
        }
    }
}

impl PrettyPrinter for TypeArray {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("[");
        self.elem.pretty_print(printer)?;
        printer.string("; ");
        self.len.pretty_print(printer)?;
        printer.string("]");
        Ok(())
    }
}

impl PrettyPrinter for TypeBareFn {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("fn(");
        for (i, ty) in self.inputs.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            ty.pretty_print(printer)?;
        }
        printer.string(")");
        if let Some(output) = &self.output {
            printer.string(" -> ");
            output.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for TypePath {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)
    }
}

impl PrettyPrinter for TypePtr {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("*");
        if self.mutable {
            printer.string("mut ");
        } else {
            printer.string("const ");
        }
        self.elem.pretty_print(printer)
    }
}

impl PrettyPrinter for TypeReference {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("&");
        if let Some(lifetime) = &self.lifetime {
            printer.string(lifetime);
            printer.string(" ");
        }
        if self.mutable {
            printer.string("mut ");
        }
        self.elem.pretty_print(printer)
    }
}
