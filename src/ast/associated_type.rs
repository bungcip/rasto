use crate::ast::generics::GenericParams;
use crate::ast::metadata::Md;
use crate::ast::types::Type;
use thin_vec::ThinVec;

/// Represents an associated type in a trait definition.
///
/// For example: `type Output: ?Sized;`
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedType {
    /// The name of the associated type.
    ///
    /// In `type Output: ?Sized;`, this would be `Output`.
    pub ident: String,
    /// The generic parameters of the associated type.
    ///
    /// For example, in `type Item<T> where T: Clone;`, this would be `<T> where T: Clone`.
    pub generics: GenericParams,
    /// The bounds on the associated type.
    ///
    /// In `type Output: ?Sized;`, this would be `?Sized`.
    pub bounds: ThinVec<Type>,
    /// The default type for the associated type.
    ///
    /// In `type Output = u32;`, this would be `u32`.
    pub default: Option<Type>,
    /// Metadata about the associated type, including attributes and comments.
    pub md: Option<Box<Md>>,
}
