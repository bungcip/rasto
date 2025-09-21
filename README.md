# Rasto - **R**ust **AST** **O**pinionated Crates

[![Latest Version](https://img.shields.io/crates/v/rasto.svg)](https://crates.io/crates/rasto)
[![Docs.rs](https://docs.rs/rasto/badge.svg)](https://docs.rs/rasto)
[![CI](https://github.com/bungcip/rasto/actions/workflows/ci.yml/badge.svg)](https://github.com/bungcip/rasto/actions/workflows/ci.yml)

`rasto` is a Rust crate for programmatic manipulation of Rust code.

It provides a set of data structures for representing a Rust Abstract Syntax Tree (AST) and a pretty-printer to format the AST back into well-structured Rust code.

## Philosophy

`rasto` is an opinionated library with a specific focus on AST manipulation and code generation. It is designed with the following principles in mind:

-   **Opinionated Comment Placement**: `rasto` enforces a specific style of comment placement. Not all AST nodes can have comments attached to them. This is a deliberate design choice to ensure that the generated code is always well-formatted and readable.

-   **No Parser**: `rasto` does not include a parser. The focus of the library is on AST manipulation and pretty-printing. Parsing is considered a separate concern and is out of the scope of this project. For parsing Rust code, we recommend using other libraries like `syn`.

## Features

-   **Fluent Builder API**: A comprehensive and easy-to-use builder API for constructing Rust AST nodes.
-   **Pretty-Printing**: A high-quality pretty-printer that formats the AST back into well-structured and readable Rust code.
-   **Extensive AST Coverage**: Data structures for a wide range of Rust syntax, including expressions, statements, items, and more.
-   **Snapshot Testing**: Integration with `insta` for robust snapshot testing of the pretty-printer.

## Comparison with other crates

| Crate         | AST Manipulation | Parsing | Pretty Printing | Focus                                            |
|---------------|------------------|---------|-----------------|--------------------------------------------------|
| `rasto`       | Yes              | No      | Yes             | Opinionated AST manipulation and pretty-printing |
| `syn`         | Yes              | Yes     | No              | Parsing Rust code into a syntax tree             |
| `quote`       | No               | No      | Yes             | Turning a syntax tree back into Rust code        |
| `proc-macro2` | No               | No      | No              | A wrapper around the compiler's `proc_macro` API |

## Setup

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rasto = "0.1.0"
```

## Project Structure

-   `src/lib.rs`: The main library file, which exports the `ast` and `pretty_printer` modules.
-   `src/ast.rs`: The root of the `ast` module, which exports all the AST nodes.
-   `src/ast/`: This directory contains the definitions for the AST nodes. Each file in this directory corresponds to a specific part of the AST, such as expressions, statements, or items.
-   `src/builder.rs`: A fluent builder API for constructing AST nodes programmatically.
-   `src/pretty_printer.rs`: This file contains the implementation of the pretty-printer. It uses a token-based approach inspired by Philip Wadler's "A Prettier Printer".
-   `tests/`: Contains the integration tests, which use snapshot testing with `insta`.
-   `Cargo.toml`: The package manifest for Rust's package manager, Cargo.

## Usage

Here's a more detailed example of how to build a function with comments and attributes, and then pretty-print it:

```rust
use rasto::ast::*;
use rasto::builder::*;
use rasto::pretty;

fn main() {
    let ast = fn_def("my_function")
        .vis(Visibility::Public)
        .attr(attr().meta(meta().path("test")))
        .comment(comment().doc(" This is a doc comment for my_function."))
        .generic(generic_param().ty("T"))
        .input_typed("arg", "T")
        .output("T")
        .statement(stmt().local(pat().ident("x")).expr(expr().lit(42)))
        .statement(expr().field(expr().ident("arg"), "field"))
        .build();

    println!("{}", pretty(&ast));
}
```

This will produce the following output:

```rust
/// This is a doc comment for my_function.
#[test]
pub fn my_function<T>(arg: T) -> T {
    let x = 42;
    arg.field;
}
```

Here is another example that demonstrates how to build a struct and an `impl` block for it:

```rust
use rasto::ast::*;
use rasto::builder::*;
use rasto::pretty;

fn main() {
    let file = file()
        .item(
            struct_def("MyStruct")
                .vis(Visibility::Public)
                .field("x", "i32")
                .field("y", "i32")
                .build(),
        )
        .item(
            impl_block("MyStruct")
                .item(
                    fn_def("new")
                        .vis(Visibility::Public)
                        .output("Self")
                        .statement(
                            expr().struct_expr(
                                "Self",
                                vec![
                                    field_value("x", expr().lit(0)),
                                    field_value("y", expr().lit(0)),
                                ],
                            ),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    println!("{}", pretty(&file));
}
```

This will produce the following output:

```rust
pub struct MyStruct {
    x: i32,
    y: i32,
}

impl MyStruct {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}
```

## Examples

You can run the examples using the following commands:

```bash
cargo run --example pretty_print_simple
cargo run --example pretty_print_trait
```

## Documentation

The full documentation for this crate can be found on [docs.rs](https://docs.rs/rasto).

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

### Testing

To run the tests, use `cargo test`. If you make changes that affect the pretty-printer's output, you can update the snapshots with `cargo insta review`.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating, you are expected to uphold this code.

## Changelog

All notable changes to this project will be documented in the `CHANGELOG.md` file.

## License

This project is licensed under the terms of the LICENSE file.
