use rasto::ast::*;
use rasto::builder::*;
use rasto::pretty;

#[test]
fn test_infer_expr() {
    let path = path("MyType")
        .generic(GenericArg::Const(expr().infer()))
        .build();

    insta::assert_display_snapshot!(pretty(&path));
}
