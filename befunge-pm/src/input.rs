use crate::callback::Callback;
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
};

pub struct BefungeInput {
    pub file: LitStr,
    pub callback: Callback,
}

impl Parse for BefungeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<crate::kw::file>()?;
        input.parse::<Token![:]>()?;
        let file = input.parse()?;
        input.parse::<Token![,]>()?;
        let callback = crate::callback::parse_callback(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(BefungeInput { file, callback })
    }
}
