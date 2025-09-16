use crate::ast::item_fn::ItemFn;
use crate::ast::item_macro::ItemMacro;
use crate::ast::types::Type;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// An `extern` block, such as `extern "C" { ... }`.
    pub struct ItemExternBlock without ident {
        /// Whether the `extern` block is `unsafe`.
        pub is_unsafe: bool,
        /// The ABI of the `extern` block, such as `"C"`.
        pub abi: Option<String>,
        /// The items inside the `extern` block.
        pub items: ThinVec<ExternalItem>,
    }
}

/// An item inside an `extern` block.
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalItem {
    /// A `static` item, such as `static FOO: i32;`.
    Static(String, Type),
    /// A function, such as `fn foo();`.
    Fn(ItemFn),
    /// A macro invocation, such as `my_macro!();`.
    Macro(ItemMacro),
}
