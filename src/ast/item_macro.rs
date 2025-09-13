use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A macro invocation in an items position: `my_macro!();`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemMacro {
    /// Attributes that appear before the macro invocation.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the macro invocation.
    pub leading_comments: Vec<Comment>,
    /// The macro invocation expression.
    pub expr: Box<Expr>,
    /// Comments that appear after the macro invocation.
    pub trailing_comments: Vec<Comment>,
}

impl fmt::Display for ItemMacro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemMacro {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
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
