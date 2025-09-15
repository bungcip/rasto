use rasto::ast::*;
use rasto::builder::{
    block, comment, enum_def, expr, field_value, file, fn_def, impl_block, pat, stmt, struct_def,
    trait_def, trait_item_fn, tt,
};
use rasto::pretty;
use thin_vec::thin_vec;

mod patterns;

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
                .input("a")
                .output("i32")
                .statement(expr().lit(42))
                .build(),
        )
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_macro_call_with_brackets() {
    let ast = fn_def("foo")
        .statement(expr().macro_call(
            "vec",
            Delimiter::Bracket,
            thin_vec![tt().lit(0), tt().punct(';', Spacing::Alone), tt().lit(256)],
        ))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_pretty_print_doc_comment() {
    let a = file()
        .item(
            fn_def("foo")
                .leading_comment(comment().doc(" This is a doc comment."))
                .build(),
        )
        .build();

    insta::assert_snapshot!(pretty(&a), @r"
    /// This is a doc comment.
    fn foo() {}
    ");
}

#[test]
fn test_block_single_comment() {
    let single = comment().block("Block comment with single line");
    insta::assert_snapshot!(pretty(&single));
}

#[test]
fn test_block_multiline_comment() {
    let single =
        comment().block("Block comment with multi line 1\nBlock comment with multi line 2");
    insta::assert_snapshot!(pretty(&single));
}

#[test]
fn test_fn() {
    let ast = fn_def("foo")
        .input("a")
        .output("i32")
        .block(
            block()
                .leading_comment(comment().block(" Block comment with single line "))
                .statement(stmt().local("hello").expr(expr().lit("world")))
                .statement(expr().lit(42))
                .has_trailing_semicolon(false),
        )
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_block_with_comments() {
    let ast = block()
        .leading_comment(comment().line(" leading comment"))
        .statement(expr().lit(42))
        .trailing_comment(comment().line(" trailing comment"))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn unary_expression() {
    let ast = expr().unary(UnOp::Neg, expr().lit(42));
    insta::assert_snapshot!(pretty(&ast), @"-42");
}

#[test]
fn unary_expression_not() {
    let ast = expr().unary(UnOp::Not, expr().lit(true));
    insta::assert_snapshot!(pretty(&ast), @"!true");
}

#[test]
fn test_expr_array() {
    let ast = expr().array([expr().lit(1), expr().lit(2)]);
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_async() {
    let ast = expr().async_block([expr().lit(1)]);
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_await() {
    let ast = expr().await_expr(expr().lit("future"));
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_break() {
    let ast = expr().break_expr();
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_call() {
    let ast = expr().call(expr().lit("foo"), [expr().lit(1), expr().lit(2)]);
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_cast() {
    let ast = expr().cast(expr().lit("x"), "u32");
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_closure() {
    let ast = expr().closure([pat().ident("a"), pat().ident("b")], expr().lit(1));
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_const() {
    let ast = expr().const_block([expr().lit(1)]);
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_continue() {
    let ast = expr().continue_expr();
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_field() {
    let ast = expr().field(expr().lit("stru"), "field");
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_index() {
    let ast = expr().index(expr().lit("arr"), expr().lit(0));
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_match() {
    let ast = expr().match_expr(
        expr().lit("x"),
        vec![
            expr()
                .arm(pat().tuple([pat().ident("Some"), pat().ident("y")]))
                .body(expr().lit(1))
                .build(),
            expr()
                .arm(pat().ident("None"))
                .guard(expr().lit(true))
                .body(expr().lit(2))
                .build(),
        ],
    );
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_method_call() {
    let ast = expr().method_call(expr().lit("obj"), "method", [expr().lit(1), expr().lit(2)]);
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_paren() {
    let ast = expr().paren(expr().lit(1));
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_range() {
    let ast = expr().range(
        Some(expr().lit(1)),
        RangeLimits::HalfOpen,
        Some(expr().lit(5)),
    );
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_reference() {
    let ast = expr().reference(true, expr().lit("x"));
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_raw_ref() {
    let const_ast = expr().raw_ref(expr().lit("x")).build();
    insta::assert_snapshot!(pretty(&const_ast), @"&raw const \"x\"");

    let mut_ast = expr().raw_ref(expr().lit("x")).mutable().build();
    insta::assert_snapshot!(pretty(&mut_ast), @"&raw mut \"x\"");
}

#[test]
fn test_expr_return() {
    let ast = expr().return_expr(Some(expr().lit(1)));
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_struct() {
    let ast = expr().struct_expr(
        "Foo",
        vec![
            field_value("a", expr().lit(1)),
            field_value("b", expr().lit(2)),
        ],
    );
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_tuple() {
    let ast = expr().tuple(vec![expr().lit(1), expr().lit(2)]);
    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_long_enum() {
    let ast = enum_def("MyLongLongLongLongLongEnum")
        .variant("AVeryLongVariantNameThatShouldCauseALineBreak")
        .variant("AnotherVeryLongVariantNameThatShouldAlsoCauseALineBreak")
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_single_field_struct() {
    let ast = struct_def("MyStruct").field("field", "i32").build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_nested_struct() {
    let ast = struct_def("Outer")
        .field("inner", "Inner")
        .field("another_field", "i32")
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_long_binary_expression() {
    let ast = fn_def("foo")
        .statement(expr().binary(
            expr().lit("a_very_long_string_that_should_cause_a_line_break"),
            BinOp::Add,
            expr().lit("another_very_long_string_that_should_also_cause_a_line_break"),
        ))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_trait() {
    let ast = trait_def("MyTrait")
        .leading_comment(comment().line(" A simple trait."))
        .item(trait_item_fn("my_func"))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_loop_expression() {
    let ast = fn_def("foo")
        .statement(expr().loop_expr([expr().lit(1)]))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_while_expression() {
    let ast = fn_def("foo")
        .statement(expr().while_loop(expr().lit(1), [expr().lit(2)]))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_for_expression() {
    let ast = fn_def("foo")
        .statement(expr().for_loop("x", expr().lit(1), [expr().lit(2)]))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_assign_expression() {
    let ast = fn_def("foo")
        .statement(expr().assign(expr().lit("x"), expr().lit(1)))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_macro_call_expression() {
    let ast = fn_def("foo")
        .statement(expr().macro_call(
            "println",
            Delimiter::Parenthesis,
            thin_vec![tt().lit("hello")],
        ))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_macro_call_expression_with_path() {
    let ast = fn_def("foo")
        .statement(expr().macro_call(
            path("std").segment("println"),
            Delimiter::Parenthesis,
            thin_vec![tt().lit("hello")],
        ))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_enum() {
    let ast = enum_def("MyEnum")
        .leading_comment(comment().line(" A simple enum."))
        .variant("Variant1")
        .variant("Variant2")
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_impl() {
    let ast = impl_block("MyStruct").item(fn_def("new").build()).build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_trait_impl() {
    let ast = impl_block("MyStruct")
        .trait_("MyTrait")
        .item(fn_def("new").build())
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_unsafe_trait_impl() {
    let ast = impl_block("MyStruct")
        .trait_("MyTrait")
        .unsafe_()
        .item(fn_def("new").build())
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_negative_impl() {
    let ast = impl_block("MyStruct").trait_("MyTrait").negative().build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_let_statement() {
    let ast = fn_def("foo")
        .statement(stmt().local("x").ty("i32").expr(expr().lit(42)))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_if_expression() {
    let ast = fn_def("foo")
        .statement(expr().if_expr(
            expr().lit(1),
            [expr().lit(2)],
            Some(expr().if_expr(
                expr().lit(3),
                [expr().lit(4)],
                Some(expr().block([expr().lit(5)])),
            )),
        ))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_binary_expression() {
    let ast = fn_def("foo")
        .statement(expr().binary(expr().lit(1), BinOp::Add, expr().lit(2)))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_expr_statement_without_semicolon() {
    let ast = fn_def("foo")
        .has_trailing_semicolon(false)
        .statement(expr().lit(42))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_item_statement() {
    let ast = fn_def("foo")
        .statement(stmt().item(struct_def("MyStruct").build()))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

use rasto::builder::path;

#[test]
fn test_macro_call_statement() {
    let ast = fn_def("foo")
        .statement(stmt().mac_call(ExprMacroCall {
            path: path("println").build(),
            delimiter: Delimiter::Parenthesis,
            tokens: thin_vec![tt().lit("hello")].into(),
        }))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_all_literals() {
    let ast = fn_def("literals")
        .statement(stmt().local("s").expr(expr().lit("hello")))
        .statement(stmt().local("bs").expr(expr().lit(b"hello")))
        .statement(stmt().local("cs").expr(expr().lit(c"hello")))
        .statement(stmt().local("b").expr(expr().lit(b'h')))
        .statement(stmt().local("c").expr(expr().lit('h')))
        .statement(stmt().local("i").expr(expr().lit(42)))
        .statement(
            stmt()
                .local("i_suffix")
                .expr(expr().int_lit_with_suffix(42, IntSuffix::U32)),
        )
        .statement(stmt().local("f").expr(expr().lit(1.23)))
        .statement(
            stmt()
                .local("f_suffix")
                .expr(expr().lit(Lit::Float(LitFloat::with_suffix("1.23", FloatSuffix::F32)))),
        )
        .statement(stmt().local("t").expr(expr().lit(true)))
        .build();

    insta::assert_snapshot!(pretty(&ast));
}

#[test]
fn test_struct() {
    let ast = struct_def("Foo")
        .leading_comment(comment().line(" A simple struct."))
        .field("field1", "i32")
        .field("field2", "String")
        .build();

    insta::assert_snapshot!(pretty(&ast));
}
