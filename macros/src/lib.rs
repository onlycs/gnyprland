mod render;
mod widget;

use proc_macro::TokenStream as RsTokenStream;
use quote::quote;
use render::widget::Widget;

#[proc_macro]
pub fn render(input: RsTokenStream) -> RsTokenStream {
    let w = syn::parse_macro_input!(input as Widget);

    quote! {
        #w
    }
    .into()
}

#[proc_macro_attribute]
pub fn widget(_: RsTokenStream, input: RsTokenStream) -> RsTokenStream {
    widget::imp(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
