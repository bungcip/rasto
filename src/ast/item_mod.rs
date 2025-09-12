use crate::ast::comments::Comment;
use crate::ast::items::Item;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// A `mod` item: `mod my_module;` or `mod my_module { ... }`
#[derive(Debug, Clone)]
pub struct ItemMod {
    /// Comments that appear before the mod item.
    pub leading_comments: Vec<Comment>,
    /// The name of the module.
    pub ident: String,
    /// The content of the module, if any.
    pub content: Option<Vec<Item>>,
    /// Comments that appear after the mod item.
    pub trailing_comments: Vec<Comment>,
}

impl ItemMod {
    /// Pretty-prints the mod item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemMod {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("mod ");
        printer.string(&self.ident);
        if let Some(content) = &self.content {
            printer.string(" {");
            printer.hard_break();
            for item in content {
                item.pretty_print_v2(printer)?;
                printer.hard_break();
            }
            printer.string("}");
        } else {
            printer.string(";");
        }
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
