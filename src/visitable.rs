#![allow(clippy::ptr_arg)]
/// Each implementation of Visitable for the Items should apply the macro
/// (defined by visitor) not only to the item it represents but also to items
/// it might contain. This way, if a user specified, for example, visit_fn,
/// and then applied the macro to a `mod`, the accept would traverse through
/// the items contained in the `mod` and apply the macro to each `fn` it finds.
///
/// Note: It might be a good idea to create two visitors; one that recurses through
/// Note: the items and one that does not.
/// Note: don't want to use [syn::NestedMeta]; it seems like that is an internal detail.
///       AttributeArgs on the other hand is the exported type to use.
use proc_macro::{TokenStream};
use crate::visitor::Visitor;
use quote::quote;
use syn::{
    ItemMod, ItemFn, ItemStruct, ItemConst, ItemEnum, ItemExternCrate,
    ItemForeignMod, ItemImpl, ItemMacro, ItemMacro2, ItemStatic, ItemTrait,
    ItemTraitAlias, ItemType, ItemUnion, ItemUse, Item, AttributeArgs,
    parse
};

type TS = TokenStream;

/// Trait to accept visitors for syn::Item's
trait Visitable {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS;
}

// Note: Items are contained in `Some(contents.1)` as a Vec<Item> where all items are allowed.
impl Visitable for ItemMod {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        // note: (sad) I need to parse again in order to walk the updated mod
        let mut module: ItemMod = parse(visitor.visit_mod(self, attrs))
            .expect("Invalid module returned from visit_mod.");
        // Grab content, its a Vec<item>
        let (_, content) = match &mut module.content {
            Some(content) => content,
            None => {
                panic!("Cannot apply macro to mod declaration.");
            }
        };

        // note (impl): map might be of help here.
        // note (impl): post and pre visit?
        let mut new_contents: Vec<Item> = Vec::new();
        for subitem in content.iter_mut(){
            // note: if we want all items applied to see the same attrs
            // note: we necessarily need to clone here.
            // note: (sad) I need to parse again.
            let res = parse(dispatch(subitem, &attrs.clone(), visitor))
                .expect("Invalid item returned from visit implementation.");
            new_contents.push(res);
        }
        // drop new_contents into content
        content.clear();
        content.append(&mut new_contents);
        // note: test test test:: does it contain new content?
        (quote! {
            #module
        }).into()
    }
}

// Note: Items are contained in `items` as a Vec<ForeignItem{ItemName}> where ItemName can be one of:
//     ForeignItemFn
//     ForeignItemStatic
//     ForeignItemType
//     ForeignItemMacro
//     TokenStream  (unused)
//  How can we change them into Items?
impl Visitable for ItemForeignMod {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        let fmod: ItemForeignMod = parse(visitor.visit_foreignmod(self, attrs))
            .expect("Invalid TokenStream returned from visit_foreignmod.");
        (quote! {
            #fmod
        }).into()
    }
}

// Note: Items are contained in `items` as a Vec<ImplItem{ItemName}> where ItemName can be one of:
//     ImplItemConst
//     ImplItemMethod
//     ImplItemType
//     ImplItemMacro
//     TokenStream  (unused)
//  How can we change them into Items?
impl Visitable for ItemImpl {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        let item_impl: ItemImpl = parse(visitor.visit_impl(self, attrs))
            .expect("Invalid TokenStream returned from visit_impl.");
        (quote! {
            #item_impl
        }).into()
    }
}

// Note: Items are contained in `items` as a Vec<TraitItem{ItemName}> where ItemName can be one of:
//     TraitItemConst
//     TraitItemMethod
//     TraitItemType
//     TraitItemMacro
//     TokenStream  (unused)
//  How can we change them into Items?
impl Visitable for ItemTrait {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        let trait_impl: ItemTrait = parse(visitor.visit_trait(self, attrs))
            .expect("Invalid TokenStream returned from visit_trait.");
        (quote! {
            #trait_impl
        }).into()
    }
}

//--- The following do not have items contained inside of them. ---//

/// Visit struct item and return altered TokenStream
impl Visitable for ItemStruct {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_struct(self, attrs)
    }
}

/// Visit fn item and return altered TokenStream
impl Visitable for ItemFn {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_fn(self, attrs)
    }
}

/// Visit const item and return altered TokenStream
impl Visitable for ItemConst {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_const(self, attrs)
    }
}

/// Visit enum item and return altered TokenStream
impl Visitable for ItemEnum {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_enum(self, attrs)
    }
}

/// Visit extern item and return altered TokenStream
impl Visitable for ItemExternCrate {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_externcrate(self, attrs)
    }
}

/// Visit macro (old) item and return altered TokenStream
impl Visitable for ItemMacro {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_macro(self, attrs)
    }
}

/// Visit macro (new) item and return altered TokenStream
impl Visitable for ItemMacro2 {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_macro2(self, attrs)
    }
}

/// Visit static item and return altered TokenStream
impl Visitable for ItemStatic {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_static(self, attrs)
    }
}

/// Visit type item and return altered TokenStream
impl Visitable for ItemType {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_type(self, attrs)
    }
}

/// Visit union item and return altered TokenStream
impl Visitable for ItemUnion {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_union(self, attrs)
    }
}

/// Visit use item and return altered TokenStream
impl Visitable for ItemUse {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_use(self, attrs)
    }
}

// todo: unstable feature, how do we feature bound things?
/// Visit trait alias item and return altered TokenStream
impl Visitable for ItemTraitAlias {
    fn accept<V: Visitor>(&mut self, attrs: &AttributeArgs, visitor: &V) -> TS {
        visitor.visit_traitalias(self, attrs)
    }
}


/// Dispatch: Call appropriate accept based on the item passed.
///
/// ##### Notes:
///
/// No need to move values here. We accept as mut references to allow use to mutate them
/// in his visit_* function.
///
pub(crate) fn dispatch<V>(item: &mut Item, attrs: &AttributeArgs, transformer: &V) -> TS
    where V: Visitor
{
    match item {
        Item::Fn(ref mut func) => {
            func.accept(attrs, transformer)
        }
        Item::Struct(ref mut structure) => {
            structure.accept(attrs, transformer)
        }
        Item::Mod(ref mut module) => {
            module.accept(attrs, transformer)
        }
        Item::Const(ref mut const_item) => {
            const_item.accept(attrs, transformer)
        }
        Item::Enum(ref mut enum_item) => {
            enum_item.accept(attrs, transformer)
        }
        Item::ExternCrate(ref mut externcrate_item) => {
            externcrate_item.accept(attrs, transformer)
        }
        Item::ForeignMod(ref mut foreignmod_item) => {
            foreignmod_item.accept(attrs, transformer)
        }
        Item::Impl(ref mut impl_item) => {
            impl_item.accept(attrs, transformer)
        }
        // macro_rules macro def
        Item::Macro(ref mut macro_item) => {
            macro_item.accept(attrs, transformer)
        }
        Item::Macro2(ref mut macro2_item) => {
            macro2_item.accept(attrs, transformer)
        }
        Item::Static(ref mut static_item) => {
            static_item.accept(attrs, transformer)
        }
        Item::Trait(ref mut trait_item) => {
            trait_item.accept(attrs, transformer)
        }
        Item::TraitAlias(ref mut traitalias_item) => {
            traitalias_item.accept(attrs, transformer)
        }
        Item::Type(ref mut type_item) => {
            type_item.accept(attrs, transformer)
        }
        Item::Union(ref mut union_item) => {
            union_item.accept(attrs, transformer)
        }
        Item::Use(ref mut use_item) => {
            use_item.accept(attrs, transformer)
        }
        // note: Does not handle Verbatim and a private __Nonexhaustive.
        _ => {
            // todo, be more descriptive.
            panic!("Item must be mod or fn.");
        }
    }
}