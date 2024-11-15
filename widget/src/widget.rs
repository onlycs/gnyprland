use crate::field::Field;
use quote::{quote, ToTokens};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Token,
};

#[derive(Clone)]
pub struct FunctionalData(Option<Option<syn::Path>>);

impl Parse for FunctionalData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        let Ok(ident) = fork.parse::<syn::Ident>() else {
            return Ok(Self(None));
        };

        if ident.to_string() != "fun" {
            return Ok(Self(None));
        } else {
            drop(fork);
            input.parse::<syn::Ident>()?;
        }

        let inner;
        parenthesized!(inner in input);

        if inner.is_empty() {
            return Ok(Self(Some(None)));
        }

        Ok(Self(Some(Some(inner.parse()?))))
    }
}

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
    fields: Punctuated<Field, syn::Token![,]>,
    func: FunctionalData,
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

        // if some and unset break
        // else allow
        while !modifiers.last().is_some_and(|m: &Modifier| m.is_unset()) {
            modifiers.push(input.parse()?);
        }

        let func = input.parse()?;
        let name = input.parse::<syn::Path>()?;
        let inner;
        braced!(inner in input);
        let fields = Punctuated::<Field, Token![,]>::parse_terminated(&inner)?;

        Ok(Self {
            modifiers,
            name,
            fields,
            func,
        })
    }
}

impl ToTokens for Widget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let var_ident = syn::Ident::new("widget", name.span());

        let mut body = if let FunctionalData(Some(Some(arg))) = &self.func {
            let fields = self.fields.iter().map(|f| f.to_functional_tokens());

            quote! {
                #name(#arg {
                    #(#fields,)*
                })
            }
        } else if let FunctionalData(Some(None)) = &self.func {
            quote! { #name() }
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
