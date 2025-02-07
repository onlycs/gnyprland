use crate::render::field::Field;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Token,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modifier {
    Unset,
    Ref,
    Optional,
    Inherited,
}

impl Modifier {
    pub fn is_unset(&self) -> bool {
        matches!(self, Modifier::Unset)
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let Ok(ident) = fork.parse::<syn::Ident>() else {
            return Ok(Modifier::Unset);
        };

        let kind = match ident.to_string().as_str() {
            "bor" => Modifier::Ref,
            "opt" => Modifier::Optional,
            "inh" => Modifier::Inherited,
            _ => Modifier::Unset,
        };

        if !matches!(kind, Modifier::Unset) {
            input.parse::<syn::Ident>()?;
        }

        Ok(kind)
    }
}

#[derive(Clone)]
pub struct Widget {
    modifiers: Vec<Modifier>,
    name: syn::Path,
    functional: bool,
    fields: Punctuated<Field, syn::Token![,]>,
}

impl Widget {
    pub fn add_mod(mut self, m: Modifier) -> Self {
        if self.modifiers[0] == Modifier::Unset {
            self.modifiers[0] = m;
        }

        self
    }
}

impl Parse for Widget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut modifiers = vec![];
        let mut functional = false;

        // if some and unset break
        // else allow
        while !modifiers.last().is_some_and(|m: &Modifier| m.is_unset()) {
            modifiers.push(input.parse()?);
        }

        if input.peek(syn::Ident) {
            let fork = input.fork();
            let fun = fork.parse::<syn::Ident>()?;

            if fun == "fun" {
                functional = true;
                input.parse::<syn::Ident>()?;
            }
        }

        let name = input.parse::<syn::Path>()?;

        let fields = match input.is_empty() || input.peek(Token![,]) {
            true => Punctuated::new(),
            false => {
                let inner;
                braced!(inner in input);
                Punctuated::<Field, Token![,]>::parse_terminated(&inner)?
            }
        };

        Ok(Self {
            modifiers,
            name,
            fields,
            functional,
        })
    }
}

impl ToTokens for Widget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let var_ident = syn::Ident::new("widget", name.span());

        let mut body = if self.functional && self.fields.len() != 0 {
            let fields = self.fields.iter().map(|f| f.to_functional_tokens());

            quote! {
                #name::widget(#name::Props {
                    #(#fields,)*
                })
            }
        } else if self.functional {
            quote! { #name::widget() }
        } else {
            let fields = self.fields.iter().map(|f| f.to_tokens(&var_ident));

            quote! {{
                #[allow(non_snake_case)]
                let #var_ident = #name::default();
                #(#fields);*
                #var_ident
            }}
        };

        for modif in self.modifiers.iter().rev() {
            body = match modif {
                Modifier::Ref => quote! { &#body },
                Modifier::Optional => quote! { Some(&#body) },
                Modifier::Inherited => quote! { ::gtk::Widget::from(#body) },
                Modifier::Unset => body,
            };
        }

        body.to_tokens(tokens);
    }
}
