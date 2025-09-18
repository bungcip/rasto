use rasto::{PrettyPrinter, Printer, ast_item, ast_item_impl, pretty};

ast_item! {
    pub struct MyMacroItem with generics {
        pub fields: Vec<String>,
    }
}

impl PrettyPrinter for MyMacroItem {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> std::fmt::Result {
        printer.string("MyMacroItem");
        Ok(())
    }
}

#[test]
fn test_ast_item_macro() {
    let item = MyMacroItem {
        vis: rasto::ast::Visibility::Public,
        ident: "MyItem".to_string(),
        generics: rasto::ast::generics::GenericParams::new(),
        fields: vec!["field1".to_string(), "field2".to_string()],
        md: None,
    };
    insta::assert_snapshot!(pretty(&item));
}
