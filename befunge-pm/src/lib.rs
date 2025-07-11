#![feature(proc_macro_def_site, proc_macro_diagnostic)]

extern crate proc_macro;

mod callback;
mod debug;
mod input;
mod interface;
mod print;
mod random_token;
mod stringify_callback;

use befunge_if::Request;
use callback::Callback;
use debug::Debug;
use input::BefungeInput;
use interface::{CloseUi, InterfaceConn, isize_to_base1};
use print::{PrintAscii, PrintInteger};
use proc_macro::{Span, TokenStream};
use proc_macro2::{Group, Literal, TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::quote;
use rand::{SeedableRng, rngs::StdRng, seq::IndexedRandom};
use random_token::ChooseRandom;
use std::{io::Write, path::PathBuf};
use stringify_callback::StringifyCallback;
use syn::{
    Error as SynError, Token,
    parse::ParseStream,
    parse_macro_input,
};

fn maybe_trailing_comma(input: ParseStream) -> syn::Result<()> {
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }
    if !input.is_empty() {
        Err(SynError::new(input.span(), "Unexpected input"))
    } else {
        Ok(())
    }
}

macro_rules! do_or_err {
    ($msg:literal, $do:expr$(,)?) => {
        if let Err(err) = $do {
            let msg = format!(concat!($msg, "\nError: {}"), err);
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    };
}

mod kw {
    syn::custom_keyword!(ascii);
    syn::custom_keyword!(callback);
    syn::custom_keyword!(choices);
    syn::custom_keyword!(file);
    syn::custom_keyword!(name);
    syn::custom_keyword!(neg);
    syn::custom_keyword!(number);
    syn::custom_keyword!(pos);
    syn::custom_keyword!(pre);
    syn::custom_keyword!(pst);
    syn::custom_keyword!(socket);
    syn::custom_keyword!(tokens);
}

#[proc_macro]
/// Reads in an input file and makes a callback with a stream of character literals as the result.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     filecontents: ['a' 'b' 'c' ...],
///     pst
/// }
/// ```
pub fn befunge_input(input: TokenStream) -> TokenStream {
    let BefungeInput { file, callback } = parse_macro_input!(input as BefungeInput);
    let file_string = file.value();
    let file_path = PathBuf::from(&file_string);
    if !file_path.exists() {
        let msg = file_path
            .is_relative()
            .then_some(())
            .and(std::env::current_dir().ok())
            .map(|pwd| {
                format!(
                    "File '{}' does not exist ({}/{0})",
                    file_path.display(),
                    pwd.display()
                )
            })
            .unwrap_or_else(|| format!("File '{}' does not exist", file_path.display()));
        file.span().unwrap().error(msg).emit();
        return TokenStream::new();
    }
    let contents = match std::fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(err) => {
            let msg = file_path
                .canonicalize()
                .ok()
                .map(|canon| format!("Error reading file contents: {err} ({})", canon.display()))
                .unwrap_or_else(|| format!("Error reading file contents: {err}"));
            file.span().unwrap().error(&msg).emit();
            return TokenStream::new();
        }
    };
    let contents_ts = TokenStream2::from_iter(contents.chars().map(|c| {
        if c.is_ascii() {
            TokenTree2::Literal(Literal::character(c))
        } else {
            let path = file_path
                .canonicalize()
                .ok()
                .map(|canon| canon.display().to_string())
                .unwrap_or_else(|| file_path.display().to_string());
            let msg = format!("File {path} contains non-ASCII character: {c:?}");
            file.span().unwrap().error(&msg).emit();
            return TokenTree2::Group(Group::new(
                proc_macro2::Delimiter::None,
                TokenStream2::new(),
            ));
        }
    }));
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            filecontents: [#contents_ts],
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Similar to [`stringify`], but capable of making a callback with the result.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     stringified: "string",
///     pst
/// }
/// ```
pub fn stringify_with_callback(ts: TokenStream) -> TokenStream {
    let StringifyCallback { tokens, callback } = parse_macro_input!(ts as StringifyCallback);
    let tokens_string = tokens.stream().to_string();
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            stringified: #tokens_string,
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Called by the interpreter when division by 0 occurs. Prompts the input interface for a response.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     res: [[sgn] [mag]],
///     pst
/// }
/// ```
pub fn div_by_zero(input: TokenStream) -> TokenStream {
    let InterfaceConn { mut conn, callback } = parse_macro_input!(input as InterfaceConn);
    do_or_err!(
        "Failed to request divide by zero resolution from Befunge UI.",
        befunge_if::ciborium::ser::into_writer(&Request::DivByZero, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let ans = match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::DivByZeroAns(ans)) => ans,
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to deserialise message.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    };
    do_or_err!(
        "Failed to write close connection.",
        befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let res = isize_to_base1(ans);
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            res: #res,
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Called by the interpreter when modulus by 0 occurs. Prompts the input interface for a response.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     res: [[sgn] [mag]],
///     pst
/// }
/// ```
pub fn mod_by_zero(input: TokenStream) -> TokenStream {
    let InterfaceConn { mut conn, callback } = parse_macro_input!(input as InterfaceConn);
    do_or_err!(
        "Failed to request modulus by zero resolution from Befunge UI.",
        befunge_if::ciborium::ser::into_writer(&Request::ModByZero, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let ans = match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::ModByZeroAns(ans)) => ans,
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to deserialise message.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    };
    do_or_err!(
        "Failed to write close connection.",
        befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let res = isize_to_base1(ans);
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            res: #res,
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Expands to a random token from its input.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     rand: tt,
///     pst
/// }
/// ```
pub fn choose_random(input: TokenStream) -> TokenStream {
    let ChooseRandom { choices, callback } = parse_macro_input!(input as ChooseRandom);
    let mut rng = StdRng::from_os_rng();
    let choices = choices.into_iter().collect::<Vec<_>>();
    let choice = TokenStream2::from(choices.choose(&mut rng).unwrap().clone());
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            rand: #choice,
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Prints out an integer over the socket described by the input.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     pst
/// }
/// ```
pub fn print_integer(input: TokenStream) -> TokenStream {
    let PrintInteger {
        number,
        mut conn,
        callback,
    } = parse_macro_input!(input as PrintInteger);
    do_or_err!(
        "Failed to send integer to Befunge UI",
        befunge_if::ciborium::ser::into_writer(&Request::PrintInteger(number), &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::Ack) => {
            do_or_err!(
                "Failed to send close connection to Befunge UI",
                befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
            );
            do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
        }
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to read response from Befunge UI.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    }
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Prints out an ASCII character over the socket described by the input.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     pst
/// }
/// ```
pub fn print_ascii(input: TokenStream) -> TokenStream {
    let PrintAscii {
        ascii,
        mut conn,
        callback,
    } = parse_macro_input!(input as PrintAscii);
    do_or_err!(
        "Failed to send integer to Befunge UI",
        befunge_if::ciborium::ser::into_writer(&Request::PrintAscii(ascii as u8), &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::Ack) => {
            do_or_err!(
                "Failed to send close connection to Befunge UI",
                befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
            );
            do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
        }
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to read response from Befunge UI.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    }
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Requests the specified socket to flush its output buffer.
pub fn flush_output(input: TokenStream) -> TokenStream {
    let CloseUi { mut conn } = parse_macro_input!(input as CloseUi);
    do_or_err!(
        "Failed to send output flush request",
        befunge_if::ciborium::ser::into_writer(&Request::FlushOutput, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::Ack) => {
            do_or_err!(
                "Failed to send close connection to Befunge UI",
                befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
            );
            do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
        }
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to read response from Befunge UI.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    }
    TokenStream::new()
}

#[proc_macro]
/// Sends a request for the interface program on the other side of the specified socket to exit.
pub fn close_ui(input: TokenStream) -> TokenStream {
    let CloseUi { mut conn } = parse_macro_input!(input as CloseUi);
    do_or_err!(
        "Failed to send close UI request",
        befunge_if::ciborium::ser::into_writer(&Request::CloseUi, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    TokenStream::new()
}

#[proc_macro]
/// Sends a request for a single digit integer input over the specified socket.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     integer: [[sgn] [mag]],
///     pst
/// }
/// ```
pub fn get_integer(input: TokenStream) -> TokenStream {
    let InterfaceConn { mut conn, callback } = parse_macro_input!(input as InterfaceConn);
    do_or_err!(
        "Failed to request integer from Befunge UI.",
        befunge_if::ciborium::ser::into_writer(&Request::GetInteger, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let ans = match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::GetIntegerAns(ans)) => ans,
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to deserialise message.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    };
    do_or_err!(
        "Failed to write close connection.",
        befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let res = isize_to_base1(ans);
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            integer: #res,
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Sends a request for a single ASCII character input over the specified socket.
/// 
/// The callback format is:
/// ```ignore
/// name! {
///     pre
///     ascii: 'a',
///     pst
/// }
/// ```
pub fn get_ascii(input: TokenStream) -> TokenStream {
    let InterfaceConn { mut conn, callback } = parse_macro_input!(input as InterfaceConn);
    do_or_err!(
        "Failed to request character from Befunge UI.",
        befunge_if::ciborium::ser::into_writer(&Request::GetAscii, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let ans = match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::GetAsciiAns(ans)) => ans,
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to deserialise message.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    };
    do_or_err!(
        "Failed to write close connection.",
        befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
    );
    do_or_err!("Failed to flush buffer to Befunge UI", conn.flush());
    let res = TokenTree2::Literal(Literal::character(ans as char));
    let Callback { name, pre, pst } = callback;
    let pre_inner = pre.stream();
    let pst_inner = pst.stream();
    let expanded = quote! {
        #name! {
            #pre_inner
            ascii: #res,
            #pst_inner
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
/// Converts the input tokens to a string and sends them to the specified socket.
pub fn socket_debug(input: TokenStream) -> TokenStream {
    let Debug { tokens, mut conn } = parse_macro_input!(input as Debug);
    let tokens = tokens.to_string();
    do_or_err!(
        "Failed to send debug request to Befunge UI.",
        befunge_if::ciborium::ser::into_writer(&Request::Debug(tokens), &mut conn),
    );
    match befunge_if::ciborium::de::from_reader(&mut conn) {
        Ok(Request::Ack) => (),
        Ok(other) => {
            let msg = format!("Received unexpected request: '{other:?}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
        Err(err) => {
            let msg = format!("Failed to deserialise message.\nError: '{err}'");
            Span::call_site().error(&msg).emit();
            return TokenStream::new();
        }
    };
    do_or_err!(
        "Failed to write close connection.",
        befunge_if::ciborium::ser::into_writer(&Request::CloseConnection, &mut conn),
    );
    TokenStream::new()
}
