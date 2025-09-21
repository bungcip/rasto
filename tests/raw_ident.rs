use rasto::ast::*;
use rasto::builder::*;
use rasto::pretty;

#[test]
fn test_raw_ident_fn() {
    let func = fn_def("r#fn")
        .output(type_().path("bool"))
        .block(block().statement(expr().lit(true)))
        .build();
    insta::assert_snapshot!(func, @r###"
    fn r#fn() -> bool {
        true;
    }
    "###);
}

#[test]
fn test_raw_ident_struct() {
    let a_struct = struct_def("r#struct")
        .field("r#field", type_().path("u32"))
        .build();
    insta::assert_snapshot!(a_struct, @r###"
    struct r#struct {
        r#field: u32,
    }
    "###);
}

#[test]
fn test_raw_ident_variable() {
    let let_stmt = stmt()
        .local(pat().ident("r#let"))
        .expr(expr().lit(1))
        .build();
    insta::assert_snapshot!(pretty(&let_stmt), @"let r#let = 1;
");
}

#[test]
fn test_raw_ident_match_arm() {
    let match_expr = expr().match_expr(
        expr().path("x"),
        vec![
            expr()
                .arm(pat().ident("r#match"))
                .body(expr().lit(1))
                .build(),
        ],
    );
    insta::assert_snapshot!(pretty(&match_expr), @r###"
    match x {
        r#match => 1,
    }
    "###);
}
