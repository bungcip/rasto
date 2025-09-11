use crate::ast::{Block, ItemFn, Signature};

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
