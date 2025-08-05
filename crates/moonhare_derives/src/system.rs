use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::add_trait_bounds;


pub fn system(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #name {
            fn init(&mut self) {

            }

            fn update(&mut self) {

            }

            fn add_child(&mut self, child: Self) {

            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
