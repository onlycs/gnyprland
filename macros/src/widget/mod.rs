use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, ItemFn, Pat};

pub fn imp(tokens: TokenStream) -> syn::Result<TokenStream> {
    let func = syn::parse2::<ItemFn>(tokens)?;
    let args = func.sig.inputs;
    let name = func.sig.ident;
    let body = *func.block;

    let mut idents = vec![];
    let mut tys = vec![];

    for arg in args {
        match arg {
            syn::FnArg::Typed(pat) => {
                let ty = pat.ty;
                let Pat::Ident(name) = *pat.pat else {
                    return Err(syn::Error::new(
                        pat.pat.span(),
                        "Expected a `name: ty` argument",
                    ));
                };

                idents.push(name.ident);
                tys.push(ty);
            }
            arg => return Err(syn::Error::new(arg.span(), "Expected a typed argument")),
        }
    }

    let props = match idents.len() {
        0 => quote! {},
        _ => quote! { pub struct Props { #(pub #idents:#tys),* } },
    };

    let func = match idents.len() {
        0 => quote! {
            pub fn widget() -> ::gtk::Widget #body
        },
        _ => quote! {
            pub fn widget(Props { #(#idents),* }: Props) -> ::gtk::Widget #body
        },
    };

    Ok(quote! {
        #[allow(non_snake_case)]
        pub mod #name {
            use super::*;

            #[warn(non_snake_case)]
            #props

            #[warn(non_snake_case)]
            #func
        }
    })
}
