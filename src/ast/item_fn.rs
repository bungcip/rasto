//! Defines the AST nodes for a function definition.

use crate::ast::generics::GenericParams;
use crate::ast::patterns::Pat;
use crate::ast::statements::Block;
use crate::ast::types::Type;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// A function definition.
    pub struct ItemFn without ident {
        /// The function signature.
        pub sig: Signature,
        /// The function body.
        pub block: Block,
    }
}

/// A function signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    // The `fn` token would go here.
    /// The name of the function.
    pub ident: String,
    /// The generic parameters of the function.
    pub generics: GenericParams,
    /// The arguments of the function.
    pub inputs: ThinVec<Pat>,
    /// The return type of the function.
    pub output: Option<Type>,
}
