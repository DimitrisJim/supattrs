/*
Const(ItemConst),
Enum(ItemEnum),
ExternCrate(ItemExternCrate),
Fn(ItemFn),
ForeignMod(ItemForeignMod),
Impl(ItemImpl),
Macro(ItemMacro),
Macro2(ItemMacro2),
Mod(ItemMod),
Static(ItemStatic),
Struct(ItemStruct),
Trait(ItemTrait),
TraitAlias(ItemTraitAlias),
Type(ItemType),
Union(ItemUnion),
Use(ItemUse),
Verbatim(TokenStream),
*/
use proc_macro::{TokenStream};
use quote::quote;
use crate::visitor::Visitor;
use syn::{ItemFn, ItemMod, ItemStruct};

type TS = TokenStream;

pub trait Visitable {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS;
}

// Note: Should visit constituent items.
// note: κατι μου λεει οτι καλυτερα αυτο να επιστρεφει Self
// note: παρά ΤS. We want user to be able to use quote.
impl Visitable for ItemMod {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_mod(self);
        return (quote! {
            #res
        }).into()
    }
}

// Note: Should visit constituent items.
impl Visitable for ItemStruct {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_struct(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemFn {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_fn(self);
        return (quote! {
            #[test]
            #res
        }).into()
    }
}