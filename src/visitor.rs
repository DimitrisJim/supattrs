use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemMod, ItemFn, ItemStruct, ItemConst, ItemEnum, ItemExternCrate, ItemForeignMod,
    ItemImpl, ItemMacro, ItemMacro2, ItemStatic, ItemTrait, ItemTraitAlias, ItemType,
    ItemUnion, ItemUse
};


// todo: macro_rules?
// Default implementations for all, simply return their items.
pub trait Visitor {
    fn visit_mod(&self, item: &mut ItemMod) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_fn(&self, item: &mut ItemFn) -> TokenStream {
        (quote! {
            #[test]
            #item
        }).into()
    }

    fn visit_struct(&self, item: &mut ItemStruct) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_const(&self, item: &mut ItemConst) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_enum(&self, item: &mut ItemEnum) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_externcrate(&self, item: &mut ItemExternCrate) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_foreignmod(&self, item: &mut ItemForeignMod) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_impl(&self, item: &mut ItemImpl) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_macro(&self, item: &mut ItemMacro) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_macro2(&self, item: &mut ItemMacro2) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_static(&self, item: &mut ItemStatic) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_trait(&self, item:  &mut ItemTrait) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_traitalias(&self, item: &mut ItemTraitAlias) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_type(&self, item: &mut ItemType) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_union(&self, item: &mut ItemUnion) -> TokenStream {
        (quote! {
            #item
        }).into()
    }

    fn visit_use(&self, item: &mut ItemUse) -> TokenStream {
        (quote! {
            #item
        }).into()
    }
}
