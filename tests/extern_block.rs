use insta::assert_snapshot;
use rasto::ast::*;
use rasto::builder::*;
use thin_vec::thin_vec;

#[test]
fn test_extern_block() {
    let extern_block = extern_block_item()
        .unsafe_()
        .abi("C")
        .item(ExternalItem::Static(
            "FOO".into(),
            type_().path(path("i32").build()).into(),
        ))
        .item(ExternalItem::Fn(fn_def("foo").build()))
        .build();

    assert_snapshot!(extern_block.to_string(), @r###"
    unsafe extern "C" {
        static FOO: i32;
        fn foo();
    }
    "###);
}

#[test]
fn test_extern_block_with_macro() {
    let extern_block = extern_block_item()
        .abi("system")
        .item(ExternalItem::Macro(
            macro_item(expr().macro_call(
                "my_macro",
                Delimiter::Parenthesis,
                TokenStream {
                    tokens: thin_vec![],
                },
            ))
            .build(),
        ))
        .build();

    assert_snapshot!(extern_block.to_string(), @r###"
    extern "system" {
        my_macro!();
    }
    "###);
}
