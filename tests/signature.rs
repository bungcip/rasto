//! Tests for the `Signature` AST node.

use rasto::ast::*;
use rasto::builder::*;
use rasto::pretty;

#[test]
fn test_const_fn() {
    let item = fn_def("my_fn").const_().build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_async_fn() {
    let item = fn_def("my_fn").async_().build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_unsafe_fn() {
    let item = fn_def("my_fn").unsafe_().build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_extern_fn() {
    let item = fn_def("my_fn").abi(Abi::Named("C".into())).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_variadic_fn() {
    let item = fn_def("my_fn").variadic(true).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_typed_input() {
    let item = fn_def("my_fn")
        .input_typed("foo", path("i32").build_type())
        .input_typed("bar", path("String").build_type())
        .build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_where_clause() {
    let mut where_clause = WhereClause::new();
    where_clause
        .predicates
        .push(WherePredicate::Type(TypePredicate {
            ty: path("T").build_type(),
            bounds: vec![path("Trait").build_type()],
        }));
    let item = fn_def("my_fn").where_clause(where_clause).build();
    insta::assert_snapshot!(pretty(&item));
}

#[test]
fn test_all_the_things() {
    let mut where_clause = WhereClause::new();
    where_clause
        .predicates
        .push(WherePredicate::Type(TypePredicate {
            ty: path("T").build_type(),
            bounds: vec![path("Trait").build_type()],
        }));
    let item = fn_def("my_fn")
        .const_()
        .async_()
        .unsafe_()
        .abi(Abi::Named("C".into()))
        .generic(generic_param().ty("T"))
        .input(pat().ident("t"))
        .variadic(true)
        .output(path("T").build_type())
        .where_clause(where_clause)
        .build();
    insta::assert_snapshot!(pretty(&item));
}
