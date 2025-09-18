use rasto::ast::{
    items::{Item, TraitItem},
    *,
};
use rasto::builder::*;
use thin_vec::thin_vec;

#[test]
fn test_item_from_fn_def() {
    let item: Item = fn_def("my_fn").build().into();
    assert!(matches!(item, Item::Fn(_)));
}

#[test]
fn test_item_from_struct_def() {
    let item: Item = struct_def("MyStruct").build().into();
    assert!(matches!(item, Item::Struct(_)));
}

#[test]
fn test_item_from_enum_def() {
    let item: Item = enum_def("MyEnum").build().into();
    assert!(matches!(item, Item::Enum(_)));
}

#[test]
fn test_item_from_impl_block() {
    let item: Item = impl_block("MyType").build().into();
    assert!(matches!(item, Item::Impl(_)));
}

#[test]
fn test_item_from_trait_def() {
    let item: Item = trait_def("MyTrait").build().into();
    assert!(matches!(item, Item::Trait(_)));
}

#[test]
fn test_item_from_const_def() {
    let item: Item = const_def("MY_CONST", "u8", expr().lit(5)).build().into();
    assert!(matches!(item, Item::Const(_)));
}

#[test]
fn test_item_from_extern_crate_item() {
    let item: Item = extern_crate_item("serde").build().into();
    assert!(matches!(item, Item::ExternCrate(_)));
}

#[test]
fn test_item_from_foreign_mod_item() {
    let item: Item = foreign_mod_item("C").build().into();
    assert!(matches!(item, Item::ForeignMod(_)));
}

#[test]
fn test_item_from_macro_item() {
    let item: Item = macro_item(expr().macro_call("my_macro", Delimiter::Parenthesis, thin_vec![]))
        .build()
        .into();
    assert!(matches!(item, Item::Macro(_)));
}

#[test]
fn test_item_from_mod_item() {
    let item: Item = mod_item("my_module").build().into();
    assert!(matches!(item, Item::Mod(_)));
}

#[test]
fn test_item_from_trait_alias_item() {
    let item: Item = trait_alias_item("MyAlias", thin_vec![]).build().into();
    assert!(matches!(item, Item::TraitAlias(_)));
}

#[test]
fn test_item_from_union_item() {
    let item: Item = union_item("MyUnion").build().into();
    assert!(matches!(item, Item::Union(_)));
}

#[test]
fn test_item_from_use_item() {
    let item: Item = use_item("std::collections::HashMap").build().into();
    assert!(matches!(item, Item::Use(_)));
}

#[test]
fn test_item_from_asm_item() {
    let item: Item = asm_item(LitStr::new("nop")).build().into();
    assert!(matches!(item, Item::Asm(_)));
}

#[test]
fn test_impl_item_from_associated_const() {
    let impl_item: ImplItem = associated_const("MY_CONST", "u8").build().into();
    assert!(matches!(impl_item, ImplItem::Const(_)));
}

#[test]
fn test_impl_item_from_associated_type() {
    let impl_item: ImplItem = associated_type("MyType").build().into();
    assert!(matches!(impl_item, ImplItem::Type(_)));
}

#[test]
fn test_trait_item_from_trait_item_fn() {
    let trait_item: TraitItem = trait_item_fn("my_fn").into();
    assert!(matches!(trait_item, TraitItem::Fn(_)));
}

#[test]
fn test_trait_item_from_associated_const() {
    let trait_item: TraitItem = associated_const("MY_CONST", "u8").build().into();
    assert!(matches!(trait_item, TraitItem::Const(_)));
}
