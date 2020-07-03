extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{AttributeArgs, ItemFn};
use quote::quote;
use supattrs::{apply_attribute, visitor::Visitor};

struct Rename {}
impl Visitor for Rename {
    // todo: mut ref for self?
    fn visit_fn(&self, func: &mut ItemFn, attrs: &AttributeArgs) -> TokenStream {
        // ignore these.
        let _attrs = attrs;

        let ident = quote::format_ident!("{}", "Foo");
        println!("In another item too but, I get visited.");
        func.sig.ident = ident;
        (quote! {
            #[test]
            #func
        }).into()
    }
}


#[proc_macro_attribute]
pub fn rename(attrs: TokenStream, it: TokenStream) -> TokenStream {
    let r = Rename{};
    // note: transformer *could* be passed by ref
    apply_attribute(attrs, it, r)
}
