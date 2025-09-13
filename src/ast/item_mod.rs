use crate::ast::items::Item;
use crate::ast::metadata::{self, Md};
use crate::pretty_printer::{BreakStyle, PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A `mod` item: `mod my_module;` or `mod my_module { ... }`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemMod {
    /// The name of the module.
    pub ident: String,
    /// The content of the module, if any.
    pub content: Option<ThinVec<Item>>,
    /// Metadata about the mod item.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemMod {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("mod ");
        printer.string(&self.ident);
        if let Some(content) = &self.content {
            printer.begin(BreakStyle::Consistent, " {");
            printer.hard_break();
            for item in content {
                item.pretty_print(printer)?;
                printer.hard_break();
            }
            printer.end("}");
        } else {
            printer.string(";");
            printer.hard_break();
        }
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
