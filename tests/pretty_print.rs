use rasto::ast::builder::{file, fn_def};
use rasto::ast::*;
use rasto::pretty_printer::{PrettyPrinter, Printer};

fn pretty_print_item(item: Item) -> String {
    let mut buf = String::new();
    let mut printer = Printer::new(&mut buf);
    item.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();
    buf
}

fn pretty_print_file(file: File) -> String {
    let mut buf = String::new();
    let mut printer = Printer::new(&mut buf);
    for (i, item) in file.items.iter().enumerate() {
        if i > 0 {
            printer.hard_break();
            printer.hard_break();
        }
        item.pretty_print(&mut printer).unwrap();
    }
    printer.finish().unwrap();
    buf
}

#[test]
fn test_file() {
    let ast = file()
        .item(Item::Struct(ItemStruct {
            attrs: vec![],
            leading_comments: vec![Comment::Line(" A simple struct.".to_string())],
            ident: "Foo".to_string(),
            fields: vec![
                Field {
                    attrs: vec![],
                    ident: "field1".to_string(),
                    ty: "i32".into(),
                },
                Field {
                    attrs: vec![],
                    ident: "field2".to_string(),
                    ty: "String".into(),
                },
            ],
            trailing_comments: vec![],
        }))
        .item(
            fn_def("foo")
                .input("i32")
                .output("i32")
                .block(Block {
                    leading_comments: vec![],
                    stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(42)))],
                    trailing_comments: vec![],
                })
                .build(),
        )
        .build();

    insta::assert_snapshot!(pretty_print_file(ast));
}

#[test]
fn test_fn() {
    let ast = Item::Fn(
        fn_def("foo")
            .input("i32")
            .output("i32")
            .block(Block {
                leading_comments: vec![Comment::Block(" An inner comment ".to_string())],
                stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(42)))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

fn pretty_print_expr(expr: Expr) -> String {
    let mut buf = String::new();
    let mut printer = Printer::new(&mut buf);
    expr.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();
    buf
}

#[test]
fn test_expr_array() {
    let ast = Expr::Array(ExprArray {
        elems: vec![Expr::Lit(Lit::Int(1)), Expr::Lit(Lit::Int(2))],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_async() {
    let ast = Expr::Async(ExprAsync {
        block: Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(1)))],
            trailing_comments: vec![],
        },
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_await() {
    let ast = Expr::Await(ExprAwait {
        expr: Box::new(Expr::Lit(Lit::Str("future".to_string()))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_break() {
    let ast = Expr::Break(ExprBreak);
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_call() {
    let ast = Expr::Call(ExprCall {
        func: Box::new(Expr::Lit(Lit::Str("foo".to_string()))),
        args: vec![Expr::Lit(Lit::Int(1)), Expr::Lit(Lit::Int(2))],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_cast() {
    let ast = Expr::Cast(ExprCast {
        expr: Box::new(Expr::Lit(Lit::Str("x".to_string()))),
        ty: "u32".into(),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_closure() {
    let ast = Expr::Closure(ExprClosure {
        inputs: vec!["a".to_string(), "b".to_string()],
        body: Box::new(Expr::Lit(Lit::Int(1))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_const() {
    let ast = Expr::Const(ExprConst {
        block: Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(1)))],
            trailing_comments: vec![],
        },
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_continue() {
    let ast = Expr::Continue(ExprContinue);
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_field() {
    let ast = Expr::Field(ExprField {
        expr: Box::new(Expr::Lit(Lit::Str("stru".to_string()))),
        member: "field".to_string(),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_index() {
    let ast = Expr::Index(ExprIndex {
        expr: Box::new(Expr::Lit(Lit::Str("arr".to_string()))),
        index: Box::new(Expr::Lit(Lit::Int(0))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_match() {
    let ast = Expr::Match(ExprMatch {
        expr: Box::new(Expr::Lit(Lit::Str("x".to_string()))),
        arms: vec![
            Arm {
                pat: "Some(y)".to_string(),
                guard: None,
                body: Box::new(Expr::Lit(Lit::Int(1))),
            },
            Arm {
                pat: "None".to_string(),
                guard: Some(Box::new(Expr::Lit(Lit::Bool(true)))),
                body: Box::new(Expr::Lit(Lit::Int(2))),
            },
        ],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_method_call() {
    let ast = Expr::MethodCall(ExprMethodCall {
        receiver: Box::new(Expr::Lit(Lit::Str("obj".to_string()))),
        method: "method".to_string(),
        args: vec![Expr::Lit(Lit::Int(1)), Expr::Lit(Lit::Int(2))],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_paren() {
    let ast = Expr::Paren(ExprParen {
        expr: Box::new(Expr::Lit(Lit::Int(1))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_range() {
    let ast = Expr::Range(ExprRange {
        start: Some(Box::new(Expr::Lit(Lit::Int(1)))),
        limits: RangeLimits::HalfOpen,
        end: Some(Box::new(Expr::Lit(Lit::Int(5)))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_reference() {
    let ast = Expr::Reference(ExprRef {
        is_mut: true,
        expr: Box::new(Expr::Lit(Lit::Str("x".to_string()))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_return() {
    let ast = Expr::Return(ExprReturn {
        expr: Some(Box::new(Expr::Lit(Lit::Int(1)))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_struct() {
    let ast = Expr::Struct(ExprStruct {
        path: "Foo".to_string(),
        fields: vec![
            FieldValue {
                member: "a".to_string(),
                value: Expr::Lit(Lit::Int(1)),
            },
            FieldValue {
                member: "b".to_string(),
                value: Expr::Lit(Lit::Int(2)),
            },
        ],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_tuple() {
    let ast = Expr::Tuple(ExprTuple {
        elems: vec![Expr::Lit(Lit::Int(1)), Expr::Lit(Lit::Int(2))],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_long_enum() {
    let ast = Item::Enum(ItemEnum {
        attrs: vec![],
        leading_comments: vec![],
        ident: "MyLongLongLongLongLongEnum".to_string(),
        variants: vec![
            Variant {
                attrs: vec![],
                ident: "AVeryLongVariantNameThatShouldCauseALineBreak".to_string(),
            },
            Variant {
                attrs: vec![],
                ident: "AnotherVeryLongVariantNameThatShouldAlsoCauseALineBreak".to_string(),
            },
        ],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_single_field_struct() {
    let ast = Item::Struct(ItemStruct {
        attrs: vec![],
        leading_comments: vec![],
        ident: "MyStruct".to_string(),
        fields: vec![Field {
            attrs: vec![],
            ident: "field".to_string(),
            ty: "i32".into(),
        }],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_nested_struct() {
    let ast = Item::Struct(ItemStruct {
        attrs: vec![],
        leading_comments: vec![],
        ident: "Outer".to_string(),
        fields: vec![
            Field {
                attrs: vec![],
                ident: "inner".to_string(),
                ty: "Inner".into(),
            },
            Field {
                attrs: vec![],
                ident: "another_field".to_string(),
                ty: "i32".into(),
            },
        ],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_long_binary_expression() {
    let ast = Item::Fn(ItemFn {
        attrs: vec![],
        leading_comments: vec![],
        sig: Signature {
            ident: "foo".to_string(),
            inputs: vec![],
            output: None,
        },
        block: Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(Expr::Binary(ExprBinary {
                left: Box::new(Expr::Lit(Lit::Str(
                    "a_very_long_string_that_should_cause_a_line_break".to_string(),
                ))),
                op: BinOp::Add,
                right: Box::new(Expr::Lit(Lit::Str(
                    "another_very_long_string_that_should_also_cause_a_line_break".to_string(),
                ))),
            }))],
            trailing_comments: vec![],
        },
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_trait() {
    let ast = Item::Trait(ItemTrait {
        attrs: vec![],
        leading_comments: vec![Comment::Line(" A simple trait.".to_string())],
        ident: "MyTrait".to_string(),
        items: vec![TraitItem::Fn(TraitItemFn {
            attrs: vec![],
            leading_comments: vec![],
            sig: Signature {
                ident: "my_func".to_string(),
                inputs: vec![],
                output: None,
            },
            block: None,
            trailing_comments: vec![],
        })],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_loop_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Expr(Expr::Loop(ExprLoop {
                    body: Block {
                        leading_comments: vec![],
                        stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(1)))],
                        trailing_comments: vec![],
                    },
                }))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_while_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Expr(Expr::While(ExprWhile {
                    cond: Box::new(Expr::Lit(Lit::Int(1))),
                    body: Block {
                        leading_comments: vec![],
                        stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(2)))],
                        trailing_comments: vec![],
                    },
                }))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_for_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Expr(Expr::For(ExprFor {
                    pat: "x".to_string(),
                    expr: Box::new(Expr::Lit(Lit::Int(1))),
                    body: Block {
                        leading_comments: vec![],
                        stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(2)))],
                        trailing_comments: vec![],
                    },
                }))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_assign_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Expr(Expr::Assign(ExprAssign {
                    left: Box::new(Expr::Lit(Lit::Str("x".to_string()))),
                    right: Box::new(Expr::Lit(Lit::Int(1))),
                }))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_macro_call_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Expr(Expr::MacroCall(ExprMacroCall {
                    ident: "println".to_string(),
                    tokens: TokenStream {
                        tokens: vec![TokenTree::Group(Group {
                            delimiter: Delimiter::Parenthesis,
                            stream: TokenStream {
                                tokens: vec![TokenTree::Literal(Lit::Str("hello".to_string()))],
                            },
                        })],
                    },
                }))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_enum() {
    let ast = Item::Enum(ItemEnum {
        attrs: vec![],
        leading_comments: vec![Comment::Line(" A simple enum.".to_string())],
        ident: "MyEnum".to_string(),
        variants: vec![
            Variant {
                attrs: vec![],
                ident: "Variant1".to_string(),
            },
            Variant {
                attrs: vec![],
                ident: "Variant2".to_string(),
            },
        ],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_impl() {
    let ast = Item::Impl(ItemImpl {
        attrs: vec![],
        leading_comments: vec![Comment::Line(" A simple impl.".to_string())],
        ty: "MyStruct".into(),
        fns: vec![fn_def("new")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![],
                trailing_comments: vec![],
            })
            .build()],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_let_statement() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Let(StmtLet {
                    ident: "x".to_string(),
                    ty: Some("i32".into()),
                    expr: Some(Expr::Lit(Lit::Int(42))),
                })],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_if_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Expr(Expr::If(ExprIf {
                    cond: Box::new(Expr::Lit(Lit::Int(1))),
                    then_branch: Block {
                        leading_comments: vec![],
                        stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(2)))],
                        trailing_comments: vec![],
                    },
                    else_branch: Some(Box::new(Expr::If(ExprIf {
                        cond: Box::new(Expr::Lit(Lit::Int(3))),
                        then_branch: Block {
                            leading_comments: vec![],
                            stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(4)))],
                            trailing_comments: vec![],
                        },
                        else_branch: Some(Box::new(Expr::Block(ExprBlock {
                            block: Block {
                                leading_comments: vec![],
                                stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(5)))],
                                trailing_comments: vec![],
                            },
                        }))),
                    }))),
                }))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_binary_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: vec![],
                stmts: vec![Stmt::Expr(Expr::Binary(ExprBinary {
                    left: Box::new(Expr::Lit(Lit::Int(1))),
                    op: BinOp::Add,
                    right: Box::new(Expr::Lit(Lit::Int(2))),
                }))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_struct() {
    let ast = Item::Struct(ItemStruct {
        attrs: vec![],
        leading_comments: vec![Comment::Line(" A simple struct.".to_string())],
        ident: "Foo".to_string(),
        fields: vec![
            Field {
                attrs: vec![],
                ident: "field1".to_string(),
                ty: "i32".into(),
            },
            Field {
                attrs: vec![],
                ident: "field2".to_string(),
                ty: "String".into(),
            },
        ],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}
