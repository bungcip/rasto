use rasto::ast::{Attribute, Comment, Md, Meta};
use thin_vec::thin_vec;

#[test]
fn test_metadata() {
    let md = Md {
        comments: thin_vec![Comment::Doc("A doc comment.".to_string().into())],
        attrs: thin_vec![Attribute::Outer(Meta::Path("my_attr".to_string().into()))],
        trailing_comments: thin_vec![Comment::Line("A trailing comment.".to_string().into())],
    };

    assert_eq!(md.comments.len(), 1);
    assert_eq!(md.attrs.len(), 1);
    assert_eq!(md.trailing_comments.len(), 1);

    assert_eq!(md.comments[0], Comment::Doc("A doc comment.".to_string().into()));
    assert_eq!(
        md.attrs[0],
        Attribute::Outer(Meta::Path("my_attr".to_string().into()))
    );
    assert_eq!(
        md.trailing_comments[0],
        Comment::Line("A trailing comment.".to_string().into())
    );
}
