use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Result,
};

struct Invocation {
    msgid: LitStr,
}

impl Parse for Invocation {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            msgid: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn gettext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Invocation);
    let Invocation { msgid } = input;
    let tokens = quote! {
        { gettextrs::gettext(#msgid) }
    };
    tokens.into()
}

