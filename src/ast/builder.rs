use crate::ast::{Block, File, Item, ItemFn, Signature};

pub struct FileBuilder {
    items: Vec<Item>,
}

impl FileBuilder {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn build(self) -> File {
        File { items: self.items }
    }
}

pub struct FnBuilder {
    ident: Option<String>,
    block: Option<Block>,
}

impl FnBuilder {
    pub fn new() -> Self {
        Self {
            ident: None,
            block: None,
        }
    }

    pub fn name(mut self, ident: impl Into<String>) -> Self {
        self.ident = Some(ident.into());
        self
    }

    pub fn block(mut self, block: Block) -> Self {
        self.block = Some(block);
        self
    }

    pub fn build(self) -> ItemFn {
        let ident = self.ident.expect("name is required");
        let block = self.block.expect("block is required");

        ItemFn {
            leading_comments: vec![],
            sig: Signature { ident },
            block,
            trailing_comments: vec![],
        }
    }
}
