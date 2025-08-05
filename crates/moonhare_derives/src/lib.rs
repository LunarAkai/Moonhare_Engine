use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};
use proc_macro::{self, TokenStream};

mod node;
mod system;

#[proc_macro_derive(System)]
pub fn system(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    system::system(input)
}



#[proc_macro_derive(Node)]
pub fn node(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    TokenStream::from(node::node(input))
}

// Add a bound `T: Node` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(node::Node));
        }
    }
    generics
}