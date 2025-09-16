//! Defines the AST node for a trait definition.

use crate::ast::associated_type::AssociatedType;
use crate::ast::items::TraitItem;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents a `trait` definition, which is a collection of methods and
    /// associated types that define a shared behavior.
    ///
    /// # Example
    ///
    /// ```rust
    /// trait MyTrait {
    ///     type MyType;
    ///     fn my_method(&self);
    /// }
    /// ```
    pub struct ItemTrait with generics {
        /// The list of associated types defined in the trait.
        pub associated_types: ThinVec<AssociatedType>,
        /// The list of items within the trait, such as methods and associated
        /// constants.
        pub items: ThinVec<TraitItem>,
    }
}
