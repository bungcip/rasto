use rasto::ast::{
    builder::{
        const_item, extern_crate_item, fn_def, foreign_mod_item, macro_item, mod_item,
        static_item, trait_alias_item, type_item, union_item, use_item,
    },
    expr, Block, Field, Item, TokenStream, Type,
};
use thin_vec::thin_vec;

#[test]
fn test_const_item() {
    let item = const_item("MAX", Type::from("u16"), expr().lit(234342)).build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_extern_crate_item() {
    let item = extern_crate_item("serde").build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_foreign_mod_item() {
    let item = foreign_mod_item("C")
        .item(Item::Fn(
            fn_def("foo")
                .block(Block {
                    leading_comments: thin_vec![],
                    stmts: thin_vec![],
                    trailing_comments: thin_vec![],
                })
                .build(),
        ))
        .build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_macro_item() {
    let item =
        macro_item(expr().macro_call("my_macro", TokenStream { tokens: thin_vec![] })).build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_mod_item() {
    let item = mod_item("my_module").build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_mod_item_with_content() {
    let item = mod_item("my_module")
        .content(thin_vec![Item::Fn(
            fn_def("foo")
                .block(Block {
                    leading_comments: thin_vec![],
                    stmts: thin_vec![],
                    trailing_comments: thin_vec![],
                })
                .build(),
        )])
        .build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_static_item() {
    let item = static_item("COUNTER", Type::from("u32"), expr().lit(0)).build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_trait_alias_item() {
    let item = trait_alias_item(
        "ShareableIterator",
        thin_vec!["Iterator".to_string(), "Sync".to_string()],
    )
    .build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_type_item() {
    let item = type_item("MyResult<T>", Type::from("Result<T, MyError>")).build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_union_item() {
    let item = union_item("MyUnion")
        .field(Field {
            ident: "f1".to_string(),
            ty: Type::from("u32"),
            md: None,
        })
        .field(Field {
            ident: "f2".to_string(),
            ty: Type::from("f32"),
            md: None,
        })
        .build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_use_item() {
    let item = use_item("std::collections::HashMap").build();
    insta::assert_snapshot!(item.to_string());
}
