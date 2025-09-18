use rasto::ast::*;
use rasto::builder::{attr, block, comment, expr, file, fn_def, meta, stmt, tt};
use rasto::pretty;
use thin_vec::thin_vec;

fn main() {
    let ast = file()
        .attr(
            attr()
                .inner()
                .meta(meta().list("deny", [meta().path("missing_docs")])),
        )
        .item(
            fn_def("main")
                .comment(comment().doc(" this is main function"))
                .statement(expr().macro_call(
                    "println",
                    Delimiter::Parenthesis,
                    thin_vec![tt().lit("\"hello world\"")],
                ))
                .statement(stmt().local("x").expr(expr().lit(20)))
                .statement(expr().call(expr().path("call"), [expr().path("x")])),
        )
        .item(
            fn_def("call").input_typed("x", "i32").output("i32").block(
                block()
                    .statement(
                        stmt().local("z").expr(
                            expr().if_expr(
                                expr().binary(expr().path("x"), BinOp::Gt, expr().lit(20)),
                                block()
                                    .statement(expr().lit(11))
                                    .has_trailing_semicolon(false),
                                Some(
                                    expr().block(
                                        block()
                                            .statement(expr().lit(30))
                                            .has_trailing_semicolon(false),
                                    ),
                                ),
                            ),
                        ),
                    )
                    .statement(expr().path("z"))
                    .has_trailing_semicolon(false),
            ),
        )
        .build();

    println!("{}", pretty(&ast));
}
