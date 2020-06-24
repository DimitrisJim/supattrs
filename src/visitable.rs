use proc_macro::{TokenStream};
use quote::quote;
use crate::visitor::Visitor;
use syn::{
    ItemMod, ItemFn, ItemStruct, ItemConst, ItemEnum, ItemExternCrate, ItemForeignMod,
    ItemImpl, ItemMacro, ItemMacro2, ItemStatic, ItemTrait, ItemTraitAlias, ItemType,
    ItemUnion, ItemUse
};

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

impl Visitable for ItemConst {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_const(self);
        return (quote! {
            #res
        }).into()
    }
}

impl Visitable for ItemEnum {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_enum(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemExternCrate {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_externcrate(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemForeignMod {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_foreignmod(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemImpl {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_impl(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemMacro {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_macro(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemMacro2 {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_macro2(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemStatic {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_static(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemTrait {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_trait(self);
        return (quote! {
            #res
        }).into()
    }
}


// todo: unstable feature, how do we feature bound things?
impl Visitable for ItemTraitAlias {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_traitalias(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemType {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_type(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemUnion {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_union(self);
        return (quote! {
            #res
        }).into()
    }
}


impl Visitable for ItemUse {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_use(self);
        return (quote! {
            #res
        }).into()
    }
}