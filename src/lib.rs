//! Mod Docs
// #[!warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{
    parse, AttributeArgs, Item,
    parse_macro_input, ItemFn,
};
use quote::quote;
mod visitable;
mod visitor;
use visitor::Visitor;
use visitable::Visitable;

struct Rename {}
impl Visitor for Rename {
    fn visit_fn(&self, func: &mut ItemFn) -> TokenStream {
        let ident = quote::format_ident!("{}", "Foo");
        func.sig.ident = ident;
        (quote! {
            #[test]
            #func
        }).into()
    }
}

// For my own ease.
type TS = TokenStream;

pub(crate) fn apply_attribute<T>(attrs: TS, it: TS, transformer: T) -> TS
    where T: Visitor
{
    // Ok, let parse consume it, we own item and we need it as mut.
    let mut item: Item = parse(it).expect("Expected an item");
    // Similar to above but no mut needed.
    let _attributes: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    // todo: handle attrs etc
    // todo: mas elegant way? This seems overkill, for every enum variant we
    // todo: do similar things here and in visitor.
    match item {
        // Handle fn application.
        Item::Fn(ref mut func) => {
            func.accept(&transformer)
        }
        Item::Struct(ref mut structure) => {
            structure.accept(&transformer)
        }
        Item::Mod(ref mut module) => {
            module.accept(&transformer)
        }
        Item::Const(ref mut const_item) => {
            const_item.accept(&transformer)
        }
        Item::Enum(ref mut enum_item) => {
            enum_item.accept(&transformer)
        }
        Item::ExternCrate(ref mut externcrate_item) => {
            externcrate_item.accept(&transformer)
        }
        Item::ForeignMod(ref mut foreignmod_item) => {
            foreignmod_item.accept(&transformer)
        }
        Item::Impl(ref mut impl_item) => {
            impl_item.accept(&transformer)
        }
        // macro_rules macro def
        Item::Macro(ref mut macro_item) => {
            macro_item.accept(&transformer)
        }
        Item::Macro2(ref mut macro2_item) => {
            macro2_item.accept(&transformer)
        }
        Item::Static(ref mut static_item) => {
            static_item.accept(&transformer)
        }
        Item::Trait(ref mut trait_item) => {
            trait_item.accept(&transformer)
        }
        Item::TraitAlias(ref mut traitalias_item) => {
            traitalias_item.accept(&transformer)
        }
        Item::Type(ref mut type_item) => {
            type_item.accept(&transformer)
        }
        Item::Union(ref mut union_item) => {
            union_item.accept(&transformer)
        }
        Item::Use(ref mut use_item) => {
            use_item.accept(&transformer)
        }
        // Can't handle this.
        _ => {
            // todo, be more descriptive.
            panic!("Item must be mod or fn.");
        }
    }
}


/// note: Merely for testing things now; need to place it as a sub crate that isn't
/// note: exported somehow (not sure how I do this, yet)
#[proc_macro_attribute]
pub fn rename(attrs: TS, it: TS) -> TS {
    let r = Rename{};
    apply_attribute(attrs, it, r)
}