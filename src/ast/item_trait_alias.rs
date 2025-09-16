use crate::pretty_printer::PrettyPrinter;
use compact_str::CompactString;
use thin_vec::ThinVec;

ast_item! {
    /// A trait alias, such as `pub trait ShareableIterator = Iterator + Sync;`.
    pub struct ItemTraitAlias without vis {
        /// The bounds of the trait alias.
        pub bounds: ThinVec<CompactString>,
    }
}
