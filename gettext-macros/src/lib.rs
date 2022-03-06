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
