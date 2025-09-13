use rasto::ast::*;

#[test]
fn test_fn_with_attributes() {
    let item = Item::from(
        builder::fn_def("my_func")
            .attr(builder::attr().meta(builder::meta().path("test")))
            .attr(builder::attr().meta(builder::meta().list(
                "derive",
                [builder::meta().path("Debug"), builder::meta().path("Clone")],
            )))
            .block(block())
            .build(),
    );

    insta::assert_snapshot!(item.to_string(), @r###"
    #[test]
    #[derive(Debug, Clone)]
    fn my_func() {}
    "###);
}
