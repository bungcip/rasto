//! Tests for patterns.

use rasto::ast::{Delimiter, Pat, RangeLimits, TokenStream};
use rasto::builder::*;
use rasto::pretty;

#[test]
fn test_wild_pattern() {
    let pat: Pat = pat().wild();
    insta::assert_snapshot!(pretty(&pat), @"_");
}

#[test]
fn test_ident_pattern() {
    let pat: Pat = pat().ident("my_var");
    insta::assert_snapshot!(pretty(&pat), @"my_var");
}

#[test]
fn test_mut_ident_pattern() {
    let pat: Pat = pat().mutable().ident("my_var");
    insta::assert_snapshot!(pretty(&pat), @"mut my_var");
}

#[test]
fn test_lit_pattern() {
    let pat: Pat = pat().lit(42);
    insta::assert_snapshot!(pretty(&pat), @"42");
}

#[test]
fn test_path_pattern() {
    let pat: Pat = pat().path("my::path");
    insta::assert_snapshot!(pretty(&pat), @"my::path");
}

#[test]
fn test_tuple_pattern() {
    let pat: Pat = pat().tuple([pat().ident("a"), pat().wild()]);
    insta::assert_snapshot!(pretty(&pat), @"(a, _)");
}

use thin_vec::thin_vec;
#[test]
fn test_const_pattern() {
    let pat: Pat = pat().const_(expr().struct_expr("MY_CONST", []));
    insta::assert_snapshot!(pretty(&pat), @"const MY_CONST");
}

#[test]
fn test_macro_pattern() {
    let mac = expr().macro_call(
        "my_macro",
        Delimiter::Parenthesis,
        TokenStream {
            tokens: thin_vec![],
        },
    );
    if let rasto::ast::Expr::MacroCall(mac) = mac {
        let pat: Pat = pat().mac(mac);
        insta::assert_snapshot!(pretty(&pat), @"my_macro!()");
    }
}

#[test]
fn test_or_pattern() {
    let pat: Pat = pat().or([pat().ident("a"), pat().ident("b")]);
    insta::assert_snapshot!(pretty(&pat), @"a | b");
}

#[test]
fn test_paren_pattern() {
    let pat: Pat = pat().paren(pat().ident("a"));
    insta::assert_snapshot!(pretty(&pat), @"(a)");
}

#[test]
fn test_range_pattern() {
    let pat: Pat = pat().range(
        Some(expr().lit(1)),
        RangeLimits::Closed,
        Some(expr().lit(10)),
    );
    insta::assert_snapshot!(pretty(&pat), @"1..=10");
}

#[test]
fn test_half_open_range_pattern() {
    let pat: Pat = pat().range(Some(expr().lit(1)), RangeLimits::HalfOpen, None);
    insta::assert_snapshot!(pretty(&pat), @"1..");
}

#[test]
fn test_reference_pattern() {
    let pat: Pat = pat().reference(pat().ident("a")).build();
    insta::assert_snapshot!(pretty(&pat), @"&a");
}

#[test]
fn test_mut_reference_pattern() {
    let pat: Pat = pat().reference(pat().ident("a")).mutable().build();
    insta::assert_snapshot!(pretty(&pat), @"&mut a");
}

#[test]
fn test_rest_pattern() {
    let pat: Pat = pat().rest();
    insta::assert_snapshot!(pretty(&pat), @"..");
}

#[test]
fn test_slice_pattern() {
    let pat: Pat = pat().slice([pat().ident("a"), pat().rest()]);
    insta::assert_snapshot!(pretty(&pat), @"[a, ..]");
}

#[test]
fn test_struct_pattern() {
    let pat: Pat = pat()
        .struct_("MyStruct")
        .field("field1", pat().ident("a"))
        .rest()
        .build();
    insta::assert_snapshot!(pretty(&pat), @"MyStruct { field1: a, .. }");
}

#[test]
fn test_tuple_struct_pattern() {
    let pat: Pat = pat()
        .tuple_struct("MyStruct")
        .pat(pat().ident("a"))
        .pat(pat().wild())
        .build();
    insta::assert_snapshot!(pretty(&pat), @"MyStruct(a, _)");
}

#[test]
fn test_type_pattern() {
    let pat: Pat = pat().type_(pat().ident("a"), type_().path("MyType"));
    insta::assert_snapshot!(pretty(&pat), @"a: MyType");
}
