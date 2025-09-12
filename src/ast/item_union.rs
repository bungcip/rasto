use crate::ast::comments::Comment;
use crate::ast::items::Field;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `union` item: `union MyUnion { f1: u32, f2: f32 }`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemUnion {
    /// Comments that appear before the union item.
    pub leading_comments: Vec<Comment>,
    /// The name of the union.
    pub ident: String,
    /// The fields of the union.
    pub fields: Vec<Field>,
    /// Comments that appear after the union item.
    pub trailing_comments: Vec<Comment>,
}

impl ItemUnion {
    /// Pretty-prints the union item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrinter for ItemUnion {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("union ");
        printer.string(&self.ident);
        printer.string(" {");
        printer.hard_break();
        for field in &self.fields {
            field.pretty_print(printer)?;
            printer.string(",");
            printer.hard_break();
        }
        printer.string("}");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
