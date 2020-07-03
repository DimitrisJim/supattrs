// don't want to use [syn::NestedMeta]; it seems like that is an internal detail.
// AttributeArgs on the other hand is the exported type to use.
#![allow(clippy::ptr_arg)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemMod, ItemFn, ItemStruct, ItemConst, ItemEnum, ItemExternCrate, ItemForeignMod,
    ItemImpl, ItemMacro, ItemMacro2, ItemStatic, ItemTrait, ItemTraitAlias, ItemType,
    ItemUnion, ItemUse,
    AttributeArgs,
};


// todo: macro_rules?
// todo: strict visitor -- i.e visitor that errors when applied wrong place
// note: attrs as mut reference, needed?
// Default implementations for all, simply return their items.
pub trait Visitor {
    fn visit_mod(&self, item: &mut ItemMod, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_fn(&self, item: &mut ItemFn, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #[test]
            #item
        }).into()
    }

    fn visit_struct(&self, item: &mut ItemStruct, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_const(&self, item: &mut ItemConst, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_enum(&self, item: &mut ItemEnum, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_externcrate(&self, item: &mut ItemExternCrate, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_foreignmod(&self, item: &mut ItemForeignMod, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_impl(&self, item: &mut ItemImpl, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_macro(&self, item: &mut ItemMacro, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_macro2(&self, item: &mut ItemMacro2, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_static(&self, item: &mut ItemStatic, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_trait(&self, item:  &mut ItemTrait, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_traitalias(&self, item: &mut ItemTraitAlias, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_type(&self, item: &mut ItemType, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_union(&self, item: &mut ItemUnion, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }

    fn visit_use(&self, item: &mut ItemUse, attrs: &AttributeArgs) -> TokenStream {
        // note: intentional; ignore anything passed to macro.
        let _attrs = attrs;
        (quote! {
            #item
        }).into()
    }
}
