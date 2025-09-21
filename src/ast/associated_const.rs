//! An associated constant in a trait or impl.

use crate::ast::expressions::Expr;
use crate::ast::ident::Ident;
use crate::ast::metadata::Md;
use crate::ast::types::Type;

/// Represents an associated constant in a trait or implementation.
///
/// An associated constant is a constant value that is associated with a specific
/// type. It is defined within a trait or an `impl` block.
///
/// # Examples
///
/// In a trait definition:
///
/// ```rust
/// trait MyTrait {
///     const MAX: u16;
/// }
/// ```
///
/// In an `impl` block:
///
/// ```rust
/// trait MyTrait {
///     const MAX: u16;
/// }
/// struct MyType;
///
/// impl MyTrait for MyType {
///     const MAX: u16 = 123;
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedConst {
    /// The name of the associated constant.
    pub ident: Ident,
    /// The type of the associated constant.
    pub ty: Type,
    /// The optional expression that provides the value of the constant.
    ///
    /// This is `None` in a trait definition where the value is not specified,
    /// and `Some` in an `impl` block where the value is provided.
    pub expr: Option<Box<Expr>>,
    /// Metadata associated with the constant, such as attributes and comments.
    pub md: Option<Box<Md>>,
}
