//! Defines macros for generating AST item structs.

/// A macro for generating AST item structs.
///
/// This macro reduces boilerplate by generating the struct definition with
/// common fields, as well as a `Display` implementation.
///
/// It supports two forms: one for items with generics and one for items without.
macro_rules! ast_item {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            /// The visibility of the item.
            pub vis: $crate::ast::visibility::Visibility,
            /// The name of the item.
            pub ident: String,
            /// Metadata about the item, including attributes and comments.
            pub md: Option<Box<$crate::ast::metadata::Md>>,
            $(
                $(#[$f_outer])*
                $f_vis $field: $ty
            ),*
        }

        impl ::std::fmt::Display for $name {
            /// Formats the item using the pretty-printer.
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut printer = $crate::pretty_printer::Printer::new(f);
                self.pretty_print(&mut printer)?;
                printer.finish()
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident without vis, ident, and md {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            $(
                $(#[$f_outer])*
                $f_vis $field: $ty
            ),*
        }

        impl ::std::fmt::Display for $name {
            /// Formats the item using the pretty-printer.
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut printer = $crate::pretty_printer::Printer::new(f);
                self.pretty_print(&mut printer)?;
                printer.finish()
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident without ident {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            /// The visibility of the item.
            pub vis: $crate::ast::visibility::Visibility,
            /// Metadata about the item, including attributes and comments.
            pub md: Option<Box<$crate::ast::metadata::Md>>,
            $(
                $(#[$f_outer])*
                $f_vis $field: $ty
            ),*
        }

        impl ::std::fmt::Display for $name {
            /// Formats the item using the pretty-printer.
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut printer = $crate::pretty_printer::Printer::new(f);
                self.pretty_print(&mut printer)?;
                printer.finish()
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident without vis and ident {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            /// Metadata about the item, including attributes and comments.
            pub md: Option<Box<$crate::ast::metadata::Md>>,
            $(
                $(#[$f_outer])*
                $f_vis $field: $ty
            ),*
        }

        impl ::std::fmt::Display for $name {
            /// Formats the item using the pretty-printer.
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut printer = $crate::pretty_printer::Printer::new(f);
                self.pretty_print(&mut printer)?;
                printer.finish()
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident without vis {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            /// The name of the item.
            pub ident: String,
            /// Metadata about the item, including attributes and comments.
            pub md: Option<Box<$crate::ast::metadata::Md>>,
            $(
                $(#[$f_outer])*
                $f_vis $field: $ty
            ),*
        }

        impl ::std::fmt::Display for $name {
            /// Formats the item using the pretty-printer.
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut printer = $crate::pretty_printer::Printer::new(f);
                self.pretty_print(&mut printer)?;
                printer.finish()
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident with generics {
            $(
                $(#[$f_outer:meta])*
                $f_vis:vis $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            /// The visibility of the item.
            pub vis: $crate::ast::visibility::Visibility,
            /// The name of the item.
            pub ident: String,
            /// The generic parameters of the item.
            pub generics: $crate::ast::generics::GenericParams,
            /// Metadata about the item, including attributes and comments.
            pub md: Option<Box<$crate::ast::metadata::Md>>,
            $(
                $(#[$f_outer])*
                $f_vis $field: $ty
            ),*
        }

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
