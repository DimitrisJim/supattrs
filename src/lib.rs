//! Mod Docs
#![allow(unused_mut)] // not sure why mut item is flagged, can't pass it as mut ref otherwise.
// #[!warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{
    parse, AttributeArgs, Item,
    parse_macro_input,
};
mod visitable;
pub mod visitor;
use visitor::Visitor;
use visitable::dispatch;

// For my own ease.
type TS = TokenStream;

/// Apply attribute: Take a visitor and applies it on all items.
///
/// ##### Notes:
///
/// attrs and it *need* to be moved in since parse consumes them; transformer
/// *could* be passed by reference. -- todo: pass by ref?
pub fn apply_attribute<T>(attrs: TS, it: TS, transformer: T) -> TS
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