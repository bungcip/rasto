#![deny(missing_docs)]

//! `rasto` is a Rust crate for programmatic manipulation of Rust code.
//!
//! It provides a set of data structures for representing a Rust Abstract Syntax Tree (AST)
//! and a pretty-printer to format the AST back into well-structured Rust code.
//!
//! This crate is designed to be a flexible and intuitive tool for developers
//! who need to generate or modify Rust code as part of their build process,
//! in procedural macros, or for other code generation tasks.
//!
//! ## Core Features
//!
//! - **Fluent Builder API**: A comprehensive and easy-to-use builder API for constructing
//!   Rust AST nodes.
//! - **High-Quality Pretty-Printer**: A pretty-printer that formats the AST back into
//!   well-structured and readable Rust code.
//! - **Extensive AST Coverage**: Data structures for a wide range of Rust syntax,
//!   including expressions, statements, items, and more.
//!
//! ## Usage
//!
//! Here's an example of how to build a simple function AST and pretty-print it:
//!
//! ```rust
//! use rasto::builder::*;
//! use rasto::pretty;
//!
//! let ast = fn_def("foo")
//!     .comment(comment().doc(" A simple function."))
//!     .statement(expr().lit(42))
//!     .build();
//!
//! let code = pretty(&ast);
//!
//! assert_eq!(code.trim(), r#"
//! /// A simple function.
//! fn foo() {
//!     42;
//! }
//! "#.trim());
//!
//! println!("{}", code);
//! ```
//!
//! This will output:
//!
//! ```rust
//! /// A simple function.
//! fn foo() {
//!     42;
//! }
//! ```

/// The `ast` module contains the definitions for the Abstract Syntax Tree (AST) nodes.
///
/// This module provides a comprehensive set of data structures for representing
/// various components of Rust code, such as expressions, statements, items, and types.
/// Each structure in this module corresponds to a specific element of the Rust language's syntax.
pub mod ast;

/// The `pretty_printer` module provides a flexible and efficient way to format Rust code
/// from an Abstract Syntax Tree (AST).
///
/// It includes the `PrettyPrinter` trait, which defines the core interface for pretty-printing,
/// and a concrete implementation, `Printer`, that generates well-formatted Rust code.
pub mod pretty_printer;

/// The `builder` module defines a fluent API for constructing AST nodes.
///
/// This API is designed to be intuitive and easy to use, allowing you to build up
/// complex AST structures with minimal boilerplate code.
pub mod builder;

/// Re-exports the main pretty-printing utilities for convenient access.
pub use pretty_printer::{pretty, PrettyPrinter, Printer};
