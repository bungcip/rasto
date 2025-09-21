mod associated_const;

use rasto::ast::{AsmDirection, AsmOption, LitStr, RegSpec};
use rasto::builder::*;
use rasto::pretty;
use thin_vec::thin_vec;

#[test]
fn test_const_item() {
    let item = const_def("MAX", "u16", expr().lit(234342)).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_type_alias_item() {
    let item = type_alias("MyResult<T>", "Result<T, MyError>").build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_extern_crate_item() {
    let item = extern_crate_item("serde").build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_foreign_mod_item() {
    let item = foreign_mod_item("C").item(fn_def("foo")).build();
    insta::assert_snapshot!(pretty(&item));
}

use rasto::ast::Delimiter;

#[test]
fn test_macro_item() {
    let item =
        macro_item(expr().macro_call("my_macro", Delimiter::Parenthesis, thin_vec![])).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_empty_mod_item() {
    let item = empty_mod_item("empty_module").build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_mod_item() {
    let item = mod_item("my_module").build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_mod_item_with_content() {
    let item = mod_item("my_module").item(fn_def("foo")).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_trait_alias_item() {
    let item = trait_alias_item(
        "ShareableIterator",
        thin_vec!["Iterator".to_string(), "Sync".to_string()],
    )
    .build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_union_item() {
    let item = union_item("MyUnion")
        .field("f1", "u32")
        .field("f2", "f32")
        .build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_static_item() {
    let item = static_item("FOO", "u32", expr().lit(42)).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_mutable_static_item() {
    let item = static_item("BAR", "u32", expr().lit(0)).mutable().build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_trait_with_function() {
    let item = trait_def("MyTrait").item(trait_item_fn("my_func")).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_use_item() {
    let item = use_item("std::collections::HashMap").build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_trait_with_associated_type() {
    let item = trait_def("MyTrait")
        .associated_type(associated_type("MyType"))
        .build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_impl_item() {
    let item = impl_block("MyType")
        .item(fn_def("my_func").build())
        .item(associated_type("MyType").build())
        .item(
            associated_const("MY_CONST", "u8")
                .expr(expr().lit(5))
                .build(),
        )
        .build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_asm_item() {
    let template = r#""
        mov {tmp}, {x}
        shl {tmp}, 1
        shl {x}, 2
        add {x}, {tmp}
    ""#
    .parse::<LitStr>()
    .unwrap();
    let item = asm_item(template)
        .operand(asm_operand().reg(
            AsmDirection::InOut,
            RegSpec::Class("reg".to_string()),
            expr().lit(42),
        ))
        .operand(asm_operand().reg(
            AsmDirection::Out,
            RegSpec::Class("reg".to_string()),
            expr().lit(0),
        ))
        .options(
            asm_options()
                .option(AsmOption::Nomem)
                .option(AsmOption::AttSyntax)
                .build(),
        )
        .build();
    insta::assert_snapshot!(pretty(&item));
}
