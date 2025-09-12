use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A macro invocation in an items position: `my_macro!();`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemMacro {
    /// Comments that appear before the macro invocation.
    pub leading_comments: Vec<Comment>,
    /// The macro invocation expression.
    pub expr: Box<Expr>,
    /// Comments that appear after the macro invocation.
    pub trailing_comments: Vec<Comment>,
}

impl ItemMacro {
    /// Pretty-prints the macro item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrinter for ItemMacro {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        self.expr.pretty_print(printer)?;
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
