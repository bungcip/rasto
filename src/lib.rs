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
//! use rasto::ast::*;
//! use rasto::pretty_printer::*;
//!
//! fn main() {
//!     let ast = Item::from(
//!         ItemFn {
//!             attrs: vec![],
//!             leading_comments: vec![Comment::Line(" A simple function.".to_string())],
//!             sig: Signature {
//!                 ident: "foo".to_string(),
//!                 inputs: vec![],
//!                 output: None,
//!             },
//!             block: Block {
//!                 leading_comments: vec![Comment::Block(" An inner comment ".to_string())],
//!                 stmts: vec![Stmt::Expr(Expr::Lit(42.into()), true)],
//!                 trailing_comments: vec![],
//!             },
//!             trailing_comments: vec![Comment::Line(" Trailing comment.".to_string())],
//!         }
//!     );
//!
//!     let mut buf = String::new();
//!     let mut printer = Printer::new(&mut buf);
//!     ast.pretty_print(&mut printer).unwrap();
//!     printer.finish().unwrap();
//!
//!     println!("{}", buf);
//! }
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

pub mod ast;
pub mod pretty_printer;
