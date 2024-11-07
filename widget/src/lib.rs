use std::mem;

use proc_macro::TokenStream as RsTokenStream;
use proc_macro2::{Delimiter, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{discouraged::AnyDelimiter, Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Kind {
    Default,
    Ref,
    Optional,
    Inherited,
}

#[derive(Clone)]
enum Value {
    Expr(syn::Expr),
    ClassName(syn::Expr),
    Child(Widget),
    Children(Punctuated<Widget, Token![,]>),
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Value::Expr(expr) => expr.to_tokens(tokens),
            Value::ClassName(expr) => expr.to_tokens(tokens),
            Value::Child(widget) => quote! {
                ::widget::widget!(opt #widget)
            }
            .to_tokens(tokens),
            Value::Children(widgets) => {
                let iter = widgets.iter();

                quote! {
                    &[#(widget::widget!(inh #iter)),*]
                }
                .to_tokens(tokens);
            }
        }
    }
}

#[derive(Clone)]
struct Field {
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
            "child" => {
                if input.peek(syn::Ident) {
                    ident = input.parse()?;
                }

                input.parse::<Token![:]>()?;
                Value::Child(input.parse()?)
            }
            "children" => {
                let (delim, span, inner) = input.parse_any_delimiter()?;

                if delim != Delimiter::Brace {
                    return Err(syn::Error::new(span.open(), "expected bracket"));
                }

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
    fn to_tokens(&self, builder_name: &syn::Ident) -> TokenStream {
        let ident_str = syn::Ident::new(&format!("set_{}", self.ident), self.ident.span());

        match self.value {
            Value::Expr(ref expr) => quote! { #builder_name . #ident_str ( #expr ); },
            Value::ClassName(ref cn) => quote! {
                for cn in #cn {
                    #builder_name.style_context().add_class(cn);
                }
            },
            Value::Child(ref child) => {
                let child = child.clone().build_upd_kind(Kind::Optional);

                quote! {
                    #builder_name . #ident_str (#child);
                }
            }
            Value::Children(ref children) => {
                let children = children
                    .iter()
                    .cloned()
                    .map(|c| c.build_upd_kind(Kind::Inherited));

                quote! {
                    #builder_name . set_children(&[#( #children ),*]);
                }
            }
        }
    }
}

#[derive(Clone)]
struct Widget {
    kind: Kind,
    name: syn::Ident,
    fields: Punctuated<Field, Token![,]>,
}

impl Widget {
    fn upd_kind(&mut self, k: Kind) {
        if self.kind == Kind::Default {
            self.kind = k;
        }
    }

    fn build_upd_kind(mut self, k: Kind) -> Self {
        self.upd_kind(k);

        self
    }
}

impl Parse for Widget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = input.parse()?;
        let mut kind = Kind::Default;

        if input.peek(syn::Ident) {
            let mut i_kind = input.parse::<syn::Ident>()?;
            mem::swap(&mut name, &mut i_kind);

            if i_kind == "bor" {
                kind = Kind::Ref;
            } else if i_kind == "opt" {
                kind = Kind::Optional;
            } else if i_kind == "inh" {
                kind = Kind::Inherited;
            } else {
                return Err(syn::Error::new(i_kind.span(), "expected `bor` or `opt`"));
            }
        }

        let (delim, span, parser) = input.parse_any_delimiter()?;

        if delim != Delimiter::Brace {
            return Err(syn::Error::new(span.open(), "expected block"));
        }

        let fields = Punctuated::<Field, Token![,]>::parse_terminated(&parser)?;

        Ok(Self { kind, name, fields })
    }
}

impl ToTokens for Widget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let var_ident = syn::Ident::new("widget", name.span());

        let fields = self.fields.iter().map(|f| f.to_tokens(&var_ident));

        let body = quote! {{
            #[allow(non_snake_case)]
            let #var_ident = #name::default();
            #(#fields);*
            #var_ident
        }};

        match self.kind {
            Kind::Default => quote! {
                #body
            },
            Kind::Ref => quote! {
                &#body
            },
            Kind::Optional => quote! {
                Some(&#body)
            },
            Kind::Inherited => quote! {
                ::gtk::Widget::from(#body)
            },
        }
        .to_tokens(tokens);
    }
}

#[proc_macro]
pub fn widget(input: RsTokenStream) -> RsTokenStream {
    let w = syn::parse_macro_input!(input as Widget);

    quote! {
        #w
    }
    .into()
}
