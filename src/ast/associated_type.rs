use crate::ast::generics::GenericParams;
use crate::ast::metadata::Md;
use crate::ast::types::Type;
use thin_vec::ThinVec;

/// An associated type in a trait.
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedType {
    /// The name of the associated type.
    pub ident: String,
    /// The generic parameters of the associated type.
    pub generics: GenericParams,
    /// The bounds on the associated type.
    pub bounds: ThinVec<Type>,
    /// The default type for the associated type.
    pub default: Option<Type>,
    /// Metadata about the associated type.
    pub md: Option<Box<Md>>,
}
