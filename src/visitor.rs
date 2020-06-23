use syn::{ItemMod, ItemFn, ItemStruct};

// Return default implementations for all.
pub trait Visitor {

    fn visit_mod<'a>(&self, item: &'a mut ItemMod) -> &'a mut ItemMod {
        item
    }
    fn visit_fn<'a>(&self, item: &'a mut ItemFn) -> &'a mut ItemFn {
        item
    }

    fn visit_struct<'a>(&self, item: &'a mut ItemStruct) -> &'a mut ItemStruct {
        item
    }
}
