#![feature(ascii_char)]

use befunge_if::Request;
use clap::Parser;
use interprocess::local_socket::{
    GenericFilePath, GenericNamespaced, Listener, ListenerOptions, Stream, prelude::*,
};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Result as IoResult, Write, stdin};

#[derive(Parser)]
struct Opts {
    #[arg(short, long)]
    socket: String,
}

fn main() -> IoResult<()> {
    let Opts { socket } = Opts::parse();
    println!("Using socket name: '{socket}'");
    let name = if GenericNamespaced::is_supported() {
        socket.to_ns_name::<GenericNamespaced>()?
    } else {
        format!("/tmp/{socket}").to_fs_name::<GenericFilePath>()?
    };
    println!("Created socket path: '{name:?}'");
    let mut lstn = ListenerOptions::new().name(name).create_sync()?;
    println!("Successfully connected to socket.");
    await_open_connection(&mut lstn)
}

fn await_open_connection(lstn: &mut Listener) -> IoResult<()> {
    let mut buf = String::new();
    let res = loop {
        match lstn.accept() {
            Ok(mut conn) => {
                let close = run_connection(&mut conn, &mut buf)?;
                if close {
                    break Ok(());
                }
            }
            Err(err) => {
                let msg = format!("Error while attempting to accept connections: '{err}'");
                break Err(IoError::new(IoErrorKind::Other, msg));
            }
        }
    };
    if !buf.is_empty() {
        println!("{buf}");
    }
    res
}

fn run_connection(mut conn: &mut Stream, buf: &mut String) -> IoResult<bool> {
    let mut expecting_ack = false;
    loop {
        match ciborium::de::from_reader(&mut conn) {
            Ok(Request::DivByZero) => {
                if !buf.is_empty() {
                    print!("{buf}");
                    buf.clear();
                }
                expecting_ack = div_by_zero(&mut conn)?;
            }
            Ok(Request::ModByZero) => {
                if !buf.is_empty() {
                    print!("{buf}");
                    buf.clear();
                }
                expecting_ack = mod_by_zero(&mut conn)?;
            }
            Ok(Request::PrintInteger(num)) => {
                buf.push_str(&format!("{num}"));
                ciborium::ser::into_writer(&Request::Ack, &mut conn).map_err(
                    |err| {
                        IoError::new(
                            IoErrorKind::Other,
                            format!("Error sending ack response: '{err}'"),
                        )
                    },
                )?;
            }
            Ok(Request::PrintAscii(c)) => {
                // println!("got print req: {c:?} ({:?})", c as char);
                if c == b'\n' {
                    println!("{buf}");
                    buf.clear();
                } else {
                    let c_ascii = std::ascii::Char::from_u8(c).unwrap();
                    buf.push(c_ascii.to_char());
                }
                ciborium::ser::into_writer(&Request::Ack, &mut conn).map_err(
                    |err| {
                        IoError::new(
                            IoErrorKind::Other,
                            format!("Error sending ack response: '{err}'"),
                        )
                    },
                )?;
            }
            Ok(Request::GetInteger) => {
                if !buf.is_empty() {
                    print!("{buf}");
                    buf.clear();
                }
                expecting_ack = ask_for_integer(&mut conn)?;
            }
            Ok(Request::GetAscii) => {
                if !buf.is_empty() {
                    print!("{buf}");
                    buf.clear();
                }
                expecting_ack = ask_for_ascii(&mut conn)?;
            }
            Ok(Request::FlushOutput) => {
                // println!("received flush");
                if !buf.is_empty() {
                    println!("{buf}");
                    buf.clear();
                }
                ciborium::ser::into_writer(&Request::Ack, &mut conn).map_err(
                    |err| {
                        IoError::new(
                            IoErrorKind::Other,
                            format!("Error sending ack response: '{err}'"),
                        )
                    },
                )?;
            }
            Ok(Request::Debug(contents)) => {
                println!("DEBUG: {contents}");
                ciborium::ser::into_writer(&Request::Ack, &mut conn).map_err(
                    |err| {
                        IoError::new(
                            IoErrorKind::Other,
                            format!("Error sending ack response: '{err}'"),
                        )
                    },
                )?;
            }
            Ok(Request::Ack) if expecting_ack => expecting_ack = false,
            Ok(Request::CloseUi) => return Ok(true),
            Ok(Request::CloseConnection) => return Ok(false),
            Ok(other) => {
                println!("Received unexpected request: '{other:?}'");
                return ciborium::ser::into_writer(&Request::Nack, &mut conn)
                    .map_err(|err| {
                        IoError::new(
                            IoErrorKind::Other,
                            format!("Error replying to client with ACK: '{err}'"),
                        )
                    })
                    .and_then(|_| conn.flush())
                    .map(|_| false);
            }
            Err(err) => {
                let msg = format!("Error while reading from data stream: '{err}'");
                break Err(IoError::new(IoErrorKind::Other, msg));
            }
        }
    }
}

fn prompt_for_integer() -> IoResult<isize> {
    let mut linebuf = String::new();
    loop {
        stdin().read_line(&mut linebuf)?;
        match linebuf.trim().parse::<isize>() {
            Ok(val) => break Ok(val),
            Err(err) => {
                println!("Error reading value: '{err}'");
                println!("Please try again:");
                linebuf.clear();
            }
        }
    }
}

fn div_by_zero(mut conn: &mut Stream) -> IoResult<bool> {
    println!("Attempted to divide by 0! What do you want the result to be?");
    let val = prompt_for_integer()?;
    ciborium::ser::into_writer(&Request::DivByZeroAns(val), &mut conn).map_err(
        |err| {
            IoError::new(
                IoErrorKind::Other,
                format!("Error sending back divide by zero response: '{err}'"),
            )
        },
    )?;
    conn.flush()?;
    Ok(true)
}

fn mod_by_zero(mut conn: &mut Stream) -> IoResult<bool> {
    println!("Attempted take a modulus with respect to 0! What do you want the result to be?");
    let val = prompt_for_integer()?;
    ciborium::ser::into_writer(&Request::ModByZeroAns(val), &mut conn).map_err(
        |err| {
            IoError::new(
                IoErrorKind::Other,
                format!("Error sending back modulus by zero response: '{err}'"),
            )
        },
    )?;
    conn.flush()?;
    Ok(true)
}

fn ask_for_integer(mut conn: &mut Stream) -> IoResult<bool> {
    println!("Please enter an integer:");
    let val = prompt_for_integer()?;
    ciborium::ser::into_writer(&Request::GetIntegerAns(val), &mut conn).map_err(
        |err| {
            IoError::new(
                IoErrorKind::Other,
                format!("Error sending back integer response: '{err}'"),
            )
        },
    )?;
    conn.flush()?;
    Ok(true)
}

fn prompt_for_char() -> IoResult<u8> {
    let mut linebuf = String::new();
    loop {
        stdin().read_line(&mut linebuf)?;
        if linebuf.starts_with("\\x")
            && linebuf.trim().len() == 4
            && linebuf
                .trim()
                .chars()
                .skip(2)
                .all(|c| c.is_ascii_hexdigit())
        {
            let &[_, _, u, l] = linebuf.as_bytes() else {
                unreachable!();
            };
            let u = match u {
                b'0'..=b'9' => u - b'0',
                b'a'..=b'f' => u - b'a',
                b'A'..=b'F' => u - b'A',
                _ => unreachable!(),
            };
            let l = match l {
                b'0'..=b'9' => l - b'0',
                b'a'..=b'f' => l - b'a',
                b'A'..=b'F' => l - b'A',
                _ => unreachable!(),
            };
            let c = u * 16 + l;
            if c.is_ascii() {
                break Ok(c);
            } else {
                println!("Entered value '{c}' is not valid ASCII! Please try again:");
                linebuf.clear();
            }
        } else {
            match linebuf.trim().parse::<char>() {
                Ok(c) if c.is_ascii() => break Ok(c as u8),
                Ok(c) => {
                    println!("Entered value '{c}' is not valid ASCII! Please try again:");
                    linebuf.clear();
                }
                Err(err) => {
                    println!("Error reading value: '{err}'");
                    println!("Please try again:");
                    linebuf.clear();
                }
            }
        }
    }
}

fn ask_for_ascii(mut conn: &mut Stream) -> IoResult<bool> {
    println!("Please enter an ASCII character (\\x00 format or literal):");
    let val = prompt_for_char()?;
    ciborium::ser::into_writer(&Request::GetAsciiAns(val), &mut conn).map_err(
        |err| {
            IoError::new(
                IoErrorKind::Other,
                format!("Error sending back ASCII response: '{err}'"),
            )
        },
    )?;
    conn.flush()?;
    Ok(true)
}
