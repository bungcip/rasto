use crate::ast::expressions::Expr;
use crate::ast::metadata::{self, Md};
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `static` item: `static COUNTER: u32 = 0;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStatic {
    /// The name of the static item.
    pub ident: String,
    /// The type of the static item.
    pub ty: Type,
    /// The value of the static item.
    pub expr: Box<Expr>,
    /// Metadata about the static item.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemStatic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemStatic {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("static ");
        printer.string(&self.ident);
        printer.string(": ");
        self.ty.pretty_print(printer)?;
        printer.string(" = ");
        self.expr.pretty_print(printer)?;
        printer.string(";");
        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
