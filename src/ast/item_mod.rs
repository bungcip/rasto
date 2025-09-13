use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::items::Item;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `mod` item: `mod my_module;` or `mod my_module { ... }`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemMod {
    /// Attributes that appear before the mod item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the mod item.
    pub leading_comments: Vec<Comment>,
    /// The name of the module.
    pub ident: String,
    /// The content of the module, if any.
    pub content: Option<Vec<Item>>,
    /// Comments that appear after the mod item.
    pub trailing_comments: Vec<Comment>,
}

use std::fmt::{Display, Formatter};

impl Display for ItemMod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        write!(f, "{buf}")
    }
}

impl PrettyPrinter for ItemMod {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("mod ");
        printer.string(&self.ident);
        if let Some(content) = &self.content {
            printer.string(" {");
            printer.hard_break();
            for item in content {
                item.pretty_print(printer)?;
                printer.hard_break();
            }
            printer.string("}");
        } else {
            printer.string(";");
        }
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
