use crate::widget::Modifier;
use crate::Widget;
use proc_macro2::TokenStream;
use quote::quote;
use syn::braced;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

#[derive(Clone)]
pub enum Value {
    Expr(syn::Expr),
    ClassName(syn::Expr),
    Child(Widget),
    Children(Punctuated<Widget, Token![,]>),
}

#[derive(Clone)]
pub struct Field {
    ident: syn::Ident,
    value: Value,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = input.parse::<syn::Ident>()?;

        let value = match ident.to_string().as_str() {
            "class_name" => {
                input.parse::<Token![:]>()?;
                Value::ClassName(input.parse()?)
            }
            "name" => {
                ident = syn::Ident::new("widget_name", ident.span());
                input.parse::<Token![:]>()?;
                Value::Expr(input.parse()?)
            }
            "child" => {
                if input.peek(syn::Ident) {
                    ident = input.parse()?;
                }

                input.parse::<Token![:]>()?;
                Value::Child(input.parse::<Widget>()?.add_mod(Modifier::Optional))
            }
            "children" => {
                let inner;
                braced!(inner in input);

                Value::Children(Punctuated::parse_terminated(&inner)?)
            }
            _ => {
                input.parse::<Token![:]>()?;
                Value::Expr(input.parse()?)
            }
        };

        Ok(Self { ident, value })
    }
}

impl Field {
    pub fn to_tokens(&self, builder_name: &syn::Ident) -> TokenStream {
        let ident_str = syn::Ident::new(&format!("set_{}", self.ident), self.ident.span());

        match self.value {
            Value::Expr(ref expr) => quote! { #builder_name . #ident_str ( #expr ); },
            Value::ClassName(ref cn) => quote! {
                for cn in #cn {
                    #builder_name.style_context().add_class(cn);
                }
            },
            Value::Child(ref child) => {
                quote! {
                    #builder_name . #ident_str (#child);
                }
            }
            Value::Children(ref children) => {
                let children = children
                    .iter()
                    .cloned()
                    .map(|c| c.add_mod(Modifier::Inherited));

                quote! {
                    #builder_name . set_children(&[#( #children ),*]);
                }
            }
        }
    }

    pub fn to_functional_tokens(&self) -> TokenStream {
        let ident = &self.ident;

        match self.value {
            Value::Expr(ref expr) => quote! { #ident : { #expr } },
            Value::ClassName(ref cn) => quote! { #ident : { #cn } },
            Value::Child(ref child) => quote! { #ident: { #child } },
            Value::Children(ref children) => {
                let children = children
                    .iter()
                    .cloned()
                    .map(|c| c.add_mod(Modifier::Inherited));

                quote! {
                    #ident: {
                        &[#( #children ),*]
                    }
                }
            }
        }
    }
}
