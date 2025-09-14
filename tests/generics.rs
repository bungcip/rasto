use rasto::ast::PathSegment;
use rasto::ast::generics::*;
use rasto::ast::types::{Type, TypePath};
use rasto::builder::*;
use thin_vec::thin_vec;

#[test]
fn test_fn_with_generics() {
    let func = fn_def("my_function")
        .generic(TypeParam {
            ident: "T".to_string(),
            bounds: vec![],
        })
        .generic(LifetimeParam {
            ident: "a".to_string(),
        })
        .block(block())
        .build();

    insta::assert_snapshot!(func.to_string(), @r###"
    fn my_function<T, 'a>() {}
    "###);
}

#[test]
fn test_struct_with_generics() {
    let s = struct_def("MyStruct")
        .generic(TypeParam {
            ident: "T".to_string(),
            bounds: vec![],
        })
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
        .generic(TypeParam {
            ident: "T".to_string(),
            bounds: vec![],
        })
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
        .generic(TypeParam {
            ident: "T".to_string(),
            bounds: vec![],
        })
        .build();

    insta::assert_snapshot!(i.to_string(), @r###"
    impl<T> MyTrait {}
    "###);
}

#[test]
fn test_trait_with_generics() {
    let t = trait_def("MyTrait")
        .generic(TypeParam {
            ident: "T".to_string(),
            bounds: vec![],
        })
        .build();

    insta::assert_snapshot!(t.to_string(), @r###"
    trait MyTrait<T> {}
    "###);
}

#[test]
fn test_type_with_generics() {
    let t = def_item(
        "MyType",
        type_alias_kind(Type::Path(TypePath {
            path: rasto::ast::expressions::Path {
                segments: thin_vec![PathSegment {
                    ident: "Vec".to_string(),
                    args: Some(GenericArgs {
                        args: vec![GenericArg::Type(Type::from("T"))],
                    }),
                }],
            },
        }))
        .generic(TypeParam {
            ident: "T".to_string(),
            bounds: vec![],
        }),
    )
    .build();

    insta::assert_snapshot!(t.to_string(), @r###"
    type MyType<T> = Vec<T>;
    "###);
}

#[test]
fn test_union_with_generics() {
    let u = union_item("MyUnion")
        .generic(TypeParam {
            ident: "T".to_string(),
            bounds: vec![],
        })
        .field("my_field", Type::from("T"))
        .build();

    insta::assert_snapshot!(u.to_string(), @r"
    union MyUnion<T> {
        my_field: T,
    }
    ");
}
