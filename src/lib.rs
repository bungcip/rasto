#![deny(missing_docs)]

//! `rasto` is a Rust crate that provides data structures for representing a
//! Rust Abstract Syntax Tree (AST) and functionality to pretty-print it back
//! into well-formatted Rust code.
//!
//! This crate is useful for procedural macros, code generation, or any other
//! task that requires manipulating Rust code programmatically.
//!
//! ## Usage
//!
//! Here's an example of how to build a simple function AST and pretty-print it:
//!
//! ```rust
//! use rasto::ast::{*, Lit, LitInt};
//! use rasto::builder::comment;
//! use rasto::pretty_printer::*;
//! use thin_vec::thin_vec;
//!
//! let ast = Item::from(
//!     ItemFn {
//!         md: Some(Box::new(Md {
//!             attrs: thin_vec![],
//!             leading_comments: thin_vec![comment().line(" A simple function.")],
//!             trailing_comments: thin_vec![comment().line(" Trailing comment.")],
//!         })),
//!         sig: Signature {
//!             ident: "foo".to_string(),
//!             generics: Default::default(),
//!             inputs: thin_vec![],
//!             output: None,
//!         },
//!         block: Block {
//!             stmts: thin_vec![Stmt::Expr(Expr::Lit(Lit::Int(LitInt::new(42))))],
//!             has_trailing_semicolon: true,
//!             ..Default::default()
//!         },
//!     }
//! );
//!
//! let mut buf = String::new();
//! let mut printer = Printer::new(&mut buf);
//! ast.pretty_print(&mut printer).unwrap();
//! printer.finish().unwrap();
//!
//! println!("{}", buf);
//! ```
//!
//! This will output:
//!
//! ```rust
//! // A simple function.
//! fn foo() {
//!     /* An inner comment */
//!     42;
//! }
//! // Trailing comment.
//! ```

/// The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes.
pub mod ast;
/// The `pretty_printer` module provides a flexible and efficient way to format Rust code from an
/// Abstract Syntax Tree (AST).
pub mod pretty_printer;

/// Defines the builder API for constructing AST nodes.
pub mod builder;

/// pretty function
pub use pretty_printer::pretty;