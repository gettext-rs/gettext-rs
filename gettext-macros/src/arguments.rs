use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

pub enum Argument {
    Pos(Expr),
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use Argument::*;
        match self {
            Pos(expr) => expr.to_tokens(tokens),
        }
    }
}

pub struct Arguments(pub Vec<Argument>);

impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use Argument::*;
        let mut pos_args = Vec::new();
        if let Ok(p) = Punctuated::<Expr, Token![,]>::parse_terminated(input) {
            pos_args = p.into_iter().map(Pos).collect();
        }
        Ok(Self(pos_args))
    }
}

impl IntoIterator for Arguments {
    type Item = Argument;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Arguments {
    type Item = &'a Argument;
    type IntoIter = std::slice::Iter<'a, Argument>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}
