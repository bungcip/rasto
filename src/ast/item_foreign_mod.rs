use crate::ast::comments::Comment;
use crate::ast::items::Item;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// A foreign mod item: `extern "C" { ... }`
#[derive(Debug, Clone)]
pub struct ItemForeignMod {
    /// Comments that appear before the foreign mod item.
    pub leading_comments: Vec<Comment>,
    /// The ABI of the foreign mod.
    pub abi: String,
    /// The items within the foreign mod.
    pub items: Vec<Item>,
    /// Comments that appear after the foreign mod item.
    pub trailing_comments: Vec<Comment>,
}

impl ItemForeignMod {
    /// Pretty-prints the foreign mod item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemForeignMod {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("extern ");
        printer.string(format!("\"{}\"", self.abi));
        printer.string(" {");
        printer.hard_break();
        for item in &self.items {
            item.pretty_print_v2(printer)?;
            printer.hard_break();
        }
        printer.string("}");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
