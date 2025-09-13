use crate::ast::expressions::Expr;
use crate::ast::metadata::{self, Md};
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
        metadata::pp_begin(&self.md, printer)?;
        self.expr.pretty_print(printer)?;
        printer.string(";");
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
