use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::generics::GenericParams;
use crate::ast::types::Type;
use crate::pretty_printer::{PrettyPrinter, Printer};
use std::fmt;

/// A `type` item: `type MyResult<T> = Result<T, MyError>;`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemType {
    /// Attributes that appear before the type item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the type item.
    pub leading_comments: Vec<Comment>,
    /// The name of the type alias.
    pub ident: String,
    /// The generic parameters of the type alias.
    pub generics: GenericParams,
    /// The type being aliased.
    pub ty: Type,
    /// Comments that appear after the type item.
    pub trailing_comments: Vec<Comment>,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemType {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("type ");
        printer.string(&self.ident);
        self.generics.pretty_print(printer)?;
        printer.string(" = ");
        self.ty.pretty_print(printer)?;
        printer.string(";");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
