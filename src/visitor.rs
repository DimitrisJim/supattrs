use syn::{
    ItemMod, ItemFn, ItemStruct, ItemConst, ItemEnum, ItemExternCrate, ItemForeignMod,
    ItemImpl, ItemMacro, ItemMacro2, ItemStatic, ItemTrait, ItemTraitAlias, ItemType,
    ItemUnion, ItemUse
};


// todo: macro_rules?
// Default implementations for all, simply return their items.
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

    fn visit_const<'a>(&self, item: &'a mut ItemConst) -> &'a mut ItemConst {
        item
    }

    fn visit_enum<'a>(&self, item: &'a mut ItemEnum) -> &'a mut ItemEnum {
        item
    }

    fn visit_externcrate<'a>(&self, item: &'a mut ItemExternCrate) -> &'a mut ItemExternCrate {
        item
    }

    fn visit_foreignmod<'a>(&self, item: &'a mut ItemForeignMod) -> &'a mut ItemForeignMod {
        item
    }

    fn visit_impl<'a>(&self, item: &'a mut ItemImpl) -> &'a mut ItemImpl {
        item
    }

    fn visit_macro<'a>(&self, item: &'a mut ItemMacro) -> &'a mut ItemMacro {
        item
    }

    fn visit_macro2<'a>(&self, item: &'a mut ItemMacro2) -> &'a mut ItemMacro2 {
        println!("{}", "NEW MACROS!");
        item
    }

    fn visit_static<'a>(&self, item: &'a mut ItemStatic) -> &'a mut ItemStatic {
        item
    }

    fn visit_trait<'a>(&self, item: &'a mut ItemTrait) -> &'a mut ItemTrait {
        item
    }

    fn visit_traitalias<'a>(&self, item: &'a mut ItemTraitAlias) -> &'a mut ItemTraitAlias {
        item
    }

    fn visit_type<'a>(&self, item: &'a mut ItemType) -> &'a mut ItemType {
        item
    }

    fn visit_union<'a>(&self, item: &'a mut ItemUnion) -> &'a mut ItemUnion {
        item
    }

    fn visit_use<'a>(&self, item: &'a mut ItemUse) -> &'a mut ItemUse {
        item
    }
}
