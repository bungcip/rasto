use rasto::ast::{Block, Expr, Lit, LitInt, LitStr, Meta, Stmt, builder::*};
use thin_vec::thin_vec;

#[test]
fn test_fn_builder() {
    let item_fn = fn_def("foo")
        .input(pat().ident("a", false))
        .input(pat().ident("b", false))
        .output("bool")
        .block(Block {
            leading_comments: thin_vec![],
            stmts: thin_vec![Stmt::Expr(
                Expr::Lit(Lit::Str(LitStr::new("Hello, world!"))),
                true,
            )],
            trailing_comments: thin_vec![],
        })
        .build();

    let actual = item_fn.to_string();

    insta::assert_snapshot!(actual);
}

use rasto::ast::{Attribute, Comment, PatIdent};

#[test]
fn test_fn_builder_with_metadata() {
    let item_fn = fn_def("foo")
        .attr(Attribute::Outer(Meta::Path("test".to_string())))
        .leading_comment(Comment::Line(" a leading comment".to_string()))
        .trailing_comment(Comment::Line(" a trailing comment".to_string()))
        .input(pat().ident("a", false))
        .input(pat().ident("b", false))
        .output("bool")
        .block(Block {
            leading_comments: thin_vec![],
            stmts: thin_vec![Stmt::Expr(
                Expr::Lit(Lit::Str(LitStr::new("Hello, world!"))),
                true,
            )],
            trailing_comments: thin_vec![],
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
        .expr(Expr::Lit(Lit::Int(LitInt::new(42))))
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

    let expr_stmt = stmt().expr(Expr::Lit(Lit::Int(LitInt::new(42))), true);

    assert_eq!(
        expr_stmt,
        Stmt::Expr(Expr::Lit(Lit::Int(LitInt::new(42))), true)
    );
}

#[test]
fn test_unary_builder() {
    use rasto::ast::builder::expr;
    use rasto::ast::{Expr, ExprUnary, Lit, UnOp};

    let expr = expr().unary(UnOp::Neg, expr().lit(Lit::Int(LitInt::new(42))));

    assert_eq!(
        expr,
        Expr::Unary(ExprUnary {
            op: UnOp::Neg,
            expr: Box::new(Expr::Lit(Lit::Int(LitInt::new(42)))),
        })
    );
}
