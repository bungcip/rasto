use rasto::ast::*;
use rasto::pretty_print::{Formatter, PrettyPrint};

fn pretty_print_item(item: Item) -> String {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);
    item.pretty_print(&mut fmt).unwrap();
    buf
}

#[test]
fn test_fn() {
    let ast = Item::Fn(ItemFn {
        leading_comments: vec![Comment::Line(" A simple function.".to_string())],
        sig: Signature {
            ident: "foo".to_string(),
        },
        block: Block {
            leading_comments: vec![Comment::Block(" An inner comment ".to_string())],
            stmts: vec![Stmt::Expr(Expr::Lit(Lit::Int(42)))],
            trailing_comments: vec![],
        },
        trailing_comments: vec![Comment::Line(" Trailing comment.".to_string())],
    });

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
        fns: vec![ItemFn {
            leading_comments: vec![],
            sig: Signature {
                ident: "new".to_string(),
            },
            block: Block {
                leading_comments: vec![],
                stmts: vec![],
                trailing_comments: vec![],
            },
            trailing_comments: vec![],
        }],
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_let_statement() {
    let ast = Item::Fn(ItemFn {
        leading_comments: vec![],
        sig: Signature {
            ident: "foo".to_string(),
        },
        block: Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Let(StmtLet {
                ident: "x".to_string(),
                ty: Some("i32".to_string()),
                expr: Some(Expr::Lit(Lit::Int(42))),
            })],
            trailing_comments: vec![],
        },
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_if_expression() {
    let ast = Item::Fn(ItemFn {
        leading_comments: vec![],
        sig: Signature {
            ident: "foo".to_string(),
        },
        block: Block {
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
        },
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(pretty_print_item(ast));
}

#[test]
fn test_binary_expression() {
    let ast = Item::Fn(ItemFn {
        leading_comments: vec![],
        sig: Signature {
            ident: "foo".to_string(),
        },
        block: Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(Expr::Binary(ExprBinary {
                left: Box::new(Expr::Lit(Lit::Int(1))),
                op: BinOp::Add,
                right: Box::new(Expr::Lit(Lit::Int(2))),
            }))],
            trailing_comments: vec![],
        },
        trailing_comments: vec![],
    });

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
