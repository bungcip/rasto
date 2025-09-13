use rasto::ast::*;
use thin_vec::thin_vec;

#[test]
fn test_fn_with_attributes() {
    let item = Item::from(ItemFn {
        md: Some(Box::new(Md {
            attrs: thin_vec![
                Attribute::Outer(Meta::Path("test".to_string())),
                Attribute::Outer(Meta::List(MetaList {
                    path: "derive".to_string(),
                    metas: thin_vec![
                        Meta::Path("Debug".to_string()),
                        Meta::Path("Clone".to_string()),
                    ],
                })),
            ],
            leading_comments: thin_vec![],
            trailing_comments: thin_vec![],
        })),
        sig: Signature {
            ident: "my_func".to_string(),
            generics: Default::default(),
            inputs: thin_vec![],
            output: None,
        },
        block: Block {
            stmts: thin_vec![],
            leading_comments: thin_vec![],
            trailing_comments: thin_vec![],
        },
    });

    insta::assert_snapshot!(item.to_string());
}
