#![allow(clippy::into_iter_on_ref)]

use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Expr, LitStr, Result, Token,
};

mod arguments;
mod directives;
mod error;

use arguments::*;
use directives::*;
use error::*;

fn fallback(
    msgid: &LitStr,
    args: &Arguments,
    inv: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let arguments1 = (&args.0).into_iter();
    let arguments2 = (&args.0).into_iter();
    quote! {{
        match gettextrs::formatter::format(
            &#inv,
            &[#(#arguments1.to_string()),*]
        ) {
            Some(s) => s,
            None => format!(#msgid, #(#arguments2),*)
        }
    }}
}

fn comma_and_further(msgid: &LitStr, input: ParseStream) -> Result<Option<Arguments>> {
    let comma = &input.parse::<Token![,]>().err();
    let args = Arguments::parse(input).map_err(|err| combine_err(comma, err))?;
    let to_format = validate(msgid, args.0.len()).map_err(|err| {
        if !args.0.is_empty() {
            return combine_err(comma, err);
        }
        err
    })?;

    if !args.0.is_empty() {
        if let Some(e) = comma {
            return Err(e.to_owned());
        }
    }

    match to_format {
        true => Ok(Some(args)),
        false => Ok(None),
    }
}

struct Invocation {
    msgid: LitStr,
    to_format: Option<Arguments>,
}

impl Parse for Invocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let msgid: LitStr = match input.parse() {
            Err(e) => return Err(Error::new(e.span(), UiError::AtLeastMsgid)),
            Ok(v) => v,
        };
        let to_format = comma_and_further(&msgid, input)?;

        Ok(Self { msgid, to_format })
    }
}

impl ToTokens for Invocation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let msgid = &self.msgid;
        let inv = quote! { gettextrs::gettext(#msgid) };

        match &self.to_format {
            None => inv.to_tokens(tokens),
            Some(args) => fallback(msgid, args, inv).to_tokens(tokens),
        }
    }
}

#[proc_macro]
pub fn gettext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Invocation)
        .to_token_stream()
        .into()
}

#[cfg(test)]
mod test_invocation_parsing {
    use quote::quote;
    use syn::parse2;

    use crate::Invocation;

    #[test]
    fn just_msgid() {
        let inv = quote! {"Hello, World!"};
        let inv: Invocation = parse2(inv).unwrap();

        assert_eq!(&inv.msgid.value(), "Hello, World!");
        assert!(inv.to_format.is_none());
    }

    #[test]
    fn trailing_comma() {
        let inv = quote! {"Hello, World!",};
        let inv: Invocation = parse2(inv).unwrap();

        assert_eq!(&inv.msgid.value(), "Hello, World!");
        assert!(inv.to_format.is_none());
    }

    #[test]
    fn formatting() {
        let inv = quote! {"Hello, {}!", "World"};
        let inv: Invocation = parse2(inv).unwrap();

        assert_eq!(&inv.msgid.value(), "Hello, {}!");
        assert!(inv.to_format.is_some());
    }

    #[test]
    fn formatting_and_trailing_comma() {
        let inv = quote! {"Hello, {}!", "World",};
        let inv: Invocation = parse2(inv).unwrap();

        assert_eq!(&inv.msgid.value(), "Hello, {}!");
        assert!(inv.to_format.is_some());
    }
}

struct DInvocation {
    d: Expr,
    msgid: LitStr,
    to_format: Option<Arguments>,
}

impl Parse for DInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let d = match input.parse::<Expr>() {
            Err(e) => return Err(Error::new(e.span(), UiError::AtLeastDAndMsgid)),
            Ok(v) => v,
        };
        let comma = input.parse::<Token![,]>();
        let msgid = match input.parse::<LitStr>() {
            Err(e) => return Err(Error::new(e.span(), UiError::MissingMsgid)),
            Ok(v) => {
                comma?;
                v
            }
        };
        let to_format = comma_and_further(&msgid, input)?;

        Ok(Self {
            d,
            msgid,
            to_format,
        })
    }
}

impl ToTokens for DInvocation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let d = &self.d;
        let msgid = &self.msgid;
        let inv = quote! { gettextrs::dgettext(#d, #msgid) };

        match &self.to_format {
            None => inv.to_tokens(tokens),
            Some(args) => fallback(msgid, args, inv).to_tokens(tokens),
        }
    }
}

#[proc_macro]
pub fn dgettext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as DInvocation)
        .to_token_stream()
        .into()
}

#[cfg(test)]
mod test_dinvocation_parsing {
    use crate::DInvocation;
    use quote::quote;
    use syn::{parse2, Expr, Lit};

    #[test]
    fn domainname_and_msgid() {
        let inv = quote! {"domainname", "Hello, World!"};
        let inv: DInvocation = parse2(inv).unwrap();

        assert!(matches!(
            inv.d,
            Expr::Lit(lit)
            if matches!(
                &lit.lit,
                Lit::Str(lit)
                if lit.value() == "domainname"
            )
        ));
        assert_eq!(&inv.msgid.value(), "Hello, World!");
        assert!(&inv.to_format.is_none());
    }
}
