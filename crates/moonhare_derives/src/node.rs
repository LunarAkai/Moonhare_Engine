use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::add_trait_bounds;

pub fn node(input: DeriveInput) -> TokenStream2 {
    let name = input.ident;

    let expanded = quote! {
        impl Node for #name {
            fn init(&mut self) {
                self.init();
            }

            fn update(&mut self) {
                self.update();
            }
        }
    };

    expanded
}