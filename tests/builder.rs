use rasto::ast::{Block, Expr, Lit, Stmt, builder::*};

#[test]
fn test_fn_builder() {
    let item_fn = fn_def("foo")
        .input("i32")
        .input("String")
        .output("bool")
        .block(Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(
                Expr::Lit(Lit::Str("Hello, world!".to_string())),
                true,
            )],
            trailing_comments: vec![],
        })
        .build();

    let actual = item_fn.to_string();

    insta::assert_snapshot!(actual);
}

#[test]
fn test_stmt_builder() {
    let local_stmt = stmt()
        .local(pat().ident("x", false))
        .ty("i32")
        .expr(Expr::Lit(Lit::Int(42)))
        .build();

    assert_eq!(
        local_stmt,
        Stmt::Local(rasto::ast::Local {
            pat: pat().ident("x", false),
            ty: Some("i32".into()),
            expr: Some(Expr::Lit(Lit::Int(42))),
        })
    );

    let expr_stmt = stmt().expr(Expr::Lit(Lit::Int(42)), true);

    assert_eq!(expr_stmt, Stmt::Expr(Expr::Lit(Lit::Int(42)), true));
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
