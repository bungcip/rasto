use rasto::ast::generics::{GenericArgs, GenericParam, generic_param};
use rasto::ast::types::Type;
use rasto::{builder::*, pretty};

#[test]
fn test_fn_with_generics() {
    let ast = fn_def("my_function")
        .generic(generic_param().ty("T"))
        .generic(generic_param().lifetime("a"))
        .build();

    insta::assert_snapshot!(pretty(&ast), @r###"
    fn my_function<T, 'a>() {}
    "###);
}

#[test]
fn test_struct_with_generics() {
    let ast = struct_def("MyStruct")
        .generic(generic_param().ty("T"))
        .field("my_field", "T")
        .build();

    insta::assert_snapshot!(pretty(&ast), @r"
    struct MyStruct<T> {
        my_field: T,
    }
    ");
}

#[test]
fn test_generic_param_from_lifetime() {
    let param: GenericParam = generic_param().lifetime("a").into();
    assert!(matches!(param, GenericParam::Lifetime(_)));
}

#[test]
fn test_generic_param_from_type() {
    let param: GenericParam = generic_param().ty("T").into();
    assert!(matches!(param, GenericParam::Type(_)));
}

#[test]
fn test_generic_param_from_const() {
    let param: GenericParam = generic_param().const_("N", "usize").into();
    assert!(matches!(param, GenericParam::Const(_)));
}

#[test]
fn test_generic_args_new() {
    let args: GenericArgs = GenericArgs::new();
    assert!(args.args.is_empty());
}

#[test]
fn test_enum_with_generics() {
    let ast = enum_def("MyEnum")
        .generic(generic_param().ty("T"))
        .variant("MyVariant")
        .build();

    insta::assert_snapshot!(pretty(&ast), @r"
    enum MyEnum<T> {
        MyVariant,
    }
    ");
}

#[test]
fn test_impl_with_generics() {
    let ast = impl_block(Type::from("MyTrait"))
        .generic(generic_param().ty("T"))
        .build();

    insta::assert_snapshot!(pretty(&ast), @r###"
    impl<T> MyTrait {}
    "###);
}

#[test]
fn test_trait_with_generics() {
    let ast = trait_def("MyTrait")
        .generic(generic_param().ty("T"))
        .build();

    insta::assert_snapshot!(pretty(&ast), @r###"
    trait MyTrait<T> {}
    "###);
}

#[test]
fn test_type_with_generics() {
    let ast = type_alias("MyType", path("Vec").generic("T").build_type())
        .generic(generic_param().ty("T"))
        .build();

    insta::assert_snapshot!(pretty(&ast), @r###"
    type MyType<T> = Vec<T>;
    "###);
}

#[test]
fn test_union_with_generics() {
    let ast = union_item("MyUnion")
        .generic(generic_param().ty("T"))
        .field("my_field", "T")
        .build();

    insta::assert_snapshot!(pretty(&ast), @r"
    union MyUnion<T> {
        my_field: T,
    }
    ");
}
