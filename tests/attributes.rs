use rasto::ast::*;
use rasto::builder;
use rasto::pretty;

#[test]
fn test_fn_with_attributes() {
    let item = Item::from(
        builder::fn_def("my_func")
            .attr(builder::attr().meta(builder::meta().path("test")))
            .attr(builder::attr().meta(builder::meta().list(
                "derive",
                [builder::meta().path("Debug"), builder::meta().path("Clone")],
            )))
            .build(),
    );

    insta::assert_snapshot!(pretty(&item), @r###"
    #[test]
    #[derive(Debug, Clone)]
    fn my_func() {}
    "###);
}
