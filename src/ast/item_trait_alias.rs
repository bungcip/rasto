//! Defines the AST node for a trait alias.

use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents a trait alias, which is a new name for a set of trait bounds.
    ///
    /// # Example
    ///
/// ```rust,ignore
/// # #![feature(trait_alias)]
/// # trait MyTrait = Clone + Send + Sync;
    /// ```
    pub struct ItemTraitAlias as TraitAlias without vis {
        /// The list of trait bounds that the alias represents.
        pub bounds: ThinVec<String>,
    }
}
