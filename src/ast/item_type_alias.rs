//! A type alias, such as `type MyResult<T> = Result<T, MyError>`;

use crate::{
    ast::{generics::GenericParams, types::Type},
    pretty_printer::PrettyPrinter,
};

ast_item! {
    /// A type alias, such as `type MyResult<T> = Result<T, MyError>;`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rasto::builder::type_alias;
    ///
    /// let item = type_alias("MyType", "u32").build();
    /// ```
    pub struct ItemTypeAlias {
        /// The generic parameters of the type alias.
        pub(crate) generics: GenericParams,
        /// The type being aliased.
        pub(crate) ty: Type,
    }
}
