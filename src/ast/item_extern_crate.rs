use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// An `extern crate` item: `extern crate serde;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemExternCrate {
    /// Attributes that appear before the extern crate item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the extern crate item.
    pub leading_comments: Vec<Comment>,
    /// The name of the crate.
    pub ident: String,
    /// Comments that appear after the extern crate item.
    pub trailing_comments: Vec<Comment>,
}

use std::fmt::{Display, Formatter};

impl Display for ItemExternCrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        write!(f, "{buf}")
    }
}

impl PrettyPrinter for ItemExternCrate {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("extern crate ");
        printer.string(&self.ident);
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
