//! Tests for patterns.

use rasto::ast::*;
use rasto::builder::*;
use rasto::pretty;
use thin_vec::thin_vec;

#[test]
fn test_let_statement_with_ident_pattern() {
    let let_stmt = stmt()
        .local(pat().ident("x", false))
        .expr(expr().lit(42))
        .build();

    insta::assert_snapshot!(pretty(&let_stmt), @"let x = 42;");
}

#[test]
fn test_let_statement_with_mut_ident_pattern() {
    let let_stmt = stmt()
        .local(pat().ident("x", true))
        .expr(expr().lit(42))
        .build();

    insta::assert_snapshot!(pretty(&let_stmt), @"let mut x = 42;");
}

#[test]
fn test_for_expression_with_ident_pattern() {
    let for_expr = expr().for_loop(pat().ident("x", false), expr().lit(10), block().build());

    insta::assert_snapshot!(pretty(&for_expr), @"for x in 10 {}");
}

#[test]
fn test_match_expression_with_rest_pattern() {
    let match_expr = expr().match_expr(
        expr().lit(10),
        thin_vec![Arm {
            pat: pat().rest(),
            guard: None,
            body: Box::new(expr().lit(42)),
        }],
    );

    insta::assert_snapshot!(pretty(&match_expr), @r"
    match 10 {
        .. => 42,
    }
    ");
}

#[test]
fn test_let_statement_with_wildcard_pattern() {
    let let_stmt = stmt().local(pat().wild()).build();

    insta::assert_snapshot!(pretty(&let_stmt), @"let _;");
}

#[test]
fn test_let_statement_with_tuple_pattern() {
    let let_stmt = stmt()
        .local(pat().tuple(thin_vec![pat().ident("x", false), pat().ident("y", false)]))
        .expr(expr().tuple(thin_vec![expr().lit(1), expr().lit(2)]))
        .build();

    insta::assert_snapshot!(pretty(&let_stmt), @"let (x, y) = (1, 2);");
}

#[test]
fn test_function_with_tuple_pattern_in_arg() {
    let fn_def = fn_def("foo")
        .input(pat().tuple(thin_vec![pat().ident("x", false), pat().ident("y", false)]))
        .build();

    insta::assert_snapshot!(pretty(&fn_def), @"fn foo((x, y)) {}");
}
