mod field;
mod widget;

use proc_macro::TokenStream as RsTokenStream;
use quote::quote;
use widget::Widget;

#[proc_macro]
pub fn widget(input: RsTokenStream) -> RsTokenStream {
    let w = syn::parse_macro_input!(input as Widget);

    quote! {
        #w
    }
    .into()
}
