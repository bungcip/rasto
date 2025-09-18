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



use rasto::ast::Comment;

#[test]
fn test_file_with_comments_and_attributes() {
    let file = file()
        .comment(Comment::Line(" This is a file-level comment.".into()))
        .attr(
            attr()
                .inner()
                .meta(meta().path("allow(dead_code)")),
        )
        .item(fn_def("my_function").build())
        .build();
    insta::assert_snapshot!(pretty(&file));
}
