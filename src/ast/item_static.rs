use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `static` item: `static COUNTER: u32 = 0;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStatic {
    /// Attributes that appear before the static item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the static item.
    pub leading_comments: Vec<Comment>,
    /// The name of the static item.
    pub ident: String,
    /// The type of the static item.
    pub ty: Type,
    /// The value of the static item.
    pub expr: Box<Expr>,
    /// Comments that appear after the static item.
    pub trailing_comments: Vec<Comment>,
}

use std::fmt::{Display, Formatter};

impl Display for ItemStatic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print(&mut printer).unwrap();
        printer.finish().unwrap();
        write!(f, "{buf}")
    }
}

impl PrettyPrinter for ItemStatic {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("static ");
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
