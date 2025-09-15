use crate::pretty_printer::PrettyPrinter;

ast_item! {
    /// An `extern crate` item, such as `extern crate serde;`.
    pub struct ItemExternCrate without vis {}
}
