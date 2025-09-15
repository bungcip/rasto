use crate::ast::item_struct::Field;
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// A `union` definition, such as `union MyUnion { f1: u32, f2: f32 }`.
    pub struct ItemUnion with generics {
        /// The fields of the union.
        pub fields: ThinVec<Field>,
    }
}
