use crate::ast::associated_type::AssociatedType;
use crate::ast::generics::GenericParams;
use crate::ast::items::TraitItem;
use crate::ast::metadata::Md;
use thin_vec::ThinVec;

/// A trait item: `trait Foo { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemTrait {
    /// The name of the trait.
    pub ident: String,
    /// The generic parameters of the trait.
    pub generics: GenericParams,
    /// The associated types of the trait.
    pub associated_types: ThinVec<AssociatedType>,
    /// The items within the trait.
    pub items: ThinVec<TraitItem>,
    /// Metadata about the trait.
    pub md: Option<Box<Md>>,
}
