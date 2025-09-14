use rasto::builder::{attr, fn_def, meta};
use rasto::pretty;

#[test]
fn test_fn_with_attributes() {
    let item = fn_def("my_func")
        .attr(attr().meta("test"))
        .attr(attr().meta(meta().list("derive", ["Debug", "Clone"])))
        .build();

    insta::assert_snapshot!(pretty(&item), @r###"
    #[test]
    #[derive(Debug, Clone)]
    fn my_func() {}
    "###);
}
