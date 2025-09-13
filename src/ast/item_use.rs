use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `use` item: `use std::collections::HashMap;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemUse {
    /// Attributes that appear before the use item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the use item.
    pub leading_comments: Vec<Comment>,
    /// The path being used.
    pub path: String,
    /// Comments that appear after the use item.
    pub trailing_comments: Vec<Comment>,
}

use std::fmt::{Display, Formatter};

impl Display for ItemUse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        write!(f, "{buf}")
    }
}

impl PrettyPrinter for ItemUse {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("use ");
        printer.string(&self.path);
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
