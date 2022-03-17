#![allow(clippy::into_iter_on_ref)]

use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Result, Token,
};

mod arguments;
mod directives;
mod helpers;

use arguments::*;
use directives::*;

struct Invocation {
    msgid: LitStr,
    to_format: Option<Arguments>,
}

impl Parse for Invocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let msgid: LitStr = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let args = Arguments::parse(input)?;
        let to_format = match validate(&msgid, args.0.len())? {
            true => Some(args),
            false => None,
        };

        Ok(Self { msgid, to_format })
    }
}

#[proc_macro]
pub fn gettext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Invocation);
    let Invocation { msgid, to_format } = input;

    match to_format {
        None => quote! { gettextrs::gettext(#msgid) }.into(),
        Some(args) => {
            let arguments1 = (&args.0).into_iter();
            let arguments2 = (&args.0).into_iter();
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
    }
}
