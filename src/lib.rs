//! Mod Docs
#![allow(unused_mut)] // not sure why mut item is flagged, can't pass it as mut ref otherwise.
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
use visitable::dispatch;

struct Rename {}
impl Visitor for Rename {
    // todo: mut ref for self?
    fn visit_fn(&self, func: &mut ItemFn, attrs: &AttributeArgs) -> TokenStream {
        // ignore these.
        let _attrs = attrs;

        let ident = quote::format_ident!("{}", "Foo");
        println!("{}", "In another item too but, I get visited.");
        func.sig.ident = ident;
        (quote! {
            #[test]
            #func
        }).into()
    }
}

// For my own ease.
type TS = TokenStream;

/// Apply attribute: Take a visitor and applies it on all items.
///
/// ##### Notes:
///
/// attrs and it *need* to be moved in since parse consumes them; transformer
/// *could* be passed by reference. -- todo: pass by ref?

pub(crate) fn apply_attribute<T>(attrs: TS, it: TS, transformer: T) -> TS
    where T: Visitor
{
    // Ok, let parse consume it, we own item and we need it as mut.
    let mut item: Item = parse(it).expect("Expected an item");
    // Similar to above but no mut needed.
    let attributes: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    // todo: handle attrs etc
    // todo: mas elegant way? This seems overkill, for every enum variant we
    // todo: do similar things here and in visitor.
    dispatch(&mut item, &attributes, &transformer)
}


/// note: Merely for testing things now; need to place it as a sub crate that isn't
/// note: exported somehow (not sure how I do this, yet)
#[proc_macro_attribute]
pub fn rename(attrs: TS, it: TS) -> TS {
    let r = Rename{};
    // note: transformer *could* be passed by ref
    apply_attribute(attrs, it, r)
}