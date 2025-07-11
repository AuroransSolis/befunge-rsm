use crate::callback::Callback;
use interprocess::local_socket::{GenericFilePath, GenericNamespaced, Stream, prelude::*};
use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::quote;
use std::iter::repeat_n;
use syn::{Error as SynError, LitStr, Token, parse::{Parse, ParseStream}};

pub struct InterfaceConn {
    pub conn: Stream,
    pub callback: Callback,
}

impl Parse for InterfaceConn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let conn = parse_socket(input)?;
        input.parse::<Token![,]>()?;
        let callback = crate::callback::parse_callback(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(InterfaceConn { conn, callback })
    }
}

pub fn parse_socket(input: ParseStream) -> syn::Result<Stream> {
    input.parse::<crate::kw::socket>()?;
    input.parse::<Token![:]>()?;
    let socket: LitStr = input.parse()?;
    let socket = socket.value();
    let name = if GenericNamespaced::is_supported() {
        socket
            .to_ns_name::<GenericNamespaced>()
            .map_err(|e| SynError::new(input.span(), format!("{e}")))?
    } else {
        format!("/tmp/{socket}")
            .to_fs_name::<GenericFilePath>()
            .map_err(|e| SynError::new(input.span(), format!("{e}")))?
    };
    let conn =
        Stream::connect(name).map_err(|e| SynError::new(input.span(), format!("{e}")))?;
    Ok(conn)
}

fn empty_group() -> TokenTree2 {
    TokenTree2::Group(Group::new(Delimiter::Bracket, TokenStream2::new()))
}

pub fn isize_to_base1(num: isize) -> TokenStream2 {
    let groups = TokenStream2::from_iter(repeat_n(empty_group(), num.abs() as usize));
    let stream = if num.is_negative() {
        quote! {
            [[neg] [#groups]]
        }
    } else {
        quote! {
            [[pos] [#groups]]
        }
    };
    TokenStream2::from(stream)
}

pub struct CloseUi {
    pub conn: Stream,
}

impl Parse for CloseUi {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let conn = parse_socket(input)?;
        crate::maybe_trailing_comma(input)?;
        Ok(CloseUi { conn })
    }
}
