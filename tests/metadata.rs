use rasto::ast::{Attribute, Comment, Md, Meta};
use thin_vec::thin_vec;

#[test]
fn test_metadata_comments() {
    let md = Md {
        comments: thin_vec![Comment::Doc("A doc comment.".to_string())],
        attrs: thin_vec![],
        trailing_comments: thin_vec![],
    };
    assert_eq!(md.comments.len(), 1);
    assert_eq!(md.comments[0], Comment::Doc("A doc comment.".to_string()));
}

#[test]
fn test_metadata_attrs() {
    let md = Md {
        comments: thin_vec![],
        attrs: thin_vec![Attribute::Outer(Meta::Path("my_attr".into()))],
        trailing_comments: thin_vec![],
    };
    assert_eq!(md.attrs.len(), 1);
    assert_eq!(md.attrs[0], Attribute::Outer(Meta::Path("my_attr".into())));
}

#[test]
fn test_metadata_trailing_comments() {
    let md = Md {
        comments: thin_vec![],
        attrs: thin_vec![],
        trailing_comments: thin_vec![Comment::Line("A trailing comment.".to_string())],
    };
    assert_eq!(md.trailing_comments.len(), 1);
    assert_eq!(
        md.trailing_comments[0],
        Comment::Line("A trailing comment.".to_string())
    );
}

#[test]
fn test_md_builder() {
    let md = rasto::ast::MdBuilder::new()
        .attr(Attribute::Outer(Meta::Path("foo".into())))
        .comment(Comment::Line(" a comment".into()))
        .trailing_comment(Comment::Line(" a trailing comment".into()))
        .build();

    assert_eq!(
        md,
        Md {
            attrs: thin_vec![Attribute::Outer(Meta::Path("foo".into()))],
            comments: thin_vec![Comment::Line(" a comment".into())],
            trailing_comments: thin_vec![Comment::Line(" a trailing comment".into())],
        }
    );
}
