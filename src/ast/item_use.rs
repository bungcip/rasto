use crate::ast::comments::Comment;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// A `use` item: `use std::collections::HashMap;`
#[derive(Debug, Clone)]
pub struct ItemUse {
    /// Comments that appear before the use item.
    pub leading_comments: Vec<Comment>,
    /// The path being used.
    pub path: String,
    /// Comments that appear after the use item.
    pub trailing_comments: Vec<Comment>,
}

impl ItemUse {
    /// Pretty-prints the use item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemUse {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("use ");
        printer.string(&self.path);
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
