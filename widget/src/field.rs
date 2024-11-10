use crate::widget::Modifier;
use crate::Widget;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{braced, Expr, Member};
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
    Mapping {
        ty: syn::Type,
        src: syn::Expr,
        src_prop: syn::Ident,
        maps: Vec<syn::Expr>,
    },
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

                let cast = input.parse::<syn::ExprCast>()?;
                let src = *cast.expr;
                let ty = *cast.ty;

                let Expr::Field(src) = src else {
                    return Err(syn::Error::new(src.span(), "expected field access"));
                };

                let lhs = *src.base;
                let Member::Named(rhs) = src.member else {
                    return Err(syn::Error::new(
                        src.member.span(),
                        "expected named field access",
                    ));
                };

                let mut maps = vec![];

                while input.peek(syn::Ident) {
                    let map = input.parse::<syn::Ident>()?;

                    if map != "map" {
                        return Err(syn::Error::new(map.span(), "expected `map`"));
                    }

                    maps.push(input.parse()?);
                }

                Value::Mapping {
                    ty,
                    src: lhs,
                    src_prop: rhs,
                    maps,
                }
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
            Value::Mapping {
                src,
                src_prop,
                ty,
                maps,
            } => {
                let src_prop = syn::LitStr::new(&src_prop.to_string(), src_prop.span());
                let dest = quote! { #name };
                let dest_prop = syn::LitStr::new(&self.ident.to_string(), self.ident.span());

                let begin = quote! {
                    #src.bind::<#ty>(#src_prop)
                };

                let maps = maps.iter().map(|m| {
                    quote! {
                        .transform(#m)
                    }
                });

                let end = quote! {
                    .bind(&#dest, #dest_prop);
                };

                quote! { #begin #(#maps)* #end; }
            }
        }
    }

    pub fn to_functional_tokens(&self) -> TokenStream {
        let ident = &self.ident;

        match &self.value {
            Value::Expr(expr) => quote! { #ident : { #expr } },
            Value::ClassName(cn) => quote! { #ident : { #cn } },
            Value::Child(child) => quote! { #ident: { #child } },
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
            Value::Mapping {
                ty,
                src,
                src_prop,
                maps,
            } => {
                let src_prop = syn::LitStr::new(&src_prop.to_string(), src_prop.span());

                let begin = quote! {
                    #src.bind::<#ty>(#src_prop)
                };

                let maps = maps.iter().map(|m| {
                    quote! {
                        .transform(|value| {
                            (#m)(value)
                        })
                    }
                });

                quote! {
                    #ident: {
                        #begin #(#maps)*
                    }
                }
            }
        }
    }
}
