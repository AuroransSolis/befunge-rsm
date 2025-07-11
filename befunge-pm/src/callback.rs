use proc_macro2::Group;
use syn::{
    Path as SynPath, Token, bracketed,
    parse::{Parse, ParseStream},
};

pub struct Callback {
    pub name: SynPath,
    pub pre: Group,
    pub pst: Group,
}

impl Parse for Callback {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<crate::kw::name>()?;
        input.parse::<Token![:]>()?;
        let name = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<crate::kw::pre>()?;
        input.parse::<Token![:]>()?;
        let pre = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<crate::kw::pst>()?;
        input.parse::<Token![:]>()?;
        let pst = input.parse()?;
        crate::maybe_trailing_comma(input)?;
        Ok(Callback { name, pre, pst })
    }
}

pub fn parse_callback(input: ParseStream) -> syn::Result<Callback> {
    input.parse::<crate::kw::callback>()?;
    input.parse::<Token![:]>()?;
    let callback;
    bracketed!(callback in input);
    callback.parse()
}
