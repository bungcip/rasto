use rasto::ast::Visibility;
use rasto::builder::*;

#[test]
fn test_visibility() {
    let pub_fn = fn_def("my_public_fn").vis(Visibility::Public).build();
    insta::assert_snapshot!(&pub_fn, @"pub fn my_public_fn() {}");

    let crate_struct = struct_def("MyCrateStruct")
        .vis(Visibility::Crate)
        .field("x", "i32")
        .build();
    insta::assert_snapshot!(&crate_struct, @r"
    pub(crate) struct MyCrateStruct {
        x: i32,
    }
    ");

    let default_enum = enum_def("MyDefaultEnum").variant("A").build();
    insta::assert_snapshot!(&default_enum, @r"
    enum MyDefaultEnum {
        A,
    }
    ");

    let pub_union = union_item("MyPublicUnion")
        .vis(Visibility::Public)
        .field("f1", "u32")
        .build();
    insta::assert_snapshot!(&pub_union, @r"
    pub union MyPublicUnion {
        f1: u32,
    }
    ");

    let crate_mod = empty_mod_item("my_crate_mod").vis(Visibility::Crate).build();
    insta::assert_snapshot!(&crate_mod, @"pub(crate) mod my_crate_mod;");

    let pub_use = use_item("std::collections::HashMap")
        .vis(Visibility::Public)
        .build();
    insta::assert_snapshot!(&pub_use, @"pub use std::collections::HashMap;");

    let crate_trait = trait_def("MyCrateTrait").vis(Visibility::Crate).build();
    insta::assert_snapshot!(&crate_trait, @"pub(crate) trait MyCrateTrait {}");

    let pub_const = const_def("MY_CONST", "u8", expr().lit(5))
        .vis(Visibility::Public)
        .build();
    insta::assert_snapshot!(&pub_const, @"pub const MY_CONST: u8 = 5;");

    let default_vis = fn_def("my_default_fn").build();
    insta::assert_snapshot!(&default_vis, @"fn my_default_fn() {}");
}
