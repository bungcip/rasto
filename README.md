# Rasto - A Rust AST Generation and Pretty-Printing Crate

[![Latest Version](https://img.shields.io/crates/v/rasto.svg)](https://crates.io/crates/rasto)
[![Docs.rs](https://docs.rs/rasto/badge.svg)](https://docs.rs/rasto)
[![CI](https://github.com/bungcip/rasto/actions/workflows/ci.yml/badge.svg)](https://github.com/bungcip/rasto/actions/workflows/ci.yml)

`rasto` is a Rust crate for programmatic manipulation of Rust code. It provides a comprehensive set of data structures for representing a Rust Abstract Syntax Tree (AST) and a high-quality pretty-printer to format the AST back into well-structured, readable Rust code.

## Philosophy

`rasto` is an opinionated library designed with a clear focus on AST manipulation and code generation. Its core principles are:

-   **Opinionated Comment Placement**: `rasto` enforces a specific style for comment placement. Not all AST nodes can have comments attached, a deliberate design choice to ensure that the generated code is always well-formatted and readable.
-   **No Parser**: `rasto` does not include a parser. The library's focus is on AST manipulation and pretty-printing. For parsing Rust code, we recommend using other excellent libraries like `syn`.

## Features

-   **Fluent Builder API**: A comprehensive and easy-to-use builder API for constructing Rust AST nodes.
-   **High-Quality Pretty-Printer**: A pretty-printer that formats the AST back into well-structured and readable Rust code, inspired by Philip Wadler's "A Prettier Printer".
-   **Extensive AST Coverage**: Data structures for a wide range of Rust syntax, including expressions, statements, items, and more.
-   **Snapshot Testing**: Robust integration with `insta` for snapshot testing of the pretty-printer's output.

## Comparison with Other Crates

| Crate         | AST Manipulation | Parsing | Pretty Printing | Focus                                            |
|---------------|------------------|---------|-----------------|--------------------------------------------------|
| `rasto`       | Yes              | No      | Yes             | Opinionated AST manipulation and pretty-printing |
| `syn`         | Yes              | Yes     | No              | Parsing Rust code into a syntax tree             |
| `quote`       | No               | No      | Yes             | Turning a syntax tree back into Rust code        |
| `proc-macro2` | No               | No      | No              | A wrapper around the compiler's `proc_macro` API |

## Getting Started

To start using `rasto`, add it to your `Cargo.toml` file:

```toml
[dependencies]
rasto = "0.1.0"
```

### Usage

`rasto` provides a fluent builder API for constructing AST nodes. Here’s an example of how to build a function with comments and attributes, and then pretty-print it:

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

## Project Structure

The project follows a standard Rust library layout:

-   `src/lib.rs`: The main library file, which exports the public modules.
-   `src/ast.rs`: The root of the `ast` module, which exports all the AST node types.
-   `src/ast/`: This directory contains the definitions for the AST nodes. Each file corresponds to a specific part of the Rust language's syntax, such as expressions (`expressions.rs`), statements (`statements.rs`), or items (`item_*.rs` files).
-   `src/builder.rs`: Implements the fluent builder API for programmatically constructing AST nodes.
-   `src/pretty_printer.rs`: Contains the implementation of the pretty-printer.
-   `tests/`: Contains integration tests that use snapshot testing with `insta`.
-   `examples/`: Contains runnable examples that demonstrate the crate's usage.

## Documentation

This crate is thoroughly documented, and the build will fail if any public item is missing documentation. You can generate and view the documentation locally by running:

```bash
cargo doc --open
```

The full documentation is also available on [docs.rs](https://docs.rs/rasto).

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on GitHub. Pull requests are also encouraged.

### Development Workflow

To contribute to the project, please follow these steps:

1.  Fork the repository and create a new branch for your changes.
2.  Make your changes, ensuring that all new public items are fully documented.
3.  Run the tests to ensure that your changes do not break existing functionality:
    ```bash
    cargo test
    ```
4.  If your changes affect the output of the pretty-printer, you will need to review and update the snapshots:
    ```bash
    cargo insta review
    ```
5.  Submit a pull request with a clear description of your changes.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating, you are expected to uphold this code.

## Changelog

All notable changes to this project will be documented in the `CHANGELOG.md` file.

## License

This project is licensed under the terms of the LICENSE file.