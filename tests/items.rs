use rasto::ast::{
    AssociatedType, Item, TokenStream, Type,
    builder::{
        const_kind, def_item, extern_crate_item, fn_def, foreign_mod_item, macro_item, mod_item,
        static_kind, trait_alias_item, trait_def, type_alias_kind, union_item, use_item,
    },
    expr,
};
use thin_vec::thin_vec;

#[test]
fn test_def_item() {
    let const_item = def_item("MAX", const_kind(Type::from("u16"), expr().lit(234342))).build();
    insta::assert_snapshot!(const_item.to_string());

    let static_item = def_item("COUNTER", static_kind(Type::from("u32"), expr().lit(0))).build();
    insta::assert_snapshot!(static_item.to_string());

    let type_item = def_item(
        "MyResult<T>",
        type_alias_kind(Type::from("Result<T, MyError>")),
    )
    .build();
    insta::assert_snapshot!(type_item.to_string());
}

#[test]
fn test_extern_crate_item() {
    let item = extern_crate_item("serde").build();
    insta::assert_snapshot!(item.to_string());
}

use rasto::ast::builder::block;

#[test]
fn test_foreign_mod_item() {
    let item = foreign_mod_item("C")
        .item(Item::Fn(fn_def("foo").block(block()).build()))
        .build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_macro_item() {
    let item = macro_item(expr().macro_call(
        "my_macro",
        TokenStream {
            tokens: thin_vec![],
        },
    ))
    .build();
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
        .content(thin_vec![Item::Fn(fn_def("foo").block(block()).build())])
        .build();
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
fn test_union_item() {
    let item = union_item("MyUnion")
        .field("f1", Type::from("u32"))
        .field("f2", Type::from("f32"))
        .build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_use_item() {
    let item = use_item("std::collections::HashMap").build();
    insta::assert_snapshot!(item.to_string());
}

#[test]
fn test_trait_with_associated_type() {
    let item = trait_def("MyTrait").associated_type(AssociatedType {
        ident: "MyType".to_string(),
        generics: Default::default(),
        bounds: thin_vec![],
        default: None,
        md: None,
    });
    insta::assert_snapshot!(item.build().to_string());
}
