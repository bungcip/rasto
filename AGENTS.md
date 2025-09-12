This is a Rust project that provides data structures for representing a Rust Abstract Syntax Tree (AST) and functionality to pretty-print it back into well-formatted Rust code.

The project follows the standard Rust project layout.

## Project Structure

-   `src/lib.rs`: The main library file, which exports the `ast` and `pretty_printer_v2` modules.
-   `src/ast/`: This directory contains the definitions for the AST nodes. Each file in this directory corresponds to a specific part of the AST, such as expressions, statements, or items.
-   `src/pretty_printer_v2.rs`: This file contains the implementation of the pretty-printer. It uses a token-based approach inspired by Philip Wadler's "A Prettier Printer".
-   `tests/`: Contains the integration tests, which use snapshot testing with `insta`.
-   `Cargo.toml`: The package manifest for Rust's package manager, Cargo.

## Running Tests

The tests use snapshot testing with the `insta` crate. To run the tests, use the following command:

```bash
cargo test
```

If the snapshot tests fail because of intentional changes, you can review and update the snapshots with:

```bash
cargo insta review
```

## Documentation

All public items in the crate are documented with doc comments. The documentation can be generated and viewed locally using `cargo doc --open`.

When making changes, please ensure that all new public items are documented and that the existing documentation is updated as needed. The build will fail if any public items are missing documentation.
