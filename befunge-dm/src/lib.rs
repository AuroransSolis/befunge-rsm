// #![recursion_limit = "16384"]
#![feature(macro_metavar_expr)]
//! A Befunge 93 interpreter implemented in `macro_rules!` macros
//!
//! Yes, you read that right. This crate contains almost everything needed to be able to run Befunge
//! programs at compile time in declarative macros. There are a couple exceptions to this where a
//! procedural macro is absolutely required, hence [`befunge_pm`]. That crate is used for:
//!
//! - [`befunge_pm::stringify_with_callback!`]: makes a callback with the stringified input.
//! - [`befunge_pm::div_by_zero!`]: used to ask the user for input when division by zero occurs.
//! - [`befunge_pm::mod_by_zero!`]: used to ask the user for input when modulus by zero occurs.
//! - [`befunge_pm::socket_debug!`]: used when the `socket_debug_default` feature is enabled to
//!   output debugging information.
//! - [`befunge_pm::choose_random!`]: used for the `?` instruction - outputs a randomly selected
//!   token from the input tokens.
//! - [`befunge_pm::print_integer!`]: outputs an integer on the output socket.
//! - [`befunge_pm::print_ascii!`]: outputs an ASCII character on the output socket.
//! - [`befunge_pm::get_integer!`]: asks for user input of a single digit integer.
//! - [`befunge_pm::get_ascii!`]: asks for user input of a single ASCII character.
//! - [`befunge_pm::close_ui!`]: sends a signal to the input and output (and debug, if
//!   `socket_debug_default` is enabled) interfaces to close.
//! - [`befunge_pm::flush_output!`]: sends a signal to the output interface to flush its output
//!   buffer.
//! - [`befunge_pm::befunge_input!`]: reads a file and makes a callback with the file contents
//!   as a space-separated list of character literals.
//!
//! On that note, when running Befunge programs with _any_ input or output (from the `/`, `%`, `.`,
//! `,`, `&`, `?`, or `~` instructions), you must be running `befunge-if` on the corresponding
//! socket (either `befunge.output` or `befunge.input`).
//!
//! Running programs - which is done simply by building the program with `cargo check`,
//! `cargo build`, or `cargo expand` - requires the `#![feature(macro_metavar_expr)]` feature, and
//! typically also requires a higher-than-normal `#![recursion_limit = "..."]`. For some programs it
//! may also be necessary to provide `RUST_MIN_STACK=A_BIG_NUMBER`.
//!
//! For information on Befunge, it is recommended to refer to the following resources:
//!
//! - [The Befunge 93 specification](https://github.com/catseye/Befunge-93/blob/master/doc/Befunge-93.markdown)
//! - [The Esolang wiki page on Befunge](https://esolangs.org/wiki/Befunge)
//! - [The Wikipedia page on Befunge](https://en.wikipedia.org/wiki/Befunge)

#[macro_use]
pub mod arith;
#[macro_use]
mod debug;
#[macro_use]
mod error;
#[macro_use]
mod init;
#[macro_use]
mod list;
#[macro_use]
mod step;
#[macro_use]
mod stringify;
#[macro_use]
mod stringmode;

pub use befunge_pm;

#[macro_export]
/// Run a Befunge 93 program. The following are valid calling formats:
/// ```ignore
/// #![feature(macro_metavar_expr)]
///
/// befunge_dm::befunge! {
///     "example.bfg"
/// }
/// ```
/// ```ignore
/// #![feature(macro_metavar_expr)]
///
/// befunge_dm::befunge! {
///     file: "example.bfg"
/// }
/// ```
/// ```ignore
/// #![feature(macro_metavar_expr)]
///
/// befunge_dm::befunge! {
///     file: "example.bfg",
/// }
/// ```
/// ```
/// #![recursion_limit = "512"]
/// #![feature(macro_metavar_expr)]
///
/// befunge_dm::befunge! {
///     file: "example.bfg",
///     debug: [[noflush]],
/// }
/// ```
/// For purposes of the above doctest, `example.bfg` contains the following:
/// ```befunge
#[doc = include_str!("../../example.bfg")]
/// ```
/// The first three examples here are untested because any Befunge program that makes use of
/// instructions that can produce output or require input (the `/`, `%`, `.`, `,`, `&`, or `~`
/// instructions) requires that `befunge-if` be running on the `befunge.output` and `befunge.input`
/// sockets respectively. Additionally, this crate can be compiled with the `socket_debug_default`
/// feature, which will output debugging information on `befunge.debug` (and thus require another
/// `befunge-if` process running on that socket).
///
/// As you can see in the example, this program can accept debugging flags! Here are the recognised
/// flags:
///
/// - `[initlines]`: Output a `const _: &str = "..."` with the contents of each line of the
///   program as it is read in.
/// - `[postinit]`: Output a `const _: &str = "..."` with the contents of the program memory
///   once the whole program has been read in.
/// - `[getdbg]`: Output `const _: &str = "..."`s as the program performs `g` instructions.
/// - `[putdbg]`: Output `const _: &str = "..."`s as the program performs `p` instructions.
/// - `[closeonend]`: Send a signal to the I/O programs to close when the program exits (hits a
///   `@` instruction).
/// - `[poststack]`: Output the contents of the stack on exit (hitting a `@` instruction).
/// - `[noflush]`: Don't request interface programs to flush output on exit (hitting `@`
///   instruction).
///
/// Debugging flags should be given as a space-separated list.
macro_rules! befunge {
    ($(file: )?$file:literal$(,)?) => {
        const _: &str = concat!("Using Befunge file: '", $file, "'");
        $crate::befunge_pm::befunge_input! {
            file: $file,
            callback: [
                name: $crate::befunge_init,
                pre: [@init],
                pst: [
                    debug: [],
                ],
            ],
        }
    };
    (
        file: $file:literal,
        debug: $debug:tt,
    ) => {
        const _: &str = concat!("Using Befunge file: '", $file, "'");
        $crate::befunge_pm::befunge_input! {
            file: $file,
            callback: [
                name: $crate::befunge_init,
                pre: [@init],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
}

#[macro_export]
/// Defines an ad-hoc equality checking macro and immediately calls it. If the input is equal to
/// the sought token, then the contents of the `true` token tree are used for expansion. Otherwise,
/// the conents of the `false` token tree are used for expansion.
///
/// # Example
/// ```
/// #![feature(macro_metavar_expr)]
///
/// let foo = {
///     befunge_dm::def_eq! {
///         lookfor: [foo],
///         input: [foo],
///         true: [true],
///         false: [false],
///     }
/// };
///
/// assert!(foo);
///
/// let bar = {
///     befunge_dm::def_eq! {
///         lookfor: [foo],
///         input: [bar],
///         true: [true],
///         false: [false],
///     }
/// };
///
/// assert!(!bar);
/// ```
macro_rules! def_eq {
    (
        lookfor: [$lookfor:tt],
        input: [$input:tt],
        true: [$($true:tt)*],
        false: [$($false:tt)*],
    ) => {
        macro_rules! token_eq {
            ($lookfor) => {
                $($true)*
            };
            ($$($$_:tt)*) => {
                $($false)*
            };
        }

        token_eq! {
            $input
        }
    };
}
