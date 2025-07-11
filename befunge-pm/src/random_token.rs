use crate::callback::Callback;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Token, bracketed,
    parse::{Parse, ParseStream},
};

pub struct ChooseRandom {
    pub choices: TokenStream2,
    pub callback: Callback,
}

impl Parse for ChooseRandom {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<crate::kw::choices>()?;
        input.parse::<Token![:]>()?;
        let choices;
        bracketed!(choices in input);
        let choices = choices.parse()?;
        input.parse::<Token![,]>()?;
        let callback = crate::callback::parse_callback(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(ChooseRandom { choices, callback })
    }
}
