//! Defines the AST nodes for a function definition.

use crate::ast::abi::Abi;
use crate::ast::generics::GenericParams;
use crate::ast::patterns::Pat;
use crate::ast::statements::Block;
use crate::ast::types::Type;
use crate::ast::where_clause::WhereClause;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents a function definition, including its signature and body.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn my_function(arg1: i32) -> i32 {
    ///     arg1 + 1
    /// }
    /// ```
    pub struct ItemFn without ident {
        /// The signature of the function, which includes its name, arguments,
        /// return type, and other properties.
        pub sig: Signature,
        /// The block of code that forms the function's body.
        pub block: Block,
    }
}

/// Represents the signature of a function, which defines its interface.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    /// `true` if the function is a `const fn`, meaning it can be evaluated at
    /// compile time.
    pub is_const: bool,
    /// `true` if the function is an `async fn`, meaning it returns a `Future`.
    pub is_async: bool,
    /// `true` if the function is `unsafe`, requiring an `unsafe` block to be
    /// called.
    pub is_unsafe: bool,
    /// The Application Binary Interface (ABI) of the function, if specified.
    /// This is typically used for FFI.
    pub abi: Option<Abi>,
    /// The name of the function.
    pub ident: String,
    /// The generic parameters of the function, such as `<T>`.
    pub generics: GenericParams,
    /// The list of input parameters (arguments) for the function.
    pub inputs: ThinVec<Pat>,
    /// `true` if the function is variadic, meaning it can accept a variable
    /// number of arguments (e.g., `...`). This is only used in `extern`
    /// function declarations.
    pub is_variadic: bool,
    /// The return type of the function. If `None`, the function returns the
    /// unit type `()`.
    pub output: Option<Type>,
    /// An optional `where` clause, which provides additional bounds on the
    /// generic parameters.
    pub where_clause: Option<WhereClause>,
}
