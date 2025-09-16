//! Defines the AST node for a trait definition.

use crate::ast::associated_type::AssociatedType;
use crate::ast::items::TraitItem;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// A trait definition, such as `trait Foo { ... }`.
    pub struct ItemTrait with generics {
        /// The associated types of the trait.
        pub associated_types: ThinVec<AssociatedType>,
        /// The items within the trait, such as methods and associated types.
        pub items: ThinVec<TraitItem>,
    }
}
