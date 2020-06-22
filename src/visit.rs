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
use syn;
use crate::transform::Transform;

type TS = TokenStream;

/// Visit is bounded by a Transform trait and requires the `visit` method. That method
/// should visit the specific item and apply the `transform` function of the
/// `Transform` trait on it.
pub trait Visit<T>
    where T: Transform
{
    fn visit(&self, attributes: syn::AttributeArgs, transformer: T) -> TS;
}

/// Implement for syn::ItemMod, should go through all items in mod and call `visit`.
impl<T> Visit<T> for syn::ItemMod
    where T: Transform
{
    fn visit(&self, attributes: syn::AttributeArgs, _transformer: T) -> TS {
        println!("{}", "I get here, itemmod");
        (quote!{
                #self
            }).into()
    }
}

/// Implement for syn::ItemFn, base case, transforms itself.
impl<T> Visit<T> for syn::ItemFn
    where T: Transform
{
    fn visit(&self, attributes: syn::AttributeArgs, _transformer: T) -> TS {
        println!("{}", "I get here, itemfn");
        (quote!{
                #self
            }).into()
    }
}
