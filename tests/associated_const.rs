use rasto::builder::*;

#[test]
fn test_trait_with_associated_const() {
    let tr = trait_def("MyTrait")
        .item(associated_const("MY_CONST", "u32").build())
        .build();

    insta::assert_snapshot!(tr, @r###"
    trait MyTrait {
        const MY_CONST: u32;
    }
    "###);
}

#[test]
fn test_impl_with_associated_const() {
    let im = impl_block("MyType")
        .trait_("MyTrait")
        .item(
            associated_const("MY_CONST", "u32")
                .expr(expr().lit(42))
                .build(),
        )
        .build();

    insta::assert_snapshot!(im, @r###"
    impl MyTrait for MyType {
        const MY_CONST: u32 = 42;
    }
    "###);
}
