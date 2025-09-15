# rasto

`rasto` is a Rust crate that provides a comprehensive set of data structures for representing a Rust Abstract Syntax Tree (AST) and a powerful pretty-printer to format the AST back into well-structured Rust code.

This library is designed for tasks such as procedural macros, code generation, source code analysis, and any other application that requires programmatic manipulation of Rust code.

## Features

-   **Comprehensive AST**: A rich set of data structures for representing all aspects of Rust's syntax, including expressions, statements, items, types, and attributes.
-   **Full Comment Support**: Easily attach leading, trailing, and inner comments to any item or statement, preserving the full context of the source code.
-   **Fluent Builder API**: A powerful and intuitive builder pattern for programmatically constructing complex AST nodes with ease.
-   **High-Quality Pretty-Printer**: A flexible and efficient pretty-printer based on Philip Wadler's "A Prettier Printer" that generates beautifully formatted Rust code from the AST.
-   **Snapshot Testing**: The project uses `insta` for robust snapshot testing, ensuring the correctness of the pretty-printer's output.

## Setup

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rasto = "0.1.0"
```

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
        .leading_comment(comment().doc(" This is a doc comment for my_function."))
        .generic(generic_param().ty("T"))
        .input(pat().ident("arg"))
        .output("T".into())
        .block(
            block()
                .leading_comment(comment().block(" An inner block comment. "))
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

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on GitHub. Pull requests are also greatly appreciated.

When contributing, please ensure that any new public items are fully documented. The build will fail if any public item is missing documentation.

### Testing

This project uses snapshot testing with `insta`. To run the tests, use:

```bash
cargo test
```

If you make a change that affects the output of the pretty-printer, you can review and update the snapshots with:

```bash
cargo insta review
```

## License

This project is licensed under the terms of the LICENSE file.
