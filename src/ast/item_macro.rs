use crate::ast::expressions::Expr;
use crate::ast::metadata::Md;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A macro invocation in an items position: `my_macro!();`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemMacro {
    /// The macro invocation expression.
    pub expr: Box<Expr>,
    /// Metadata about the macro invocation.
    pub md: Option<Box<Md>>,
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
        if let Some(md) = &self.md {
            for attr in &md.attrs {
                attr.pretty_print(printer)?;
                printer.hard_break();
            }
            for comment in &md.leading_comments {
                comment.pretty_print(printer)?;
            }
        }
        self.expr.pretty_print(printer)?;
        printer.string(";");
        if let Some(md) = &self.md {
            for comment in &md.trailing_comments {
                comment.pretty_print(printer)?;
            }
        }
        Ok(())
    }
}
