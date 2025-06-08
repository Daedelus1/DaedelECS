use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn generate_component_tuple_impls(input: TokenStream) -> TokenStream {
    let max_size: usize = parse_macro_input!(input as syn::LitInt)
        .base10_parse()
        .expect("Failed to parse integer literal");

    let mut impls = Vec::new();

    for i in 0..=max_size {
        let type_params: Vec<_> = (0..i).map(|j| quote::format_ident!("T{}", j)).collect();
        let type_constraints = type_params
            .iter()
            .map(|t| quote! { #t: Component + 'static });

        let tuple_type = match type_params.len() {
            0 => quote! { () },
            1 => {
                let param = &type_params[0];
                quote! { #param }
            }
            _ => quote! { (#(#type_params),*) }
        };

        let type_ids_impl = if type_params.is_empty() {
            quote! {
                fn type_ids() -> Vec<TypeId> {
                    vec![]
                }
            }
        } else {
            let type_ids = type_params.iter().map(|t| {
                quote! { TypeId::of::<#t>() }
            });
            quote! {
                fn type_ids() -> Vec<TypeId> {
                    vec![#(#type_ids),*]
                }
            }
        };

        impls.push(quote! {
            impl<#(#type_constraints),*> SystemData for #tuple_type {
                #type_ids_impl
            }
        });
    }

    quote! {
        #(#impls)*
    }
        .into()
}
