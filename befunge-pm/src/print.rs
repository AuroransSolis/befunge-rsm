use crate::callback::Callback;
use interprocess::local_socket::Stream;
use syn::{LitChar, LitInt, Token, parse::{Parse, ParseStream}};

pub struct PrintInteger {
    pub number: isize,
    pub conn: Stream,
    pub callback: Callback,
}

impl Parse for PrintInteger {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<crate::kw::number>()?;
        input.parse::<Token![:]>()?;
        let number: LitInt = input.parse()?;
        let number: isize = number.base10_parse()?;
        input.parse::<Token![,]>()?;
        let conn = crate::interface::parse_socket(input)?;
        input.parse::<Token![,]>()?;
        let callback = crate::callback::parse_callback(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(PrintInteger {
            number,
            conn,
            callback,
        })
    }
}

pub struct PrintAscii {
    pub ascii: char,
    pub conn: Stream,
    pub callback: Callback,
}

impl Parse for PrintAscii {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<crate::kw::ascii>()?;
        input.parse::<Token![:]>()?;
        let ascii: LitChar = input.parse()?;
        let ascii: char = ascii.value();
        input.parse::<Token![,]>()?;
        let conn = crate::interface::parse_socket(input)?;
        input.parse::<Token![,]>()?;
        let callback = crate::callback::parse_callback(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(PrintAscii {
            ascii,
            conn,
            callback,
        })
    }
}
