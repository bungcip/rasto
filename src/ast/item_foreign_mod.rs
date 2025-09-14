use crate::ast::items::Item;
use crate::ast::metadata::{self, Md};
use crate::pretty_printer::{BreakStyle, PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A foreign module, such as `extern "C" { ... }`.
///
/// This contains a list of items that are defined in a foreign library.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemForeignMod {
    /// The ABI of the foreign module, e.g., `"C"`.
    pub abi: String,
    /// The items within the foreign module.
    pub items: ThinVec<Item>,
    /// Metadata about the foreign module, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemForeignMod {
    /// Formats the `ItemForeignMod` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemForeignMod {
    /// Pretty-prints the `ItemForeignMod` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("extern ");
        printer.string(format!("\"{}\"", self.abi));
        printer.begin(BreakStyle::Consistent, " {");
        if !self.items.is_empty() {
            printer.hard_break();
            let num_items = self.items.len();
            for (i, item) in self.items.iter().enumerate() {
                item.pretty_print(printer)?;
                if i < num_items - 1 {
                    printer.hard_break();
                }
            }
        }
        printer.end("}");
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
