use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, LitInt, LitStr, Result, Token,
};

mod arguments;
mod directives;
mod helpers;

use arguments::*;
use directives::*;
use helpers::*;

struct Invocation {
    span: Span,
    msgid: LitStr,
    directives: Directives,
    arguments: Arguments,
}

impl Parse for Invocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let msgid: LitStr = input.parse()?;
        let directives = Directives::try_from(&msgid)?;
        let arguments = match input.parse::<Token![,]>() {
            Ok(_) => Arguments::parse(input)?,
            Err(_) => Arguments(Vec::with_capacity(0)),
        };

        Ok(Self {
            span,
            msgid,
            directives,
            arguments,
        })
    }
}

#[proc_macro]
pub fn gettext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Invocation);
    let Invocation {
        span,
        msgid,
        directives,
        arguments,
    } = input;

    match check_amount(directives.directives.len(), arguments.0.len()) {
        Ok(0) if directives.escapes == false => quote! { gettextrs::gettext(#msgid) }.into(),
        Ok(_) => {
            let arguments1 = (&arguments).into_iter();
            let arguments2 = (&arguments).into_iter();
            quote! {{
                match gettextrs::formatter::format(
                    &gettextrs::gettext(#msgid),
                    &[#(#arguments1.to_string()),*]
                ) {
                    Some(s) => s,
                    None => format!(#msgid, #(#arguments2),*)
                }
            }}
            .into()
        }
        Err(e) => Error::new(span, e).into_compile_error().into(),
    }
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

        Ok(Self {
            msgid,
            msgid_plural,
            n,
        })
    }
}

#[proc_macro]
pub fn ngettext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
