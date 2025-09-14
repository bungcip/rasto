use crate::ast::items::Item;
use crate::ast::metadata::Md;
use crate::ast::visibility::Visibility;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A module, such as `mod my_module;` or `mod my_module { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemMod {
    /// The visibility of the module.
    pub vis: Visibility,
    /// The name of the module.
    pub ident: String,
    /// The content of the module. If this is `None`, the module is declared
    /// with a semicolon, and the content is in a separate file.
    pub content: Option<ThinVec<Item>>,
    /// Metadata about the module, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemMod {
    /// Formats the `ItemMod` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}
