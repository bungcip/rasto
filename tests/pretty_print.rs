use rasto::ast::builder::{file, fn_def};
use rasto::ast::*;
use rasto::pretty_print::{Formatter, PrettyPrint};

fn pretty_print_item(item: Item) -> String {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);
    item.pretty_print(&mut fmt).unwrap();
    buf
}

fn pretty_print_file(file: File) -> String {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);
    file.pretty_print(&mut fmt).unwrap();
    buf
}

#[test]
fn test_file() {
    let ast = file()
        .item(Item::Struct(ItemStruct {
            leading_comments: vec![Comment::Line(" A simple struct.".to_string())],
            ident: "Foo".to_string(),
            fields: vec![
                Field {
                    ident: "field1".to_string(),
                    ty: "i32".to_string(),
                },
                Field {
                    ident: "field2".to_string(),
                    ty: "String".to_string(),
                },
            ],
            trailing_comments: vec![],
        }))
        .item(
            fn_def("foo")
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
            .block(Block {
                leading_comments: vec![Comment::Block(" An inner comment ".to_string())],
                stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(42)))],
                trailing_comments: vec![],
            })
            .build(),
    );

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_trait() {
    let ast = Item::Trait(ItemTrait {
        leading_comments: vec![Comment::Line(" A simple trait.".to_string())],
        ident: "MyTrait".to_string(),
        items: vec![TraitItem::Fn(TraitItemFn {
            leading_comments: vec![],
            sig: Signature {
                ident: "my_func".to_string(),
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
        leading_comments: vec![Comment::Line(" A simple enum.".to_string())],
        ident: "MyEnum".to_string(),
        variants: vec![
            Variant {
                ident: "Variant1".to_string(),
            },
            Variant {
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
        leading_comments: vec![Comment::Line(" A simple impl.".to_string())],
        ident: "MyStruct".to_string(),
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
                    ty: Some("i32".to_string()),
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
        leading_comments: vec![Comment::Line(" A simple struct.".to_string())],
        ident: "Foo".to_string(),
        fields: vec![
            Field {
                ident: "field1".to_string(),
                ty: "i32".to_string(),
            },
            Field {
                ident: "field2".to_string(),
                ty: "String".to_string(),
            },
        ],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}
