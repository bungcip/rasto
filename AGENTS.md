This is a Rust project that provides data structures for representing a Rust Abstract Syntax Tree (AST) and functionality to pretty-print it back into well-formatted Rust code.

## Project Structure

- `src/lib.rs`: The main library code, defining the AST nodes and the `PrettyPrint` trait.
- `tests/`: Contains the integration tests.
- `Cargo.toml`: The package manifest for Rust's package manager, Cargo.

## Running Tests

The tests use snapshot testing with the `insta` crate. To run the tests, use the following command:

```bash
cargo test
```

If the snapshot tests fail because of intentional changes, you can update the snapshots with:

```bash
cargo insta review
```
