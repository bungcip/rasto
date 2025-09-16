use crate::ast::abi::Abi;
use crate::ast::generics::GenericParams;
use crate::ast::patterns::Pat;
use crate::ast::statements::Block;
use crate::ast::types::Type;
use crate::ast::where_clause::WhereClause;
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
    /// Whether the function is `const`.
    pub is_const: bool,
    /// Whether the function is `async`.
    pub is_async: bool,
    /// Whether the function is `unsafe`.
    pub is_unsafe: bool,
    /// The ABI of the function, if any.
    pub abi: Option<Abi>,
    // The `fn` token would go here.
    /// The name of the function.
    pub ident: String,
    /// The generic parameters of the function.
    pub generics: GenericParams,
    /// The arguments of the function.
    pub inputs: ThinVec<Pat>,
    /// Whether the function is variadic.
    pub is_variadic: bool,
    /// The return type of the function.
    pub output: Option<Type>,
    /// The `where` clause of the function.
    pub where_clause: Option<WhereClause>,
}
