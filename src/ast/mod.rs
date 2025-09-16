//! The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes.
//!
//! This module defines the data structures that represent the syntactic structure of Rust code.
//! It includes nodes for expressions, statements, items, literals, and more. The AST is used
//! by the pretty-printer to generate formatted Rust code.

#[macro_use]
mod macros;

/// Defines the AST node for an `extern` block.
pub mod abi;
/// Defines the AST node for an associated const in a trait.
pub mod associated_const;
/// Defines the AST node for an associated type in a trait.
pub mod associated_type;
/// Defines the AST nodes for attributes.
pub mod attributes;
/// Defines the AST nodes for comments.
pub mod comments;
/// Defines the AST nodes for expressions.
pub mod expressions;
/// Defines the AST node for a file.
pub mod file;
/// Defines the AST nodes for generics.
pub mod generics;
/// Defines the AST node for an `asm!` expression.
pub mod item_asm;
/// Defines the AST node for a `const`, `static`, or `type` item.
pub mod item_def;
/// Defines the AST node for an enum.
pub mod item_enum;
/// Defines the AST node for an `extern` block.
pub mod item_extern_block;
/// Defines the AST node for an `extern crate` item.
pub mod item_extern_crate;
/// Defines the AST node for a function.
pub mod item_fn;
/// Defines the AST node for a foreign module.
pub mod item_foreign_mod;
/// Defines the AST node for an `impl` item.
pub mod item_impl;
/// Defines the AST node for a macro definition.
pub mod item_macro;
/// Defines the AST node for a module.
pub mod item_mod;
/// Defines the AST node for a static item.
pub mod item_static;
/// Defines the AST node for a struct.
pub mod item_struct;
/// Defines the AST node for a trait.
pub mod item_trait;
/// Defines the AST node for a trait alias.
pub mod item_trait_alias;
/// Defines the AST node for a `union` item.
pub mod item_union;
/// Defines the AST node for a `use` item.
pub mod item_use;
/// Defines the AST nodes for items.
pub mod items;
/// Defines the AST nodes for keywords.
pub mod keyword;
/// Defines the AST nodes for literals.
pub mod literals;
/// Defines the AST nodes for metadata.
pub mod metadata;
/// Defines the AST nodes for patterns.
pub mod patterns;
/// Defines the AST nodes for statements.
pub mod statements;
/// Defines the AST nodes for tokens.
pub mod tokens;
/// Defines the AST nodes for types.
pub mod types;
/// Defines the AST nodes for visibility.
pub mod visibility;
/// Defines the AST nodes for `where` clauses.
pub mod where_clause;

pub use abi::*;
pub use associated_const::*;
pub use associated_type::*;
pub use attributes::*;
pub use comments::*;
pub use expressions::*;
pub use file::*;
pub use generics::*;
pub use item_asm::*;
pub use item_def::*;
pub use item_enum::*;
pub use item_extern_block::*;
pub use item_extern_crate::*;
pub use item_fn::*;
pub use item_foreign_mod::*;
pub use item_impl::*;
pub use item_macro::*;
pub use item_mod::*;
pub use item_static::*;
pub use item_struct::*;
pub use item_trait::*;
pub use item_trait_alias::*;
pub use item_union::*;
pub use item_use::*;
pub use literals::*;
pub use metadata::*;
pub use patterns::*;
pub use statements::*;
pub use tokens::*;
pub use types::*;
pub use visibility::*;
pub use where_clause::*;
