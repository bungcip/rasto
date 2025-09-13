//! Tests for patterns.

use rasto::ast::builder::*;
use rasto::ast::*;
use rasto::pretty_printer::{PrettyPrinter, Printer};

fn pretty_print(item: &impl PrettyPrinter) -> String {
    let mut buffer = String::new();
    let mut printer = Printer::new(&mut buffer);
    item.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();
    buffer
}

#[test]
fn test_let_statement_with_ident_pattern() {
    let let_stmt = stmt()
        .local(pat().ident("x", false))
        .expr(expr().lit(Lit::Int(42)))
        .build();

    insta::assert_snapshot!(pretty_print(&let_stmt), @"let x = 42;");
}

#[test]
fn test_let_statement_with_mut_ident_pattern() {
    let let_stmt = stmt()
        .local(pat().ident("x", true))
        .expr(expr().lit(Lit::Int(42)))
        .build();

    insta::assert_snapshot!(pretty_print(&let_stmt), @"let mut x = 42;");
}

#[test]
fn test_for_expression_with_ident_pattern() {
    let for_expr = expr().for_loop(
        pat().ident("x", false),
        expr().lit(Lit::Int(10)),
        Block {
            leading_comments: vec![],
            stmts: vec![],
            trailing_comments: vec![],
        },
    );

    insta::assert_snapshot!(pretty_print(&for_expr), @"for x in 10 {}");
}

#[test]
fn test_match_expression_with_rest_pattern() {
    let match_expr = expr().match_expr(
        expr().lit(Lit::Int(10)),
        vec![Arm {
            pat: pat().rest(),
            guard: None,
            body: Box::new(expr().lit(Lit::Int(42))),
        }],
    );

    insta::assert_snapshot!(pretty_print(&match_expr), @r"
    match 10 {
    .. => 42,
    }
    ");
}
