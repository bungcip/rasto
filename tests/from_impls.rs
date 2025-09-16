use rasto::ast::{
    items::{Item, TraitItem},
    *,
};
use rasto::builder::*;
use thin_vec::thin_vec;

#[test]
fn test_from_impls() {
    let item: Item = fn_def("my_fn").build().into();
    assert!(matches!(item, Item::Fn(_)));

    let item: Item = struct_def("MyStruct").build().into();
    assert!(matches!(item, Item::Struct(_)));

    let item: Item = enum_def("MyEnum").build().into();
    assert!(matches!(item, Item::Enum(_)));

    let item: Item = impl_block("MyType").build().into();
    assert!(matches!(item, Item::Impl(_)));

    let item: Item = trait_def("MyTrait").build().into();
    assert!(matches!(item, Item::Trait(_)));

    let item: Item = def_item("MY_CONST", const_kind("u8", expr().lit(5)))
        .build()
        .into();
    assert!(matches!(item, Item::Def(_)));

    let item: Item = extern_crate_item("serde").build().into();
    assert!(matches!(item, Item::ExternCrate(_)));

    let item: Item = foreign_mod_item("C").build().into();
    assert!(matches!(item, Item::ForeignMod(_)));

    let item: Item = macro_item(expr().macro_call("my_macro", Delimiter::Parenthesis, thin_vec![]))
        .build()
        .into();
    assert!(matches!(item, Item::Macro(_)));

    let item: Item = mod_item("my_module").build().into();
    assert!(matches!(item, Item::Mod(_)));

    let item: Item = trait_alias_item("MyAlias", thin_vec![]).build().into();
    assert!(matches!(item, Item::TraitAlias(_)));

    let item: Item = union_item("MyUnion").build().into();
    assert!(matches!(item, Item::Union(_)));

    let item: Item = use_item("std::collections::HashMap").build().into();
    assert!(matches!(item, Item::Use(_)));

    let item: Item = asm_item(LitStr::new("nop")).build().into();
    assert!(matches!(item, Item::Asm(_)));

    let impl_item: ImplItem = associated_const("MY_CONST", "u8").build().into();
    assert!(matches!(impl_item, ImplItem::Const(_)));

    let impl_item: ImplItem = associated_type("MyType").build().into();
    assert!(matches!(impl_item, ImplItem::Type(_)));

    let trait_item: TraitItem = trait_item_fn("my_fn").into();
    assert!(matches!(trait_item, TraitItem::Fn(_)));

    let trait_item: TraitItem = associated_const("MY_CONST", "u8").build().into();
    assert!(matches!(trait_item, TraitItem::Const(_)));
}
