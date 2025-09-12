use crate::ast::comments::Comment;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// A trait alias item: `pub trait ShareableIterator = Iterator + Sync;`
#[derive(Debug, Clone)]
pub struct ItemTraitAlias {
    /// Comments that appear before the trait alias.
    pub leading_comments: Vec<Comment>,
    /// The name of the trait alias.
    pub ident: String,
    /// The bounds of the trait alias.
    pub bounds: Vec<String>,
    /// Comments that appear after the trait alias.
    pub trailing_comments: Vec<Comment>,
}

impl ItemTraitAlias {
    /// Pretty-prints the trait alias item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemTraitAlias {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
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
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
