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
        Ok(Self(
            Punctuated::<Expr, Token![,]>::parse_terminated(input)?
                .into_iter()
                .map(Pos)
                .collect(),
        ))
    }
}
