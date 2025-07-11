use interprocess::local_socket::Stream;
use proc_macro2::Group;
use syn::{parse::Parse, Token};

pub struct Debug {
    pub tokens: Group,
    pub conn: Stream,
}

impl Parse for Debug {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<crate::kw::tokens>()?;
        input.parse::<Token![:]>()?;
        let tokens = input.parse()?;
        input.parse::<Token![,]>()?;
        let conn = crate::interface::parse_socket(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(Debug { tokens, conn })
    }
}
