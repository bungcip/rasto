use crate::ast::attributes::Attribute;
use crate::ast::comments::Comment;
use crate::ast::generics::GenericParams;
use crate::ast::items::Field;
use crate::pretty_printer::{BreakStyle, PrettyPrinter, Printer};
use std::fmt;

/// A `union` item: `union MyUnion { f1: u32, f2: f32 }`
#[derive(Debug, Clone, PartialEq)]
pub struct ItemUnion {
    /// Attributes that appear before the union item.
    pub attrs: Vec<Attribute>,
    /// Comments that appear before the union item.
    pub leading_comments: Vec<Comment>,
    /// The name of the union.
    pub ident: String,
    /// The generic parameters of the union.
    pub generics: GenericParams,
    /// The fields of the union.
    pub fields: Vec<Field>,
    /// Comments that appear after the union item.
    pub trailing_comments: Vec<Comment>,
}

impl fmt::Display for ItemUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemUnion {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for attr in &self.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &self.leading_comments {
            comment.pretty_print(printer)?;
        }
        printer.string("union ");
        printer.string(&self.ident);
        self.generics.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.fields.is_empty() {
            printer.hard_break();
            for field in &self.fields {
                field.pretty_print(printer)?;
                printer.string(",");
                printer.hard_break();
            }
        }
        printer.end("}");
        for comment in &self.trailing_comments {
            comment.pretty_print(printer)?;
        }
        Ok(())
    }
}
