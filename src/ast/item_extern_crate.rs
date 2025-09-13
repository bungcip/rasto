use crate::ast::metadata::{self, Md};
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// An `extern crate` item: `extern crate serde;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemExternCrate {
    /// The name of the crate.
    pub ident: String,
    /// Metadata about the extern crate item.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemExternCrate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemExternCrate {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("extern crate ");
        printer.string(&self.ident);
        printer.string(";");
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
