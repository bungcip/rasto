use crate::ast::metadata::{self, Md};
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A trait alias, such as `pub trait ShareableIterator = Iterator + Sync;`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemTraitAlias {
    /// The name of the trait alias.
    pub ident: String,
    /// The bounds of the trait alias.
    pub bounds: ThinVec<String>,
    /// Metadata about the trait alias, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemTraitAlias {
    /// Formats the `ItemTraitAlias` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemTraitAlias {
    /// Pretty-prints the `ItemTraitAlias` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("trait ");
        printer.string(&self.ident);
        printer.string(" = ");
        for (i, bound) in self.bounds.iter().enumerate() {
            if i > 0 {
                printer.string(" + ");
            }
            printer.string(bound);
        }
        printer.string(";");
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
