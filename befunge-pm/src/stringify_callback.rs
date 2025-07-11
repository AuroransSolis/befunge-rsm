use crate::callback::Callback;
use proc_macro2::Group;
use syn::{
    Token,
    parse::{Parse, ParseStream},
};

pub struct StringifyCallback {
    pub tokens: Group,
    pub callback: Callback,
}

impl Parse for StringifyCallback {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<crate::kw::tokens>()?;
        input.parse::<Token![:]>()?;
        let tokens = input.parse()?;
        input.parse::<Token![,]>()?;
        let callback = crate::callback::parse_callback(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(StringifyCallback { tokens, callback })
    }
}
