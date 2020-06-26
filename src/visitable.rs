use proc_macro::{TokenStream};
use crate::visitor::Visitor;
use syn::{
    ItemMod, ItemFn, ItemStruct, ItemConst, ItemEnum, ItemExternCrate, ItemForeignMod,
    ItemImpl, ItemMacro, ItemMacro2, ItemStatic, ItemTrait, ItemTraitAlias, ItemType,
    ItemUnion, ItemUse, Item
};

type TS = TokenStream;

/// Each implementation of Visitable for the Items should apply the macro
/// (defined by visitor) not only to the item it represents but also to items
/// it might contain. This way, if a user specified, for example, visit_fn,
/// and then applied the macro to a `mod`, the accept would traverse through
/// the items contained in the `mod` and apply the macro to each `fn` it finds.
///
/// Note: It might be a good idea to create two visitors; one that recurses through
/// note: the items and one that does not.
pub trait Visitable {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS;
}

// Visit the module; calls visit_mod and then goes through its
// items calling visit_* for each found.
impl Visitable for ItemMod {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        // Alright, pass on to visit mod and then visit items.
        // todo: do we need to clone?
        let mut module = visitor.visit_mod(&mut self.clone());
        // Grab content, its a Vec<item>
        let (_, content) = match &mut self.content {
            Some(content) => content,
            None => {
                panic!("Cannot apply macro to mod declaration.");
            }
        };
        // Handle fn application.
        for subitem in content.iter_mut(){
            dispatch(subitem, visitor);
        }
        module
    }
}

// Note: Should visit constituent items.
impl Visitable for ItemStruct {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_struct(self);
        res
    }
}


impl Visitable for ItemFn {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_fn(self);
        res
    }
}

impl Visitable for ItemConst {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_const(self);
        res
    }
}

impl Visitable for ItemEnum {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_enum(self);
        res
    }
}


impl Visitable for ItemExternCrate {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_externcrate(self);
        res
    }
}


impl Visitable for ItemForeignMod {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_foreignmod(self);
        res
    }
}


impl Visitable for ItemImpl {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_impl(self);
        res
    }
}


impl Visitable for ItemMacro {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_macro(self);
        res
    }
}


impl Visitable for ItemMacro2 {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_macro2(self);
        res
    }
}


impl Visitable for ItemStatic {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_static(self);
        res
    }
}


impl Visitable for ItemTrait {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_trait(self);
        res
    }
}


// todo: unstable feature, how do we feature bound things?
impl Visitable for ItemTraitAlias {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_traitalias(self);
        res
    }
}


impl Visitable for ItemType {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_type(self);
        res
    }
}


impl Visitable for ItemUnion {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_union(self);
        res
    }
}


impl Visitable for ItemUse {
    fn accept<V: Visitor>(&mut self, visitor: &V) -> TS {
        let res = visitor.visit_use(self);
        res
    }
}


pub fn dispatch<V: Visitor>(item: &mut Item, transformer: &V) -> TS {
    match item {
        Item::Fn(ref mut func) => {
            func.accept(transformer)
        }
        Item::Struct(ref mut structure) => {
            structure.accept(transformer)
        }
        Item::Mod(ref mut module) => {
            module.accept(transformer)
        }
        Item::Const(ref mut const_item) => {
            const_item.accept(transformer)
        }
        Item::Enum(ref mut enum_item) => {
            enum_item.accept(transformer)
        }
        Item::ExternCrate(ref mut externcrate_item) => {
            externcrate_item.accept(transformer)
        }
        Item::ForeignMod(ref mut foreignmod_item) => {
            foreignmod_item.accept(transformer)
        }
        Item::Impl(ref mut impl_item) => {
            impl_item.accept(transformer)
        }
        // macro_rules macro def
        Item::Macro(ref mut macro_item) => {
            macro_item.accept(transformer)
        }
        Item::Macro2(ref mut macro2_item) => {
            macro2_item.accept(transformer)
        }
        Item::Static(ref mut static_item) => {
            static_item.accept(transformer)
        }
        Item::Trait(ref mut trait_item) => {
            trait_item.accept(transformer)
        }
        Item::TraitAlias(ref mut traitalias_item) => {
            traitalias_item.accept(transformer)
        }
        Item::Type(ref mut type_item) => {
            type_item.accept(transformer)
        }
        Item::Union(ref mut union_item) => {
            union_item.accept(transformer)
        }
        Item::Use(ref mut use_item) => {
            use_item.accept(transformer)
        }
        // note: Does not handle Verbatim and a private __Nonexhaustive.
        _ => {
            // todo, be more descriptive.
            panic!("Item must be mod or fn.");
        }
    }
}