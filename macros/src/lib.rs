extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(ResolveError)]
pub fn derive_resolve_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let typ = &input.ident;
    let expanded = if let Data::Enum(data) = &input.data {
        let variants = data.variants.iter().map(|v| &v.ident);
        quote! {
            impl crate::loc::ResolveLoc for #typ {
                fn resolve_loc(&mut self, source: &[u8]) {
                    match self {
                        #(
                            Self::#variants { loc, .. } => loc.resolve(source)
                        ),*
                    }
                }
            }
        }
    } else {
        quote! {
            compile_error!("derive macro `error::ResolveError` may only be applied to `enum`s");
        }
    };
    expanded.into()
}