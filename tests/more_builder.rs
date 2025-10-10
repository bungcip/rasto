//! Tests for various builders in `src/builder.rs`.

use rasto::builder::*;
use rasto::ast::*;

#[test]
fn test_const_def_builder() {
    let const_def = const_def("MY_CONST", type_().path("u32"), expr().lit(42))
        .vis(Visibility::Public)
        .comment(comment().doc(" A constant value."))
        .build();

    let expected_output = r#"
/// A constant value.
pub const MY_CONST: u32 = 42;"#;
    assert_eq!(const_def.to_string().trim(), expected_output.trim());
}

#[test]
fn test_type_alias_builder() {
    let type_alias = type_alias("MyType", type_().path("String"))
        .vis(Visibility::Public)
        .generic(generic_param().lifetime("a"))
        .comment(comment().doc(" A type alias."))
        .build();

    let expected_output = r#"
/// A type alias.
pub type MyType<'a> = String;"#;
    assert_eq!(type_alias.to_string().trim(), expected_output.trim());
}

#[test]
fn test_extern_block_builder() {
    let extern_block = extern_block_item()
        .unsafe_()
        .abi("C")
        .item(ExternalItem::Fn(
            fn_def("my_c_function")
                .input_typed("arg1", type_().path("i32"))
                .output(type_().path("i32"))
                .build(),
        ))
        .build();

    let expected_output = r#"unsafe extern "C" {
    fn my_c_function(arg1: i32) -> i32;
}"#;
    assert_eq!(extern_block.to_string().trim(), expected_output.trim());
}

#[test]
fn test_trait_builder() {
    let trait_def = trait_def("MyTrait")
        .vis(Visibility::Public)
        .generic(generic_param().ty("T"))
        .associated_type(associated_type("MyType").bound(type_().path("SomeTrait")))
        .item(trait_item_fn("my_func").input_typed("arg", type_().path("T")))
        .build();

    let expected_output = r#"
pub trait MyTrait<T> {
    type MyType: SomeTrait;
    fn my_func(arg: T);
}"#;
    assert_eq!(trait_def.to_string().trim(), expected_output.trim());
}

#[test]
fn test_impl_builder() {
    let impl_block = impl_block(type_().path("MyStruct"))
        .trait_(type_().path("MyTrait"))
        .item(
            fn_def("my_func")
                .input_typed("arg", type_().path("i32"))
                .block(block())
                .build(),
        )
        .build();

    let expected_output = r#"
impl MyTrait for MyStruct {
    fn my_func(arg: i32) {}
}"#;
    assert_eq!(impl_block.to_string().trim(), expected_output.trim());
}

#[test]
fn test_enum_builder() {
    let enum_def = enum_def("MyEnum")
        .vis(Visibility::Public)
        .variant("Variant1")
        .variant("Variant2")
        .build();

    let expected_output = r#"
pub enum MyEnum {
    Variant1,
    Variant2,
}"#;
    assert_eq!(enum_def.to_string().trim(), expected_output.trim());
}

#[test]
fn test_struct_builder() {
    let struct_def = struct_def("MyStruct")
        .vis(Visibility::Public)
        .field("field1", type_().path("u32"))
        .field("field2", type_().path("String"))
        .build();

    let expected_output = r#"
pub struct MyStruct {
    field1: u32,
    field2: String,
}"#;
    assert_eq!(struct_def.to_string().trim(), expected_output.trim());
}

#[test]
fn test_static_item_builder() {
    let static_item = static_item("MY_STATIC", type_().path("u32"), expr().lit(100))
        .vis(Visibility::Public)
        .mutable()
        .build();

    let expected_output = r#"pub static mut MY_STATIC: u32 = 100;"#;
    assert_eq!(static_item.to_string().trim(), expected_output.trim());
}

#[test]
fn test_union_builder() {
    let union_def = union_item("MyUnion")
        .vis(Visibility::Public)
        .generic(generic_param().ty("T"))
        .field("f1", type_().path("u32"))
        .field("f2", type_().path("f32"))
        .build();

    let expected_output = r#"
pub union MyUnion<T> {
    f1: u32,
    f2: f32,
}"#;
    assert_eq!(union_def.to_string().trim(), expected_output.trim());
}