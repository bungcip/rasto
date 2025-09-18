use rasto::builder::{attr, fn_def, meta};
use rasto::pretty;

#[test]
fn test_fn_with_test_attribute() {
    let item = fn_def("my_func").attr(attr().meta("test")).build();

    insta::assert_snapshot!(pretty(&item), @r###"
    #[test]
    fn my_func() {}
    "###);
}

#[test]
fn test_fn_with_derive_attribute() {
    let item = fn_def("my_func")
        .attr(attr().meta(meta().list("derive", ["Debug", "Clone"])))
        .build();

    insta::assert_snapshot!(pretty(&item), @r###"
    #[derive(Debug, Clone)]
    fn my_func() {}
    "###);
}
