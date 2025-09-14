use crate::ast::generics::GenericParams;
use crate::ast::items::Field;
use crate::ast::metadata::{self, Md};
use crate::pretty_printer::{BreakStyle, PrettyPrinter, Printer};
use std::fmt;
use thin_vec::ThinVec;

/// A `union` definition, such as `union MyUnion { f1: u32, f2: f32 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemUnion {
    /// The name of the union.
    pub ident: String,
    /// The generic parameters of the union.
    pub generics: GenericParams,
    /// The fields of the union.
    pub fields: ThinVec<Field>,
    /// Metadata about the union, including attributes and comments.
    pub md: Option<Box<Md>>,
}

impl fmt::Display for ItemUnion {
    /// Formats the `ItemUnion` using the pretty-printer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = Printer::new(f);
        self.pretty_print(&mut printer)?;
        printer.finish()
    }
}

impl PrettyPrinter for ItemUnion {
    /// Pretty-prints the `ItemUnion` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        metadata::pp_begin(&self.md, printer)?;
        printer.string("union ");
        printer.string(&self.ident);
        self.generics.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.fields.is_empty() {
            printer.hard_break();
            let num_fields = self.fields.len();
            for (i, field) in self.fields.iter().enumerate() {
                field.pretty_print(printer)?;
                printer.string(",");
                if i < num_fields - 1 {
                    printer.hard_break();
                }
            }
        }
        printer.end("}");

        metadata::pp_end(&self.md, printer)?;
        Ok(())
    }
}
