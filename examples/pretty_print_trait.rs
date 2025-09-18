use rasto::ast::*;
use rasto::builder::{block, expr, file, fn_def, impl_block, stmt, struct_def, trait_def, trait_item_fn, path};
use rasto::pretty;

fn main() {
    let ast = file()
        .item(
            trait_def("Something").item(
                trait_item_fn("do_something")
                    .output(path("String").build_type())
                    .build(),
            ),
        )
        .item(
            struct_def("Foo")
                .vis(Visibility::Public)
                .field("value", "i32")
                .build(),
        )
        .item(
            impl_block(path("Foo").build_type())
                .trait_(path("Something").build_type())
                .item(
                    fn_def("do_something")
                        .output(path("String").build_type())
                        .block(
                            block()
                                .statement(
                                    stmt()
                                        .local("name")
                                        .expr(
                                            expr().method_call(
                                                expr().lit("cat"),
                                                "to_string",
                                                [],
                                            ),
                                        ),
                                )
                                .statement(expr().path("name"))
                                .has_trailing_semicolon(false),
                        ),
                )
                .build(),
        )
        .build();

    println!("{}", pretty(&ast));
}
