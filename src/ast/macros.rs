/// A macro for generating the `Display` implementation for an AST item.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_display_for_item {
    ($name:ident) => {
        impl ::std::fmt::Display for $name {
            /// Formats the item using the pretty-printer.
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut printer = $crate::pretty_printer::Printer::new(f);
                self.pretty_print(&mut printer)?;
                printer.finish()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{pretty, ast::{items::TestItem, visibility::Visibility, generics::GenericParams}};

    #[test]
    fn test_ast_item_macro() {
        let item = TestItem {
            vis: Visibility::Public,
            ident: "MyItem".into(),
            generics: GenericParams::new(),
            fields: vec!["field1".to_string(), "field2".to_string()],
            md: None,
        };
        insta::assert_snapshot!(pretty(&item));
    }
}

/// A helper macro for generating AST item structs.
#[macro_export]
macro_rules! ast_item_impl {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident as $variant:ident {
            $(
                $(#[$common_f_outer:meta])*
                $common_f_vis:vis $common_field:ident: $common_ty:ty
            ),*
        }
        {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            $(
                $(#[$common_f_outer])*
                $common_f_vis $common_field: $common_ty,
            )*
            $(
                $(#[$f_outer])*
                $f_vis $field: $ty
            ),*
        }
        $crate::impl_display_for_item!($name);

        impl From<$name> for $crate::ast::items::Item {
            fn from(item: $name) -> Self {
                $crate::ast::items::Item::$variant(item)
            }
        }
    }
}

/// A macro for generating AST item structs.
///
/// This macro reduces boilerplate by generating the struct definition with
/// common fields, as well as a `Display` implementation.
///
/// It supports two forms: one for items with generics and one for items without.
#[macro_export]
macro_rules! ast_item {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident as $variant:ident {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        ast_item_impl! {
            $(#[$outer])*
            $vis struct $name as $variant {
                /// The visibility of the item.
                pub vis: $crate::ast::visibility::Visibility,
                /// The name of the item.
                pub ident: $crate::ast::ident::Ident,
                /// Metadata about the item, including attributes and comments.
                pub md: Option<Box<$crate::ast::metadata::Md>>
            }
            {
                $(
                    $(#[$f_outer])*
                    $f_vis $field: $ty
                ),*
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident as $variant:ident without vis, ident, and md {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        ast_item_impl! {
            $(#[$outer])*
            $vis struct $name as $variant {}
            {
                $(
                    $(#[$f_outer])*
                    $f_vis $field: $ty
                ),*
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident as $variant:ident without ident {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        ast_item_impl! {
            $(#[$outer])*
            $vis struct $name as $variant {
                /// The visibility of the item.
                pub vis: $crate::ast::visibility::Visibility,
                /// Metadata about the item, including attributes and comments.
                pub md: Option<Box<$crate::ast::metadata::Md>>
            }
            {
                $(
                    $(#[$f_outer])*
                    $f_vis $field: $ty
                ),*
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident as $variant:ident without vis and ident {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        ast_item_impl! {
            $(#[$outer])*
            $vis struct $name as $variant {
                /// Metadata about the item, including attributes and comments.
                pub md: Option<Box<$crate::ast::metadata::Md>>
            }
            {
                $(
                    $(#[$f_outer])*
                    $f_vis $field: $ty
                ),*
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident as $variant:ident without vis {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        ast_item_impl! {
            $(#[$outer])*
            $vis struct $name as $variant {
                /// The name of the item.
                pub ident: $crate::ast::ident::Ident,
                /// Metadata about the item, including attributes and comments.
                pub md: Option<Box<$crate::ast::metadata::Md>>
            }
            {
                $(
                    $(#[$f_outer])*
                    $f_vis $field: $ty
                ),*
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident as $variant:ident with generics {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        ast_item_impl! {
            $(#[$outer])*
            $vis struct $name as $variant {
                /// The visibility of the item.
                pub vis: $crate::ast::visibility::Visibility,
                /// The name of the item.
                pub ident: $crate::ast::ident::Ident,
                /// The generic parameters of the item.
                pub generics: $crate::ast::generics::GenericParams,
                /// Metadata about the item, including attributes and comments.
                pub md: Option<Box<$crate::ast::metadata::Md>>
            }
            {
                $(
                    $(#[$f_outer])*
                    $f_vis $field: $ty
                ),*
            }
        }
    };
}
