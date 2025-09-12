use rasto::ast::{builder::fn_def, Block, Expr, Lit, Stmt};

#[test]
fn test_fn_builder() {
    let item_fn = fn_def("foo")
        .input("i32")
        .input("String")
        .output("bool")
        .block(Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(Expr::Lit(Lit::Str(
                "Hello, world!".to_string(),
            )))],
            trailing_comments: vec![],
        })
        .build();

    let actual = item_fn.to_string();

    insta::assert_snapshot!(actual);
}

#[test]
fn test_unary_builder() {
    use rasto::ast::builder::expr;
    use rasto::ast::{Expr, ExprUnary, Lit, UnOp};

    let expr = expr().unary(UnOp::Neg, expr().lit(Lit::Int(42)));

    assert_eq!(
        expr,
        Expr::Unary(ExprUnary {
            op: UnOp::Neg,
            expr: Box::new(Expr::Lit(Lit::Int(42))),
        })
    );
}
