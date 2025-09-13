use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::items::Item;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A foreign mod item: `extern "C" { ... }`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemForeignMod {
    /// Attributes that appear before the foreign mod item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the foreign mod item.
    pub leading_comments: Vec<Comment>,
    /// The ABI of the foreign mod.
    pub abi: String,
    /// The items within the foreign mod.
    pub items: Vec<Item>,
    /// Comments that appear after the foreign mod item.
    pub trailing_comments: Vec<Comment>,
}

impl fmt::Display for ItemForeignMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemForeignMod {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("extern ");
        printer.string(format!("\"{}\"", self.abi));
        printer.string(" {");
        printer.hard_break();
        for item in &self.items {
            item.pretty_print(printer)?;
            printer.hard_break();
        }
        printer.string("}");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
