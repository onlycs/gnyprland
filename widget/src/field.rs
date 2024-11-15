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
pub enum BindKind {
    Normal,
    Css,
    ClassName,
}

#[derive(Clone)]
pub enum Value {
    Expr(syn::Expr),
    ClassName(syn::Expr),
    Child(Widget),
    Children(Punctuated<Widget, Token![,]>),
    Binding { kind: BindKind, src: syn::Expr },
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
            "bind" => {
                ident = input.parse()?;
                input.parse::<Token![:]>()?;

                let kind = match ident.to_string().as_str() {
                    "css" => BindKind::Css,
                    "class_name" => BindKind::ClassName,
                    _ => BindKind::Normal,
                };

                let src = input.parse()?;

                Value::Binding { kind, src }
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
    pub fn to_tokens(&self, name: &syn::Ident) -> TokenStream {
        let ident_str = syn::Ident::new(&format!("set_{}", self.ident), self.ident.span());

        match &self.value {
            Value::Expr(expr) => quote! { #name.#ident_str(#expr); },
            Value::ClassName(cn) => quote! {
                for cn in #cn {
                    #name.style_context().add_class(cn);
                }
            },
            Value::Child(child) => quote! { #name.#ident_str(#child); },
            Value::Children(children) => {
                let children = children
                    .iter()
                    .cloned()
                    .map(|c| c.add_mod(Modifier::Inherited));

                quote! {
                    #name.set_children(&[#( #children ),*]);
                }
            }
            Value::Binding { src, kind } => {
                let dest = quote! { #name };
                let dest_prop = syn::LitStr::new(&self.ident.to_string(), self.ident.span());

                match kind {
                    BindKind::Css => quote! {
                        (#src).bind_css(&#dest);
                    },
                    BindKind::ClassName => quote! {
                        (#src).bind_class_name(&#dest);
                    },
                    BindKind::Normal => quote! {
                        (#src).bind(&#dest, #dest_prop);
                    },
                }
            }
        }
    }

    pub fn to_functional_tokens(&self) -> TokenStream {
        let ident = &self.ident;

        match &self.value {
            Value::Expr(expr) => quote! { #ident : { #expr } },
            Value::ClassName(cn) => quote! { #ident : { #cn } },
            Value::Child(child) => quote! { #ident: { #child } },
            Value::Binding { .. } => {
                quote! { compile_error!("Do not use the `bind` prefix in functional widgets") }
            }
            Value::Children(children) => {
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
