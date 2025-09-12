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
