use rasto::builder::*;
use rasto::pretty;

#[test]
fn test_file_display() {
    let file = file()
        .item(fn_def("my_function").build())
        .item(struct_def("MyStruct").build())
        .build();
    insta::assert_snapshot!(pretty(&file));
}
