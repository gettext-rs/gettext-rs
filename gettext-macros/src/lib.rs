use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitInt, LitStr, Result, Token,
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

struct NInvocation {
    msgid: LitStr,
    msgid_plural: LitStr,
    n: LitInt,
}

impl Parse for NInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let msgid: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let msgid_plural: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let n: LitInt = input.parse()?;

        Ok(NInvocation {
            msgid,
            msgid_plural,
            n,
        })
    }
}

#[proc_macro]
pub fn ngettext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as NInvocation);
    let NInvocation {
        msgid,
        msgid_plural,
        n,
    } = input;
    let tokens = quote! {
        { gettextrs::ngettext(#msgid, #msgid_plural, #n) }
    };
    tokens.into()
}
