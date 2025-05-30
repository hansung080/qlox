extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(ResolveSnippet)]
pub fn derive_resolve_snippet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let typ = &input.ident;
    let expanded = if let Data::Enum(data) = &input.data {
        let variants = data.variants.iter().map(|v| &v.ident);
        quote! {
            impl crate::src::ResolveSnippet for #typ {
                fn resolve_snippet(&mut self, source: &[u8]) {
                    match self {
                        #(
                            Self::#variants { snippet, .. } => snippet.resolve(source)
                        ),*
                    }
                }
            }
        }
    } else {
        quote! {
            compile_error!("derive macro `src::ResolveSnippet` can only be applied to `enum`s");
        }
    };
    expanded.into()
}