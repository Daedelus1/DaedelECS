extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    // Parse the input tokens directly
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();

    // Build the impl
    let name = &ast.ident;
    let generated: TokenStream = quote! {
        impl Component for #name {}
    }
        .into();

    // Return the generated impl
    generated.into()
}
