# rasto

[![CI](https://github.com/YOUR_USERNAME/YOUR_REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/YOUR_REPO/actions/workflows/ci.yml)

_Note: Remember to replace `YOUR_USERNAME/YOUR_REPO` with your actual GitHub repository details._

`rasto` is a Rust crate that provides data structures for representing a Rust Abstract Syntax Tree (AST) and functionality to pretty-print it back into well-formatted Rust code.

This is useful for procedural macros, code generation, or any other task that requires manipulating Rust code programmatically.

## Usage

Here's an example of how to build a simple function AST and pretty-print it:

```rust
use rasto::*;

fn main() {
    let ast = Item::Fn(ItemFn {
        leading_comments: vec![Comment::Line(" A simple function.".to_string())],
        sig: Signature {
            ident: "foo".to_string(),
        },
        block: Block {
            leading_comments: vec![Comment::Block(" An inner comment ".to_string())],
            stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(42)))],
            trailing_comments: vec![],
        },
        trailing_comments: vec![Comment::Line(" Trailing comment.".to_string())],
    });

    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);
    ast.pretty_print(&mut fmt).unwrap();

    println!("{}", buf);
}
```

This will output:

```rust
// A simple function.
fn foo() {
    /* An inner comment */
    42;
}
// Trailing comment.
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
