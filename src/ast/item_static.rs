use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// A `static` item: `static COUNTER: u32 = 0;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStatic {
    /// Comments that appear before the static item.
    pub leading_comments: Vec<Comment>,
    /// The name of the static item.
    pub ident: String,
    /// The type of the static item.
    pub ty: String,
    /// The value of the static item.
    pub expr: Box<Expr>,
    /// Comments that appear after the static item.
    pub trailing_comments: Vec<Comment>,
}

impl ItemStatic {
    /// Pretty-prints the static item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemStatic {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("static ");
        printer.string(&self.ident);
        printer.string(": ");
        printer.string(&self.ty);
        printer.string(" = ");
        self.expr.pretty_print_v2(printer)?;
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
