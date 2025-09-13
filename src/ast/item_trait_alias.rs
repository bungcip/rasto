use crate::ast::metadata::Md;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A trait alias item: `pub trait ShareableIterator = Iterator + Sync;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemTraitAlias {
    /// The name of the trait alias.
    pub ident: String,
    /// The bounds of the trait alias.
    pub bounds: ThinVec<String>,
    /// Metadata about the trait alias.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemTraitAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemTraitAlias {
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
        if let Some(md) = &self.md {
            for comment in &md.trailing_comments {
                comment.pretty_print(printer)?;
            }
        }
        Ok(())
    }
}
