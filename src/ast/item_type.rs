use crate::ast::comments::Comment;
use crate::ast::types::Type;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// A `type` item: `type MyResult<T> = Result<T, MyError>;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemType {
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
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemType {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("type ");
        printer.string(&self.ident);
        printer.string(" = ");
        self.ty.pretty_print_v2(printer)?;
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
