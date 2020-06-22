//! Mod Docs
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
use syn::{parse, AttributeArgs, Item, parse_macro_input};
mod visit;
mod transform;
use visit::Visit;
use transform::Transform;

struct Temp {}
impl Transform for Temp {
    fn transform(self) -> Self {
        self
    }
}

// For my own ease.
type TS = TokenStream;

pub fn apply_attribute(attrs: TS, it: TS) -> TS {
    // Grab it as an item.
    let item: Item = parse(it.clone()).expect("Expected an item");
    let attributes: AttributeArgs = parse_macro_input!(attrs as AttributeArgs);
    match item {
        // Handle fn application.
        Item::Fn(item) => {
            let t = Temp{};
            item.visit(attributes, t)
        }
        // Handle module application.
        Item::Mod(item) => {
            let t = Temp{};
            item.visit(attributes, t)
        }
        // Can't handle this.
        _ => {
            panic!("Item must be mod or fn.");
        }
    }
}