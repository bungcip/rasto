use rasto::ast::*;

#[test]
fn test_fn_with_attributes() {
    let item = Item::from(ItemFn {
        attrs: vec![
            Attribute::Outer(Meta::Path("test".to_string())),
            Attribute::Outer(Meta::List(MetaList {
                path: "derive".to_string(),
                metas: vec![
                    Meta::Path("Debug".to_string()),
                    Meta::Path("Clone".to_string()),
                ],
            })),
        ],
        leading_comments: vec![],
        sig: Signature {
            ident: "my_func".to_string(),
            generics: Default::default(),
            inputs: vec![],
            output: None,
        },
        block: Block {
            stmts: vec![],
            leading_comments: vec![],
            trailing_comments: vec![],
        },
        trailing_comments: vec![],
    });

    insta::assert_snapshot!(item.to_string());
}
