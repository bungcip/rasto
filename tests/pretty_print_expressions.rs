use insta::assert_snapshot;
use rasto::{
    ast::{
        builder::{expr, fn_def},
        *,
    },
    pretty_print::{Formatter, PrettyPrint},
};

fn pretty_print(item: impl PrettyPrint) -> String {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);
    item.pretty_print(&mut fmt).unwrap();
    buf
}

#[test]
fn pretty_print_expressions() {
    let expressions = vec![
        (
            "array",
            expr().array(vec![expr().lit(1), expr().lit(2), expr().lit(3)]),
        ),
        (
            "async_block",
            expr().async_block(Block {
                leading_comments: vec![],
                stmts: vec![],
                trailing_comments: vec![],
            }),
        ),
        ("await", expr().await_expr(expr().lit("future"))),
        ("break", expr().break_expr()),
        (
            "call",
            expr().call(
                expr().lit("foo"),
                vec![expr().lit(1), expr().lit("bar")],
            ),
        ),
        ("cast", expr().cast(expr().lit("x"), "u32")),
        (
            "closure",
            expr().closure(vec!["a", "b"], expr().binary(expr().lit("a"), BinOp::Add, expr().lit("b"))),
        ),
        (
            "const_block",
            expr().const_block(Block {
                leading_comments: vec![],
                stmts: vec![],
                trailing_comments: vec![],
            }),
        ),
        ("continue", expr().continue_expr()),
        ("field", expr().field(expr().lit("stru"), "field")),
        ("index", expr().index(expr().lit("arr"), expr().lit("i"))),
        (
            "match",
            expr().match_expr(
                expr().lit("x"),
                vec![
                    Arm {
                        pat: "Some(y)".to_string(),
                        guard: None,
                        body: Box::new(expr().lit("y")),
                    },
                    Arm {
                        pat: "None".to_string(),
                        guard: None,
                        body: Box::new(expr().lit(0)),
                    },
                ],
            ),
        ),
        (
            "method_call",
            expr().method_call(
                expr().lit("obj"),
                "method",
                vec![expr().lit(1), expr().lit("bar")],
            ),
        ),
        ("paren", expr().paren(expr().binary(expr().lit(1), BinOp::Add, expr().lit(2)))),
        (
            "range",
            expr().range(Some(expr().lit(1)), RangeLimits::HalfOpen, Some(expr().lit(10))),
        ),
        ("reference", expr().reference(false, expr().lit("x"))),
        ("return", expr().return_expr(Some(expr().lit(1)))),
        (
            "struct",
            expr().struct_expr(
                "Foo",
                vec![
                    FieldValue {
                        member: "a".to_string(),
                        value: expr().lit(1),
                    },
                    FieldValue {
                        member: "b".to_string(),
                        value: expr().lit("bar"),
                    },
                ],
            ),
        ),
        (
            "tuple",
            expr().tuple(vec![expr().lit(1), expr().lit("bar"), expr().lit(true)]),
        ),
    ];

    for (name, expression) in expressions {
        let item = fn_def("test_fn");
        let block = Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(expression)],
            trailing_comments: vec![],
        };
        let item = item.block(block).build();
        let s = pretty_print(item);
        assert_snapshot!(name, s);
    }
}
