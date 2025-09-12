use crate::ast::comments::Comment;
use crate::ast::expressions::Expr;
use crate::ast::types::Type;
use crate::pretty_printer_v2::{PrettyPrintV2, Printer};
use std::fmt;

/// A `const` item: `const MAX: u16 = 234342;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemConst {
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

impl ItemConst {
    /// Pretty-prints the const item to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut printer = Printer::new(&mut buf);
        self.pretty_print_v2(&mut printer).unwrap();
        printer.finish().unwrap();
        buf
    }
}

impl PrettyPrintV2 for ItemConst {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("const ");
        printer.string(&self.ident);
        printer.string(": ");
        self.ty.pretty_print_v2(printer)?;
        printer.string(" = ");
        self.expr.pretty_print_v2(printer)?;
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}
