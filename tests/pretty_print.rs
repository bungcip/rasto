use rasto::ast::builder::{
    block, comment, enum_def, file, fn_def, pat, stmt, struct_def, trait_def,
};
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

fn pretty_print_comment(comment: Comment) -> String {
    let mut buf = String::new();
    let mut printer = Printer::new(&mut buf);
    comment.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();
    buf
}

fn pretty_print_file(file: File) -> String {
    file.to_string()
}

#[test]
fn test_file() {
    let ast = file()
        .item(
            struct_def("Foo")
                .leading_comment(comment().line(" A simple struct."))
                .field("field1", "i32")
                .field("field2", "String")
                .build(),
        )
        .item(
            fn_def("foo")
                .input(pat().ident("a", false))
                .output("i32")
                .block(
                    block()
                        .statement(Stmt::Expr(Expr::Lit(42.into())))
                        .has_trailing_semicolon(true),
                )
                .build(),
        )
        .build();

    insta::assert_snapshot!(pretty_print_file(ast));
}

#[test]
fn test_block_single_comment() {
    let single = comment().block("Block comment with single line");
    insta::assert_snapshot!(pretty_print_comment(single));
}

#[test]
fn test_block_multiline_comment() {
    let single =
        comment().block("Block comment with multi line 1\nBlock comment with multi line 2");
    insta::assert_snapshot!(pretty_print_comment(single));
}

#[test]
fn test_fn() {
    let ast = Item::Fn(
        fn_def("foo")
            .input(pat().ident("a", false))
            .output("i32")
            .block(
                block()
                    .leading_comment(comment().block(" Block comment with single line "))
                    .statement(
                        stmt()
                            .local(pat().ident("hello", false))
                            .expr(expr().lit("world")),
                    )
                    .statement(expr().lit(42))
                    .has_trailing_semicolon(false),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_block_with_comments() {
    let ast = block()
        .leading_comment(comment().line(" leading comment"))
        .statement(Stmt::Expr(Expr::Lit(42.into())))
        .has_trailing_semicolon(true)
        .trailing_comment(comment().line(" trailing comment"))
        .build();

    let mut buf = String::new();
    let mut printer = Printer::new(&mut buf);
    ast.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();

    insta::assert_snapshot!(buf);
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
        block: block()
            .statement(Stmt::Expr(Expr::Lit(1.into())))
            .has_trailing_semicolon(true)
            .build(),
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
        block: block()
            .statement(Stmt::Expr(Expr::Lit(1.into())))
            .has_trailing_semicolon(true)
            .build(),
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
                pat: pat().tuple(thin_vec![
                    pat().ident("Some", false),
                    pat().ident("y", false)
                ]),
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
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::Binary(ExprBinary {
                        left: Box::new(Expr::Lit(
                            "a_very_long_string_that_should_cause_a_line_break".into(),
                        )),
                        op: BinOp::Add,
                        right: Box::new(Expr::Lit(
                            "another_very_long_string_that_should_also_cause_a_line_break".into(),
                        )),
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_trait() {
    let ast = Item::Trait(
        trait_def("MyTrait")
            .leading_comment(comment().line(" A simple trait."))
            .item(TraitItem::Fn(TraitItemFn {
                md: None,
                sig: Signature {
                    ident: "my_func".to_string(),
                    generics: Default::default(),
                    inputs: thin_vec![],
                    output: None,
                },
                block: None,
            }))
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_loop_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::Loop(ExprLoop {
                        body: block()
                            .statement(Stmt::Expr(Expr::Lit(1.into())))
                            .has_trailing_semicolon(true)
                            .build(),
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_while_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::While(ExprWhile {
                        cond: Box::new(Expr::Lit(1.into())),
                        body: block()
                            .statement(Stmt::Expr(Expr::Lit(2.into())))
                            .has_trailing_semicolon(true)
                            .build(),
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_for_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::For(ExprFor {
                        pat: pat().ident("x", false),
                        expr: Box::new(Expr::Lit(1.into())),
                        body: block()
                            .statement(Stmt::Expr(Expr::Lit(2.into())))
                            .has_trailing_semicolon(true)
                            .build(),
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_assign_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::Assign(ExprAssign {
                        left: Box::new(Expr::Lit("x".into())),
                        right: Box::new(Expr::Lit(1.into())),
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_macro_call_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::MacroCall(ExprMacroCall {
                        ident: "println".to_string(),
                        tokens: TokenStream {
                            tokens: thin_vec![TokenTree::Group(Group {
                                delimiter: Delimiter::Parenthesis,
                                stream: TokenStream {
                                    tokens: thin_vec![TokenTree::Literal("hello".into())],
                                },
                            })],
                        },
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_enum() {
    let ast = Item::Enum(
        enum_def("MyEnum")
            .leading_comment(comment().line(" A simple enum."))
            .variant("Variant1")
            .variant("Variant2")
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_impl() {
    let ast = Item::Impl(
        impl_block("MyStruct")
            .function(fn_def("new").block(block()).build())
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_trait_impl() {
    let ast = Item::Impl(
        impl_block("MyStruct")
            .trait_("MyTrait")
            .function(fn_def("new").block(block()).build())
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_unsafe_trait_impl() {
    let ast = Item::Impl(
        impl_block("MyStruct")
            .trait_("MyTrait")
            .unsafe_()
            .function(fn_def("new").block(block()).build())
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_negative_impl() {
    let ast = Item::Impl(impl_block("MyStruct").trait_("MyTrait").negative().build());

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_let_statement() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block().statement(
                    stmt()
                        .local(pat().ident("x", false))
                        .ty("i32")
                        .expr(Expr::Lit(42.into()))
                        .build(),
                ),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_if_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::If(ExprIf {
                        cond: Box::new(Expr::Lit(1.into())),
                        then_branch: block()
                            .statement(Stmt::Expr(Expr::Lit(2.into())))
                            .has_trailing_semicolon(true)
                            .build(),
                        else_branch: Some(Box::new(Expr::If(ExprIf {
                            cond: Box::new(Expr::Lit(3.into())),
                            then_branch: block()
                                .statement(Stmt::Expr(Expr::Lit(4.into())))
                                .has_trailing_semicolon(true)
                                .build(),
                            else_branch: Some(Box::new(Expr::Block(ExprBlock {
                                block: block()
                                    .statement(Stmt::Expr(Expr::Lit(5.into())))
                                    .has_trailing_semicolon(true)
                                    .build(),
                            }))),
                        }))),
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_binary_expression() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::Binary(ExprBinary {
                        left: Box::new(Expr::Lit(1.into())),
                        op: BinOp::Add,
                        right: Box::new(Expr::Lit(2.into())),
                    })))
                    .has_trailing_semicolon(true),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_expr_statement_without_semicolon() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(
                block()
                    .statement(Stmt::Expr(Expr::Lit(42.into())))
                    .has_trailing_semicolon(false),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_item_statement() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(block().statement(Stmt::Item(Item::Struct(ItemStruct {
                md: None,
                ident: "MyStruct".to_string(),
                generics: Default::default(),
                fields: thin_vec![],
            }))))
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_macro_call_statement() {
    let ast = Item::Fn(
        fn_def("foo")
            .block(block().statement(Stmt::MacCall(ExprMacroCall {
                ident: "println".to_string(),
                tokens: TokenStream {
                    tokens: thin_vec![TokenTree::Group(Group {
                        delimiter: Delimiter::Parenthesis,
                        stream: TokenStream {
                            tokens: thin_vec![TokenTree::Literal("hello".into())],
                        },
                    })],
                },
            })))
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_all_literals() {
    let ast = Item::Fn(
        fn_def("literals")
            .block(
                block()
                    .statement(stmt().local("s").expr(Expr::Lit("hello".into())).build())
                    .statement(
                        stmt()
                            .local("bs")
                            .expr(Expr::Lit(Lit::ByteStr(LitByteStr::new(b"hello"))))
                            .build(),
                    )
                    .statement(
                        stmt()
                            .local("cs")
                            .expr(Expr::Lit(Lit::CStr(LitCStr::new("hello"))))
                            .build(),
                    )
                    .statement(
                        stmt()
                            .local("b")
                            .expr(Expr::Lit(Lit::Byte(LitByte::new(b'h'))))
                            .build(),
                    )
                    .statement(
                        stmt()
                            .local("c")
                            .expr(Expr::Lit(Lit::Char(LitChar::new('h'))))
                            .build(),
                    )
                    .statement(stmt().local("i").expr(Expr::Lit(42.into())).build())
                    .statement(
                        stmt()
                            .local("i_suffix")
                            .expr(Expr::Lit(Lit::Int(
                                LitInt::new(42).with_suffix(IntSuffix::U32),
                            )))
                            .build(),
                    )
                    .statement(
                        stmt()
                            .local("f")
                            .expr(Expr::Lit(Lit::Float(LitFloat::new("1.23"))))
                            .build(),
                    )
                    .statement(
                        stmt()
                            .local("f_suffix")
                            .expr(Expr::Lit(Lit::Float(
                                LitFloat::new("1.23").with_suffix(FloatSuffix::F32),
                            )))
                            .build(),
                    )
                    .statement(stmt().local("t").expr(Expr::Lit(true.into())).build()),
            )
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_struct() {
    let ast = Item::Struct(
        struct_def("Foo")
            .leading_comment(comment().line(" A simple struct."))
            .field("field1", "i32")
            .field("field2", "String")
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}
