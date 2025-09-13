//! The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes.
//!
//! This module defines the data structures that represent the syntactic structure of Rust code.
//! It includes nodes for expressions, statements, items, literals, and more. The AST is used
//! by the pretty-printer to generate formatted Rust code.

/// Defines the AST node for an associated type in a trait.
pub mod associated_type;
pub mod attributes;
pub mod comments;
pub mod expressions;
pub mod file;
pub mod generics;
/// Defines the AST node for a `const` item.
pub mod item_const;
/// Defines the AST node for an `extern crate` item.
pub mod item_extern_crate;
/// Defines the AST node for a foreign module.
pub mod item_foreign_mod;
/// Defines the AST node for a macro definition.
pub mod item_macro;
/// Defines the AST node for a module.
pub mod item_mod;
/// Defines the AST node for a `static` item.
pub mod item_static;
/// Defines the AST node for a trait.
pub mod item_trait;
/// Defines the AST node for a trait alias.
pub mod item_trait_alias;
/// Defines the AST node for a type alias.
pub mod item_type;
/// Defines the AST node for a `union` item.
pub mod item_union;
/// Defines the AST node for a `use` item.
pub mod item_use;
pub mod items;
pub mod literals;
pub mod metadata;
pub mod statements;
pub mod tokens;

pub use associated_type::*;
pub use attributes::*;
pub use comments::*;
pub use expressions::*;
pub use file::*;
pub use item_const::*;
pub use item_extern_crate::*;
pub use item_foreign_mod::*;
pub use item_macro::*;
pub use item_mod::*;
pub use item_static::*;
pub use item_trait::*;
pub use item_trait_alias::*;
pub use item_type::*;
pub use item_union::*;
pub use item_use::*;
pub use items::*;
pub use literals::*;
pub use metadata::*;
pub use statements::*;

pub mod types;
pub use types::*;

pub mod patterns;
pub use patterns::*;

pub mod builder;
pub use builder::*;
pub use generics::*;
pub use tokens::*;
