//! Mod Docs
// #[!warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

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
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{
    parse, AttributeArgs, Item,
    parse_macro_input, ItemFn,
};
mod visitable;
mod visitor;
////use visit::Visit;
use visitor::Visitor;
use visitable::Visitable;

struct Rename {}
impl Visitor for Rename {
    fn visit_fn<'a>(&self, func: &'a mut ItemFn) -> &'a mut ItemFn {
        let ident = quote::format_ident!("{}", "Foo");
        func.sig.ident = ident;

        func
    }
}

// For my own ease.
type TS = TokenStream;

pub(crate) fn apply_attribute<T>(attrs: TS, it: TS, transformer: T) -> TS
    where T: Visitor
{
    // We want a mut item here, we plan to change it in some way.
    // todo: do they syn docs advise against this?
    let mut item: Item = parse(it.clone()).expect("Expected an item");
    let _attributes: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    // todo: add rest.
    match item {
        // Handle fn application.
        Item::Fn(ref mut item) => {
            item.accept(&transformer)
        }
        Item::Struct(ref mut item) => {
            item.accept(&transformer)
        }
        Item::Mod(ref mut item) => {
            item.accept(&transformer)
        }
        // Can't handle this.
        _ => {
            panic!("Item must be mod or fn.");
        }
    }
}

#[proc_macro_attribute]
pub fn rename(attrs: TS, it: TS) -> TS {
    let r = Rename{};
    apply_attribute(attrs, it, r)
}