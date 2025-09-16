//! Defines the AST node for a trait alias.

use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// A trait alias, such as `pub trait ShareableIterator = Iterator + Sync;`.
    pub struct ItemTraitAlias without vis {
        /// The bounds of the trait alias.
        pub bounds: ThinVec<String>,
    }
}
