use rasto::ast::{Expr, ExprUnary, Lit, LitInt, PatIdent, Stmt, UnOp};
use rasto::builder::*;

#[test]
fn test_fn_builder() {
    let item_fn = fn_def("foo")
        .input("a")
        .input("b")
        .output("bool")
        .statement(expr().lit("Hello, world!"))
        .build();

    let actual = item_fn.to_string();

    insta::assert_snapshot!(actual, @r#"
    fn foo(a, b) -> bool {
        "Hello, world!";
    }
    "#);
}

#[test]
fn test_fn_builder_with_metadata() {
    let item_fn = fn_def("foo")
        .attr(attr().meta("test"))
        .leading_comment(comment().line(" a leading comment"))
        .trailing_comment(comment().line(" a trailing comment"))
        .input("a")
        .input("b")
        .output("bool")
        .statement(expr().lit("Hello, world!"))
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
    let local_stmt = stmt().local("x").ty("i32").expr(expr().lit(42)).build();

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
    let expr = expr().unary(UnOp::Neg, expr().lit(42));

    assert_eq!(
        expr,
        Expr::Unary(ExprUnary {
            op: UnOp::Neg,
            expr: Box::new(Expr::Lit(Lit::Int(LitInt::new(42)))),
        })
    );
}

#[test]
fn test_comment_builder() {
    let line_comment = comment().line(" a line comment");
    assert_eq!(
        line_comment,
        rasto::ast::Comment::Line(" a line comment".to_string())
    );

    let block_comment = comment().block(" a block comment");
    assert_eq!(
        block_comment,
        rasto::ast::Comment::Block(" a block comment".to_string())
    );

    let doc_comment = comment().doc(" a doc comment");
    assert_eq!(
        doc_comment,
        rasto::ast::Comment::Doc(" a doc comment".to_string())
    );
}
