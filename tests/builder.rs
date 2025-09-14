use rasto::ast::{Expr, Lit, LitInt, Stmt, builder::*};

#[test]
fn test_fn_builder() {
    let item_fn = fn_def("foo")
        .input(pat().ident("a", false))
        .input(pat().ident("b", false))
        .output("bool")
        .block(block().statement(expr().lit("Hello, world!")))
        .build();

    let actual = item_fn.to_string();

    insta::assert_snapshot!(actual, @r#"
    fn foo(a, b) -> bool {
        "Hello, world!";
    }
    "#);
}

use rasto::ast::PatIdent;

#[test]
fn test_fn_builder_with_metadata() {
    let item_fn = fn_def("foo")
        .attr(attr().meta("test"))
        .leading_comment(comment().line(" a leading comment"))
        .trailing_comment(comment().line(" a trailing comment"))
        .input(pat().ident("a", false))
        .input(pat().ident("b", false))
        .output("bool")
        .block(
            block()
                .statement(stmt().expr(expr().lit("Hello, world!")))
                
        )
        .build();

    let actual = item_fn.to_string();

    insta::assert_snapshot!(actual, @r#"
    #[test]

    // a leading comment
    fn foo(a, b) -> bool {
        "Hello, world!";
    }
    // a trailing comment
    "#);
}

#[test]
fn test_stmt_builder() {
    let local_stmt = stmt()
        .local(pat().ident("x", false))
        .ty("i32")
        .expr(expr().lit(42))
        .build();

    assert_eq!(
        local_stmt,
        Stmt::Local(rasto::ast::Local {
            pat: rasto::ast::Pat::Ident(PatIdent {
                ident: "x".to_string(),
                is_mut: false
            }),
            ty: Some("i32".into()),
            expr: Some(Expr::Lit(Lit::Int(LitInt::new(42)))),
        })
    );

    let expr_stmt = stmt().expr(expr().lit(42));

    assert_eq!(expr_stmt, Stmt::Expr(Expr::Lit(Lit::Int(LitInt::new(42)))));
}

#[test]
fn test_unary_builder() {
    use rasto::ast::builder::expr;
    use rasto::ast::{Expr, ExprUnary, Lit, UnOp};

    let expr = expr().unary(UnOp::Neg, expr().lit(42));

    assert_eq!(
        expr,
        Expr::Unary(ExprUnary {
            op: UnOp::Neg,
            expr: Box::new(Expr::Lit(Lit::Int(LitInt::new(42)))),
        })
    );
}
