use crate::ast::items::Item;
use crate::ast::metadata::{self, Md};
use crate::pretty_printer::{BreakStyle, PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A foreign mod item: `extern "C" { ... }`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemForeignMod {
    /// The ABI of the foreign mod.
    pub abi: String,
    /// The items within the foreign mod.
    pub items: ThinVec<Item>,
    /// Metadata about the foreign mod item.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemForeignMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemForeignMod {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("extern ");
        printer.string(format!("\"{}\"", self.abi));
        printer.begin(BreakStyle::Consistent, " {");
        printer.hard_break();
        for item in &self.items {
            item.pretty_print(printer)?;
            printer.hard_break();
        }
        printer.end("}");
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
