use rasto::ast::{BinOp, Delimiter, RangeLimits, Spacing, TokenStream, UnOp};
use rasto::builder::*;
use rasto::pretty;
use thin_vec::thin_vec;

#[test]
fn test_assign_expr() {
    let expr = expr().assign("a".into(), "b".into());
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_binary_expr() {
    let expr = expr().binary("a".into(), BinOp::Add, "b".into());
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_block_expr() {
    let expr = expr().block(block().statement(expr().lit(42)));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_array_expr() {
    let expr = expr().array(vec![expr().lit(1), expr().lit(2), expr().lit(3)]);
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_async_expr() {
    let expr = expr().async_block(block().statement(expr().lit(42)));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_await_expr() {
    let expr = expr().await_expr("future".into());
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_break_expr() {
    let expr = expr().break_expr();
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_call_expr() {
    let expr = expr().call("my_func".into(), vec![expr().lit(1), "b".into()]);
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_paren_expr() {
    let expr = expr().paren(expr().binary(expr().lit(1), BinOp::Add, expr().lit(2)));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_range_expr() {
    let expr = expr().range(
        Some(expr().lit(0)),
        RangeLimits::Closed,
        Some(expr().lit(10)),
    );
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_ref_expr() {
    let expr = expr().reference(true, "x".into());
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_raw_ref_expr() {
    let expr = expr().raw_ref("x".into()).mutable().build();
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_return_expr() {
    let expr = expr().return_expr(Some(expr().lit(42)));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_struct_expr() {
    let expr = expr().struct_expr("MyStruct", vec![field_value("my_field", expr().lit(42))]);
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_try_expr() {
    let expr = expr().try_block(block().statement(expr().lit("may_fail?")));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_tuple_expr() {
    let expr = expr().tuple(vec![expr().lit(1), "a".into()]);
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_unary_expr() {
    let expr = expr().unary(UnOp::Neg, expr().lit(1));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_while_expr() {
    let expr = expr().while_loop(
        "cond".into(),
        block().statement(expr().call("do_something".into(), vec![])),
    );
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_cast_expr() {
    let expr = expr().cast("x".into(), "u32");
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_closure_expr() {
    let expr = expr().closure(
        vec![pat().ident("a"), pat().ident("b")],
        expr().binary("a".into(), BinOp::Add, "b".into()),
    );
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_const_block_expr() {
    let expr = expr().const_block(block().statement(expr().lit(42)));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_continue_expr() {
    let expr = expr().continue_expr();
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_field_expr() {
    let expr = expr().field("my_struct".into(), "my_field");
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_for_expr() {
    let expr = expr().for_loop(
        pat().ident("i"),
        expr().range(Some(expr().lit(0)), RangeLimits::HalfOpen, None),
        block().statement(expr().call("do_something".into(), vec!["i".into()])),
    );
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_if_expr() {
    let expr = expr().if_expr(
        "cond".into(),
        block().statement(expr().lit(1)),
        Some(expr().block(block().statement(expr().lit(2)))),
    );
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_index_expr() {
    let expr = expr().index("my_array".into(), expr().lit(0));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_loop_expr() {
    let expr = expr().loop_expr(block().statement(expr().break_expr()));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_macro_call_expr() {
    let tokens = TokenStream {
        tokens: thin_vec![
            tt().ident("arg1"),
            tt().punct(',', Spacing::Alone),
            tt().ident("arg2"),
        ],
    };
    let expr = expr().macro_call("my_macro", Delimiter::Parenthesis, tokens);
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_match_expr() {
    let expr = expr().match_expr(
        "x".into(),
        vec![expr().arm(pat().lit(1)).body(expr().lit("one")).build()],
    );
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_method_call_expr() {
    let expr = expr().method_call("my_obj".into(), "my_method", vec![expr().lit(1)]);
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_binary_expr_parentheses() {
    let inner_expr = expr().binary(expr().path("a"), BinOp::Add, expr().path("b"));
    let outer_expr = expr().binary(inner_expr, BinOp::Mul, expr().path("c"));
    insta::assert_snapshot!(pretty(&outer_expr));
}

#[test]
fn test_gen_expr() {
    let expr = expr().gen_block(block().statement(expr().lit(42)));
    insta::assert_snapshot!(pretty(&expr));
}

#[test]
fn test_nested_assign_expr() {
    let inner_expr = expr().assign("b".into(), "c".into());
    let outer_expr = expr().assign("a".into(), inner_expr);
    insta::assert_snapshot!(pretty(&outer_expr));
}
