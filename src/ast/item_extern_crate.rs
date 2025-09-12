use crate::ast::comments::Comment;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// An `extern crate` item: `extern crate serde;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemExternCrate {
    /// Comments that appear before the extern crate item.
    pub leading_comments: Vec<Comment>,
    /// The name of the crate.
    pub ident: String,
    /// Comments that appear after the extern crate item.
    pub trailing_comments: Vec<Comment>,
}

impl ItemExternCrate {
    /// Pretty-prints the extern crate item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemExternCrate {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("extern crate ");
        printer.string(&self.ident);
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
