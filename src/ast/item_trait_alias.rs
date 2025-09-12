use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A trait alias item: `pub trait ShareableIterator = Iterator + Sync;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemTraitAlias {
    /// Attributes that appear before the trait alias.
    pub attrs: Vec<Attribute>,
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
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrinter for ItemTraitAlias {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
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
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
