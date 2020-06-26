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
use visitable::dispatch;

struct Rename {}
impl Visitor for Rename {
    // todo: mut ref?
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
    dispatch(&mut item, &transformer)
}


/// note: Merely for testing things now; need to place it as a sub crate that isn't
/// note: exported somehow (not sure how I do this, yet)
#[proc_macro_attribute]
pub fn rename(attrs: TS, it: TS) -> TS {
    let r = Rename{};
    // note: transformer *could* be passed by ref
    apply_attribute(attrs, it, r)
}