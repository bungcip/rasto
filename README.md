# rasto

`rasto` is a Rust crate that provides data structures for representing a Rust Abstract Syntax Tree (AST) and functionality to pretty-print it back into well-formatted Rust code.

This is useful for procedural macros, code generation, or any other task that requires manipulating Rust code programmatically.

## Features

-   **Abstract Syntax Tree (AST)**: A comprehensive set of data structures for representing Rust code.
-   **Pretty-Printer**: A flexible and efficient pretty-printer for generating formatted Rust code from the AST.
-   **Builder API**: A fluent builder API for constructing AST nodes programmatically.

## Setup

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rasto = "0.1.0"
```

## Usage

Here's an example of how to build a simple function AST and pretty-print it:

```rust
use rasto::ast::*;
use rasto::pretty_printer::*;
use thin_vec::thin_vec;

fn main() {
    let ast = Item::from(
        ItemFn {
            md: None,
            sig: Signature {
                ident: "foo".to_string(),
                generics: Default::default(),
                inputs: thin_vec![],
                output: None,
            },
            block: Block {
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(Expr::Lit(Lit::Int(LitInt::new(42))))],
                has_trailing_semicolon: true,
                trailing_comments: thin_vec![],
            },
        }
    );

    println!("{}", ast);
}
```

This will output:

```text
fn foo() {
    42;
}
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

### Testing

This project uses snapshot testing with `insta`. To run the tests, use:

```bash
cargo test
```

If you make a change that affects the output, you can review and update the snapshots with:

```bash
cargo insta review
```

## License

This project is licensed under the terms of the LICENSE file.
