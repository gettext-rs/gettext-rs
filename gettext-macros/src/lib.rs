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

    match check_amount(directives.number, arguments.0.len()) {
        Ok(0) if !directives.escapes => quote! { gettextrs::gettext(#msgid) }.into(),
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
    span: Span,
    msgid: LitStr,
    msgid_plural: LitStr,
    n: LitInt,
    directives: Directives,
    directives_plural: Directives,
    arguments: Arguments,
}

impl Parse for NInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let msgid: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let msgid_plural: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let n: LitInt = input.parse()?;
        let directives = Directives::try_from(&msgid)?;
        let directives_plural = Directives::try_from(&msgid_plural)?;
        let arguments = match input.parse::<Token![,]>() {
            Ok(_) => Arguments::parse(input)?,
            Err(_) => Arguments(Vec::with_capacity(0)),
        };

        Ok(Self {
            span,
            msgid,
            msgid_plural,
            n,
            directives,
            directives_plural,
            arguments,
        })
    }
}

#[proc_macro]
pub fn ngettext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as NInvocation);
    let NInvocation {
        span,
        msgid,
        msgid_plural,
        n,
        directives,
        directives_plural,
        arguments,
    } = input;

    let n_arguments = arguments.0.len();
    match check_amount(directives.number, n_arguments) {
        Ok(n_directives) => match check_amount(directives_plural.number, n_arguments) {
            Ok(0)
                if n_directives == 0
                    && (!directives.escapes || !directives_plural.escapes) =>
            {
                quote! { gettextrs::ngettext(#msgid, #msgid_plural, #n) }.into()
            }
            Ok(_) => {
                let arguments1 = (&arguments).into_iter();
                let arguments2 = (&arguments).into_iter();
                let format = {
                    match n.base10_parse() {
                        Ok(1) => quote! { format!(#msgid, #(#arguments2),*) },
                        Ok(_) => quote! { format!(#msgid_plural, #(#arguments2),*) },
                        Err(e) => e.into_compile_error(),
                    }
                };
                quote! {{
                    match gettextrs::formatter::format(
                        &gettextrs::ngettext(#msgid, #msgid_plural, #n),
                        &[#(#arguments1.to_string()),*]
                    ) {
                        Some(s) => s,
                        None => #format
                    }
                }}
                .into()
            }
            Err(e) => Error::new(span, e).into_compile_error().into(),
        },
        Err(e) => Error::new(span, e).into_compile_error().into(),
    }
}
