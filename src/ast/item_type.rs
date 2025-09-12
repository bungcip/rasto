use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `type` item: `type MyResult<T> = Result<T, MyError>;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemType {
    /// Attributes that appear before the type item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the type item.
    pub leading_comments: Vec<Comment>,
    /// The name of the type alias.
    pub ident: String,
    /// The type being aliased.
    pub ty: Type,
    /// Comments that appear after the type item.
    pub trailing_comments: Vec<Comment>,
}

impl ItemType {
    /// Pretty-prints the type item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrinter for ItemType {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("type ");
        printer.string(&self.ident);
        printer.string(" = ");
        self.ty.pretty_print(printer)?;
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
