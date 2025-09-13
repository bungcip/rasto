use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `const` item: `const MAX: u16 = 234342;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemConst {
    /// Attributes that appear before the const item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the const item.
    pub leading_comments: Vec<Comment>,
    /// The name of the const item.
    pub ident: String,
    /// The type of the const item.
    pub ty: Type,
    /// The value of the const item.
    pub expr: Box<Expr>,
    /// Comments that appear after the const item.
    pub trailing_comments: Vec<Comment>,
}

impl fmt::Display for ItemConst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemConst {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("const ");
        printer.string(&self.ident);
        printer.string(": ");
        self.ty.pretty_print(printer)?;
        printer.string(" = ");
        self.expr.pretty_print(printer)?;
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
