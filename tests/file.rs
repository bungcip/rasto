//! Tests for the `file` module.

use rasto::builder::*;
use rasto::ast::*;

#[test]
fn test_file_with_items_and_metadata() {
    let file = file()
        .comment(comment().line(" This is a file-level comment."))
        .attr(
            attr()
                .inner()
                .meta(meta().path("allow_internal_unstable"))
        )
        .item(
            fn_def("main")
                .block(
                    block()
                        .statement(
                            expr().call(
                                expr().path("println!"),
                                vec![expr().lit("Hello, world!")],
                            ),
                        ),
                )
                .build(),
        )
        .build();

    let expected_output = r#"#![allow_internal_unstable]

// This is a file-level comment.
fn main() {
    println!("Hello, world!");
}"#;
    assert_eq!(file.to_string(), expected_output);
}

#[test]
fn test_empty_file() {
    let file = file().build();
    assert_eq!(file.to_string(), "");
}

#[test]
fn test_file_with_multiple_items() {
    let file = file()
        .item(struct_def("MyStruct").field("field1", type_().path("u32")).build())
        .item(
            fn_def("my_function")
                .input_typed("s", type_().path("MyStruct"))
                .block(block().statement(expr().path("s")))
                .build(),
        )
        .build();

    let expected_output = r#"struct MyStruct {
    field1: u32,
}
fn my_function(s: MyStruct) {
    s;
}"#;
    assert_eq!(file.to_string(), expected_output);
}