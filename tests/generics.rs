use rasto::ast::generics::generic_param;
use rasto::ast::types::Type;
use rasto::builder::*;

#[test]
fn test_fn_with_generics() {
    let func = fn_def("my_function")
        .generic(generic_param().ty("T"))
        .generic(generic_param().lifetime("a"))
        .build();

    insta::assert_snapshot!(func.to_string(), @r###"
    fn my_function<T, 'a>() {}
    "###);
}

#[test]
fn test_struct_with_generics() {
    let s = struct_def("MyStruct")
        .generic(generic_param().ty("T"))
        .field("my_field", Type::from("T"))
        .build();

    insta::assert_snapshot!(s.to_string(), @r"
    struct MyStruct<T> {
        my_field: T,
    }
    ");
}

#[test]
fn test_enum_with_generics() {
    let e = enum_def("MyEnum")
        .generic(generic_param().ty("T"))
        .variant("MyVariant")
        .build();

    insta::assert_snapshot!(e.to_string(), @r"
    enum MyEnum<T> {
        MyVariant,
    }
    ");
}

#[test]
fn test_impl_with_generics() {
    let i = impl_block(Type::from("MyTrait"))
        .generic(generic_param().ty("T"))
        .build();

    insta::assert_snapshot!(i.to_string(), @r###"
    impl<T> MyTrait {}
    "###);
}

#[test]
fn test_trait_with_generics() {
    let t = trait_def("MyTrait")
        .generic(generic_param().ty("T"))
        .build();

    insta::assert_snapshot!(t.to_string(), @r###"
    trait MyTrait<T> {}
    "###);
}

#[test]
fn test_type_with_generics() {
    let t = def_item(
        "MyType",
        type_alias_kind(path("Vec").generic("T").build_type()).generic(generic_param().ty("T")),
    )
    .build();

    insta::assert_snapshot!(t.to_string(), @r###"
    type MyType<T> = Vec<T>;
    "###);
}

#[test]
fn test_union_with_generics() {
    let u = union_item("MyUnion")
        .generic(generic_param().ty("T"))
        .field("my_field", Type::from("T"))
        .build();

    insta::assert_snapshot!(u.to_string(), @r"
    union MyUnion<T> {
        my_field: T,
    }
    ");
}
