use rasto::ast::builder::{file, fn_def, pat, stmt};
use rasto::ast::*;
use rasto::pretty_printer::{PrettyPrinter, Printer};
use thin_vec::thin_vec;

mod patterns;

fn pretty_print_item(item: Item) -> String {
    let mut buf = String::new();
    let mut printer = Printer::new(&mut buf);
    item.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();
    buf
}

fn pretty_print_file(file: File) -> String {
    file.to_string()
}

#[test]
fn test_file() {
    let ast = file()
        .item(Item::Struct(ItemStruct {
            md: Some(Box::new(Md {
                attrs: thin_vec![],
                leading_comments: thin_vec![Comment::Line(" A simple struct.".to_string())],
                trailing_comments: thin_vec![],
            })),
            ident: "Foo".to_string(),
            generics: Default::default(),
            fields: thin_vec![
                Field {
                    md: None,
                    ident: "field1".to_string(),
                    ty: "i32".into(),
                },
                Field {
                    md: None,
                    ident: "field2".to_string(),
                    ty: "String".into(),
                },
            ],
        }))
        .item(
            fn_def("foo")
                .input(pat().ident("a", false))
                .output("i32")
                .block(Block {
                    leading_comments: thin_vec![],
                    stmts: thin_vec![Stmt::Expr(Expr::Lit(42.into()), true)],
                    trailing_comments: thin_vec![],
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
            .input(pat().ident("a", false))
            .output("i32")
            .block(Block {
                leading_comments: thin_vec![Comment::Block(" An inner comment ".to_string())],
                stmts: thin_vec![Stmt::Expr(Expr::Lit(42.into()), true)],
                trailing_comments: thin_vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn unary_expression() {
    let expr = expr().unary(UnOp::Neg, expr().lit(42));

    let mut output = String::new();
    let mut printer = Printer::new(&mut output);
    expr.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();

    insta::assert_snapshot!(output, @"-42");
}

#[test]
fn unary_expression_not() {
    let expr = expr().unary(UnOp::Not, expr().lit(true));

    let mut output = String::new();
    let mut printer = Printer::new(&mut output);
    expr.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();

    insta::assert_snapshot!(output, @"!true");
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
        elems: thin_vec![Expr::Lit(1.into()), Expr::Lit(2.into())],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_async() {
    let ast = Expr::Async(ExprAsync {
        block: Block {
            leading_comments: thin_vec![],
            stmts: thin_vec![Stmt::Expr(Expr::Lit(1.into()), true)],
            trailing_comments: thin_vec![],
        },
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_await() {
    let ast = Expr::Await(ExprAwait {
        expr: Box::new(Expr::Lit("future".into())),
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
        func: Box::new(Expr::Lit("foo".into())),
        args: thin_vec![Expr::Lit(1.into()), Expr::Lit(2.into())],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_cast() {
    let ast = Expr::Cast(ExprCast {
        expr: Box::new(Expr::Lit("x".into())),
        ty: "u32".into(),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_closure() {
    let ast = Expr::Closure(ExprClosure {
        inputs: thin_vec![pat().ident("a", false), pat().ident("b", false)],
        body: Box::new(Expr::Lit(1.into())),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_const() {
    let ast = Expr::Const(ExprConst {
        block: Block {
            leading_comments: thin_vec![],
            stmts: thin_vec![Stmt::Expr(Expr::Lit(1.into()), true)],
            trailing_comments: thin_vec![],
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
        expr: Box::new(Expr::Lit("stru".into())),
        member: "field".to_string(),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_index() {
    let ast = Expr::Index(ExprIndex {
        expr: Box::new(Expr::Lit("arr".into())),
        index: Box::new(Expr::Lit(0.into())),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_match() {
    let ast = Expr::Match(ExprMatch {
        expr: Box::new(Expr::Lit("x".into())),
        arms: thin_vec![
            Arm {
                pat: pat().tuple(thin_vec![pat().ident("Some", false), pat().ident("y", false)]),
                guard: None,
                body: Box::new(Expr::Lit(1.into())),
            },
            Arm {
                pat: pat().ident("None", false),
                guard: Some(Box::new(Expr::Lit(true.into()))),
                body: Box::new(Expr::Lit(2.into())),
            },
        ],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_method_call() {
    let ast = Expr::MethodCall(ExprMethodCall {
        receiver: Box::new(Expr::Lit("obj".into())),
        method: "method".to_string(),
        args: thin_vec![Expr::Lit(1.into()), Expr::Lit(2.into())],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_paren() {
    let ast = Expr::Paren(ExprParen {
        expr: Box::new(Expr::Lit(1.into())),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_range() {
    let ast = Expr::Range(ExprRange {
        start: Some(Box::new(Expr::Lit(1.into()))),
        limits: RangeLimits::HalfOpen,
        end: Some(Box::new(Expr::Lit(5.into()))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_reference() {
    let ast = Expr::Reference(ExprRef {
        is_mut: true,
        expr: Box::new(Expr::Lit("x".into())),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_return() {
    let ast = Expr::Return(ExprReturn {
        expr: Some(Box::new(Expr::Lit(1.into()))),
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_struct() {
    let ast = Expr::Struct(ExprStruct {
        path: "Foo".to_string(),
        fields: thin_vec![
            FieldValue {
                member: "a".to_string(),
                value: Expr::Lit(1.into()),
            },
            FieldValue {
                member: "b".to_string(),
                value: Expr::Lit(2.into()),
            },
        ],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_expr_tuple() {
    let ast = Expr::Tuple(ExprTuple {
        elems: thin_vec![Expr::Lit(1.into()), Expr::Lit(2.into())],
    });
    insta::assert_snapshot!(pretty_print_expr(ast));
}

#[test]
fn test_long_enum() {
    let ast = Item::Enum(ItemEnum {
        md: None,
        ident: "MyLongLongLongLongLongEnum".to_string(),
        generics: Default::default(),
        variants: thin_vec![
            Variant {
                md: None,
                ident: "AVeryLongVariantNameThatShouldCauseALineBreak".to_string(),
            },
            Variant {
                md: None,
                ident: "AnotherVeryLongVariantNameThatShouldAlsoCauseALineBreak".to_string(),
            },
        ],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_single_field_struct() {
    let ast = Item::Struct(ItemStruct {
        md: None,
        ident: "MyStruct".to_string(),
        generics: Default::default(),
        fields: thin_vec![Field {
            md: None,
            ident: "field".to_string(),
            ty: "i32".into(),
        }],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_nested_struct() {
    let ast = Item::Struct(ItemStruct {
        md: None,
        ident: "Outer".to_string(),
        generics: Default::default(),
        fields: thin_vec![
            Field {
                md: None,
                ident: "inner".to_string(),
                ty: "Inner".into(),
            },
            Field {
                md: None,
                ident: "another_field".to_string(),
                ty: "i32".into(),
            },
        ],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_long_binary_expression() {
    let ast = Item::Fn(ItemFn {
        md: None,
        sig: Signature {
            ident: "foo".to_string(),
            generics: Default::default(),
            inputs: thin_vec![],
            output: None,
        },
        block: Block {
            leading_comments: thin_vec![],
            stmts: thin_vec![Stmt::Expr(
                Expr::Binary(ExprBinary {
                    left: Box::new(Expr::Lit(
                        "a_very_long_string_that_should_cause_a_line_break".into(),
                    )),
                    op: BinOp::Add,
                    right: Box::new(Expr::Lit(
                        "another_very_long_string_that_should_also_cause_a_line_break".into(),
                    )),
                }),
                true,
            )],
            trailing_comments: thin_vec![],
        },
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_trait() {
    let ast = Item::Trait(ItemTrait {
        md: Some(Box::new(Md {
            attrs: thin_vec![],
            leading_comments: thin_vec![Comment::Line(" A simple trait.".to_string())],
            trailing_comments: thin_vec![],
        })),
        ident: "MyTrait".to_string(),
        generics: Default::default(),
        items: thin_vec![TraitItem::Fn(TraitItemFn {
            md: None,
            sig: Signature {
                ident: "my_func".to_string(),
                generics: Default::default(),
                inputs: thin_vec![],
                output: None,
            },
            block: None,
        })],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_loop_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(
                    Expr::Loop(ExprLoop {
                        body: Block {
                            leading_comments: thin_vec![],
                            stmts: thin_vec![Stmt::Expr(Expr::Lit(1.into()), true)],
                            trailing_comments: thin_vec![],
                        },
                    }),
                    true,
                )],
                trailing_comments: thin_vec![],
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
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(
                    Expr::While(ExprWhile {
                        cond: Box::new(Expr::Lit(1.into())),
                        body: Block {
                            leading_comments: thin_vec![],
                            stmts: thin_vec![Stmt::Expr(Expr::Lit(2.into()), true)],
                            trailing_comments: thin_vec![],
                        },
                    }),
                    true,
                )],
                trailing_comments: thin_vec![],
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
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(
                    Expr::For(ExprFor {
                        pat: pat().ident("x", false),
                        expr: Box::new(Expr::Lit(1.into())),
                        body: Block {
                            leading_comments: thin_vec![],
                            stmts: thin_vec![Stmt::Expr(Expr::Lit(2.into()), true)],
                            trailing_comments: thin_vec![],
                        },
                    }),
                    true,
                )],
                trailing_comments: thin_vec![],
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
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(
                    Expr::Assign(ExprAssign {
                        left: Box::new(Expr::Lit("x".into())),
                        right: Box::new(Expr::Lit(1.into())),
                    }),
                    true,
                )],
                trailing_comments: thin_vec![],
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
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(
                    Expr::MacroCall(ExprMacroCall {
                        ident: "println".to_string(),
                        tokens: TokenStream {
                            tokens: thin_vec![TokenTree::Group(Group {
                                delimiter: Delimiter::Parenthesis,
                                stream: TokenStream {
                                    tokens: thin_vec![TokenTree::Literal("hello".into())],
                                },
                            })],
                        },
                    }),
                    true,
                )],
                trailing_comments: thin_vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_enum() {
    let ast = Item::Enum(ItemEnum {
        md: Some(Box::new(Md {
            attrs: thin_vec![],
            leading_comments: thin_vec![Comment::Line(" A simple enum.".to_string())],
            trailing_comments: thin_vec![],
        })),
        ident: "MyEnum".to_string(),
        generics: Default::default(),
        variants: thin_vec![
            Variant {
                md: None,
                ident: "Variant1".to_string(),
            },
            Variant {
                md: None,
                ident: "Variant2".to_string(),
            },
        ],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_impl() {
    let ast = Item::Impl(ItemImpl {
        md: Some(Box::new(Md {
            attrs: thin_vec![],
            leading_comments: thin_vec![Comment::Line(" A simple impl.".to_string())],
            trailing_comments: thin_vec![],
        })),
        generics: Default::default(),
        ty: "MyStruct".into(),
        fns: thin_vec![
            fn_def("new")
                .block(Block {
                    leading_comments: thin_vec![],
                    stmts: thin_vec![],
                    trailing_comments: thin_vec![],
                })
                .build(),
        ],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_let_statement() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: thin_vec![],
                stmts: thin_vec![
                    stmt()
                        .local(pat().ident("x", false))
                        .ty("i32")
                        .expr(Expr::Lit(42.into()))
                        .build(),
                ],
                trailing_comments: thin_vec![],
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
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(
                    Expr::If(ExprIf {
                        cond: Box::new(Expr::Lit(1.into())),
                        then_branch: Block {
                            leading_comments: thin_vec![],
                            stmts: thin_vec![Stmt::Expr(Expr::Lit(2.into()), true)],
                            trailing_comments: thin_vec![],
                        },
                        else_branch: Some(Box::new(Expr::If(ExprIf {
                            cond: Box::new(Expr::Lit(3.into())),
                            then_branch: Block {
                                leading_comments: thin_vec![],
                                stmts: thin_vec![Stmt::Expr(Expr::Lit(4.into()), true)],
                                trailing_comments: thin_vec![],
                            },
                            else_branch: Some(Box::new(Expr::Block(ExprBlock {
                                block: Block {
                                    leading_comments: thin_vec![],
                                    stmts: thin_vec![Stmt::Expr(Expr::Lit(5.into()), true)],
                                    trailing_comments: thin_vec![],
                                },
                            }))),
                        }))),
                    }),
                    true,
                )],
                trailing_comments: thin_vec![],
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
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(
                    Expr::Binary(ExprBinary {
                        left: Box::new(Expr::Lit(1.into())),
                        op: BinOp::Add,
                        right: Box::new(Expr::Lit(2.into())),
                    }),
                    true,
                )],
                trailing_comments: thin_vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_expr_statement_without_semicolon() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Expr(Expr::Lit(42.into()), false)],
                trailing_comments: thin_vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_item_statement() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::Item(Item::Struct(ItemStruct {
                    md: None,
                    ident: "MyStruct".to_string(),
                    generics: Default::default(),
                    fields: thin_vec![],
                }))],
                trailing_comments: thin_vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_macro_call_statement() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(Block {
                leading_comments: thin_vec![],
                stmts: thin_vec![Stmt::MacCall(ExprMacroCall {
                    ident: "println".to_string(),
                    tokens: TokenStream {
                        tokens: thin_vec![TokenTree::Group(Group {
                            delimiter: Delimiter::Parenthesis,
                            stream: TokenStream {
                                tokens: thin_vec![TokenTree::Literal("hello".into())],
                            },
                        })],
                    },
                })],
                trailing_comments: thin_vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_all_literals() {
    let ast = Item::Fn(
        fn_def("literals")
            .block(Block {
                leading_comments: thin_vec![],
                stmts: thin_vec![
                    stmt().local("s").expr(Expr::Lit("hello".into())).build(),
                    stmt()
                        .local("bs")
                        .expr(Expr::Lit(Lit::ByteStr(LitByteStr::new(b"hello"))))
                        .build(),
                    stmt()
                        .local("cs")
                        .expr(Expr::Lit(Lit::CStr(LitCStr::new("hello"))))
                        .build(),
                    stmt()
                        .local("b")
                        .expr(Expr::Lit(Lit::Byte(LitByte::new(b'h'))))
                        .build(),
                    stmt()
                        .local("c")
                        .expr(Expr::Lit(Lit::Char(LitChar::new('h'))))
                        .build(),
                    stmt().local("i").expr(Expr::Lit(42.into())).build(),
                    stmt()
                        .local("i_suffix")
                        .expr(Expr::Lit(Lit::Int(LitInt::new(42).with_suffix(IntSuffix::U32))))
                        .build(),
                    stmt()
                        .local("f")
                        .expr(Expr::Lit(Lit::Float(LitFloat::new("1.23"))))
                        .build(),
                    stmt()
                        .local("f_suffix")
                        .expr(Expr::Lit(Lit::Float(
                            LitFloat::new("1.23").with_suffix(FloatSuffix::F32),
                        )))
                        .build(),
                    stmt().local("t").expr(Expr::Lit(true.into())).build(),
                ],
                trailing_comments: thin_vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_struct() {
    let ast = Item::Struct(ItemStruct {
        md: Some(Box::new(Md {
            attrs: thin_vec![],
            leading_comments: thin_vec![Comment::Line(" A simple struct.".to_string())],
            trailing_comments: thin_vec![],
        })),
        ident: "Foo".to_string(),
        generics: Default::default(),
        fields: thin_vec![
            Field {
                md: None,
                ident: "field1".to_string(),
                ty: "i32".into(),
            },
            Field {
                md: None,
                ident: "field2".to_string(),
                ty: "String".into(),
            },
        ],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}
