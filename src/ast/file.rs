use crate::ast::items::Item;
use crate::pretty_print::{Formatter, PrettyPrint};

/// A virtual file that contains a collection of items.
#[derive(Debug, Clone)]
pub struct File {
    /// The items in the file.
    pub items: Vec<Item>,
}

impl File {
    /// Pretty-prints the file to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut fmt = Formatter::new(&mut buf);
        self.pretty_print(&mut fmt).unwrap();
        buf
    }
}
