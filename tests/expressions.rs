use rasto::ast::BinOp;
use rasto::builder::*;
use rasto::pretty;

#[test]
fn test_assign_expr() {
    let expr = expr().assign(expr().lit("a"), expr().lit("b"));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_binary_expr() {
    let expr = expr().binary(expr().lit("a"), BinOp::Add, expr().lit("b"));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_block_expr() {
    let expr = expr().block(block().statement(expr().lit(42)));
    insta::assert_snapshot!(pretty(&expr));
}
