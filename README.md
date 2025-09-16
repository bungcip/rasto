# `rasto`

`rasto` is a Rust crate for programmatic manipulation of Rust code. It provides a set of data structures for representing a Rust Abstract Syntax Tree (AST) and a pretty-printer to format the AST back into well-structured Rust code.

## Philosophy

`rasto` is an opinionated library with a specific focus on AST manipulation and code generation. It is designed with the following principles in mind:

-   **Ergonomic Builder API**: `rasto` provides a fluent builder API for constructing AST nodes programmatically. This allows for a more readable and maintainable way of creating complex AST structures.
-   **Opinionated Comment Placement**: `rasto` enforces a specific style of comment placement. Not all AST nodes can have comments attached to them. This is a deliberate design choice to ensure that the generated code is always well-formatted and readable.
-   **No Parser**: `rasto` does not include a parser. The focus of the library is on AST manipulation and pretty-printing. Parsing is considered a separate concern and is out of the scope of this project. For parsing Rust code, we recommend using other libraries like `syn`.

## Features

-   **Comprehensive AST**: `rasto` provides a comprehensive set of data structures for representing a Rust AST, including expressions, statements, items, patterns, types, and more.
-   **Fluent Builder API**: A rich builder API for constructing AST nodes programmatically.
-   **Pretty-Printer**: A flexible and efficient pretty-printer for formatting the AST back into well-formatted Rust code. The pretty-printer is based on Philip Wadler's "A Prettier Printer".
-   **Extensive Documentation**: All public items are fully documented with examples.

## Comparison with other crates

| Crate | AST Manipulation | Parsing | Pretty Printing | Focus |
| --- | --- | --- | --- | --- |
| `rasto` | Yes | No | Yes | Opinionated AST manipulation and pretty-printing |
| `syn` | Yes | Yes | No | Parsing Rust code into a syntax tree |
| `quote` | No | No | Yes | Turning a syntax tree back into Rust code |
| `proc-macro2` | No | No | No | A wrapper around the compiler's `proc_macro` API |

## Setup

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rasto = "0.1.0"
```

## Usage

### Simple Example

Here's a simple example of how to build a function and pretty-print it:

```rust
use rasto::builder::*;
use rasto::pretty;

fn main() {
    let ast = fn_def("my_function")
        .vis(Visibility::Public)
        .input(pat().ident("arg"))
        .output("T".into())
        .statement(expr().lit(42))
        .build();

    println!("{}", pretty(&ast));
}
```

This will produce the following output:

```rust
pub fn my_function(arg) -> T {
    42;
}
```

### Detailed Example

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
        .input(pat().ident("arg"))
        .output("T".into())
        .block(
            block()
                .comment(comment().block(" An inner block comment. "))
                .statement(
                    stmt()
                        .local(pat().ident("x"))
                        .expr(expr().lit(42))
                        .build(),
                )
                .statement(expr().field(expr().ident("arg"), "field"))
        )
        .trailing_comment(comment().line(" A trailing line comment."))
        .build();

    println!("{}", pretty(&ast));
}
```

This will produce the following output:

```rust
/// This is a doc comment for my_function.
#[test]
pub fn my_function<T>(arg) -> T {
    /* An inner block comment. */
    let x = 42;
    arg.field;
}
// A trailing line comment.
```

## Documentation

The documentation for this crate can be generated and viewed locally using `cargo doc`:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

### Testing

To run the tests, use `cargo test`.

## License

This project is licensed under the terms of the LICENSE file.
