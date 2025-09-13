use crate::ast::metadata::Md;
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
        if let Some(md) = &self.md {
            for attr in &md.attrs {
                attr.pretty_print(printer)?;
                printer.hard_break();
            }
            for comment in &md.leading_comments {
                comment.pretty_print(printer)?;
            }
        }
        printer.string("extern crate ");
        printer.string(&self.ident);
        printer.string(";");
        if let Some(md) = &self.md {
            for comment in &md.trailing_comments {
                comment.pretty_print(printer)?;
            }
        }
        Ok(())
    }
}
