//! An extern type, such as `extern type MyType;`

use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// An extern type, such as `extern type MyType;`
    pub struct ItemExternType {}
}
