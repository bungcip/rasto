//! Defines the AST node for an associated type in a trait.

use crate::ast::{generics::GenericParams, ident::Ident, metadata::Md, types::Type};
use thin_vec::ThinVec;

/// Represents an associated type within a trait.
///
/// An associated type is a placeholder type used in a trait definition, which is
/// then specified by the implementing type.
///
/// # Examples
///
/// A simple associated type:
///
/// ```rust
/// trait Iterator {
///     type Item;
///     // ...
/// }
/// ```
///
/// An associated type with bounds:
///
/// ```rust
/// trait MyTrait {
///     type MyType: Clone + Default;
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedType {
    /// The identifier of the associated type.
    ///
    /// For `type Item;`, the ident is `Item`.
    pub ident: Ident,
    /// The generic parameters for the associated type.
    ///
    /// For `type Item<T>;`, the generics are `<T>`.
    pub generics: GenericParams,
    /// The trait bounds that the associated type must satisfy.
    ///
    /// For `type Item: Clone;`, the bounds are `Clone`.
    pub bounds: ThinVec<Type>,
    /// An optional default type.
    ///
    /// For `type Item = u32;`, the default is `u32`.
    pub default: Option<Type>,
    /// Metadata, such as attributes and comments, attached to the associated type.
    pub md: Option<Box<Md>>,
}
