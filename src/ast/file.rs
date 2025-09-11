use crate::ast::items::Item;

/// A virtual file that contains a collection of items.
#[derive(Debug, Clone)]
pub struct File {
    /// The items in the file.
    pub items: Vec<Item>,
}

impl File {
    /// Pretty-prints the file to a string.
    pub fn to_string(&self) -> String {
        todo!();
    }
}
