/// Simple wrapper for defining an anonymous constant made by stringifying and then concatenating
/// all input tokens.
#[macro_export]
macro_rules! dbg_const {
    ($($tt:tt)*) => {
        const _: &str = concat!($(stringify!($tt)),*);
    };
}

/// Macro that looks for a certain debug token and then expands to the contents of a given token
/// tree if that token is found. Optionally, an "else" may be provided, which will be expanded
/// instead if the desired token is not found.
///
/// # Example
/// ```
/// #![feature(macro_metavar_expr)]
///
/// let foo = {
///     befunge_dm::dbg_maybe_expand! {
///         @dbg
///         debug: [[present_a] [present_b]],
///         lookfor: [[present_b]],
///         expand: [true],
///     }
/// };
///
/// assert!(foo);
///
/// let bar = {
///     befunge_dm::dbg_maybe_expand! {
///         @dbg
///         debug: [[present_a] [present_b]],
///         lookfor: [[not_present]],
///         expand: [false],
///         orelse: [true],
///     }
/// };
///
/// assert!(bar);
///
/// let baz = {
///     befunge_dm::dbg_maybe_expand! {
///         @dbg
///         debug: [[present_a] [present_b] [present_c]],
///         lookfor: [[present_c]],
///         expand: [true],
///         orelse: [false],
///     }
/// };
///
/// assert!(baz);
/// ```
///
/// Execution strategy:
///     1. Expand to an ad-hoc equality checking macro ([`crate::def_eq`]) that checks if the
///        head of the debug token tree list is equal to the token tree we are looking for. If yes,
///        then expand to the contents of the desired token tree. Otherwise, recurse with the rest
///        of the debug token trees.
///     2. If the debug token tree list is empty, expand to the contents of the "otherwise" token
///        if present.
#[macro_export]
macro_rules! dbg_maybe_expand {
    (
        @dbg
        debug: [$debugh:tt $($debugt:tt)*],
        lookfor: $lookfor:tt,
        expand: $expand:tt,
        $(orelse: $orelse:tt$(,)?)?
    ) => {
        $crate::def_eq! {
            lookfor: $lookfor,
            input: [$debugh],
            true: $expand,
            false: [
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: [$($debugt)*],
                    lookfor: $lookfor,
                    expand: $expand,
                    $(orelse: $orelse,)?
                }
            ],
        }
    };
    (
        @dbg
        debug: [],
        lookfor: $lookfor:tt,
        expand: $expand:tt,
        $(orelse: [$($orelse:tt)*]$(,)?)?
    ) => {
        $($($orelse)*)?
    };
}

/// Prints the stack of a Befunge program for debugging purposes.
#[macro_export]
macro_rules! dbg_print_stack {
    (
        @printstack
        stack: $stack:tt,
    ) => {
        $crate::dbg_print_stack! {
            @printstack @loop
            stack: $stack,
            tokens: [],
        }
    };
    (
        @printstack @loop
        stack: [],
        tokens: [],
    ) => {
        const _: &str = "Empty stack!";
    };
    (
        @printstack @loop
        stack: [],
        tokens: [[$hfst:tt$(, $hsnd:tt)?] $([$tfst:tt$(, $tsnd:tt)?])*],
    ) => {
        const _: &str = concat!(
            "top: ",
            $hfst,
            $(
                " (",
                $hsnd,
                ")",
            )?
        );
        $(
            const _: &str = concat!(
                "     ",
                $tfst,
                $(
                    " (",
                    $tsnd,
                    ")",
                )?
            );
        )*
    };
    (
        @printstack @loop
        stack: [$stackh:tt $($stackt:tt)*],
        tokens: $tokens:tt,
    ) => {
        $crate::code_to_char_pretty! {
            @match
            num: $stackh,
            callback: [
                name: $crate::dbg_print_stack,
                pre: [
                    @printstack @loopcatch
                    stack: [$($stackt)*],
                    tokens: $tokens,
                ],
                pst: [],
            ],
        }
    };
    (
        @printstack @loopcatch
        stack: $stack:tt,
        tokens: [$($token:tt)*],
        char: $char:tt,
    ) => {
        $crate::dbg_print_stack! {
            @printstack @loop
            stack: $stack,
            tokens: [$($token)* $char],
        }
    };
}

/// Converts a signed magnitude base 1 number in the representation used by the interpreter to a
/// literal number.
///
/// # Example
///
/// ```
/// #![feature(macro_metavar_expr)]
///
/// macro_rules! wrapper {
///     (num: $num:tt,) => { $num };
///     (num: -$num:tt,) => { -$num };
/// }
///
/// let foo = {
///     befunge_dm::dbg_get_number! {
///         num: [[neg] [[] [] [] [] [] [] []]],
///         callback: [
///             name: wrapper,
///             pre: [],
///             pst: [],
///         ],
///     }
/// };
///
/// assert_eq!(foo, -7);
/// ```
#[macro_export]
macro_rules! dbg_get_number {
    (
        num: [[$(pos)?] [$($num:tt)*]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            num: ${count($num)},
            $($pst)*
        }
    };
    (
        num: [[neg] [$($num:tt)*]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            num: -${count($num)},
            $($pst)*
        }
    };
}

#[cfg(feature = "socket_debug_default")]
/// Sends a message to the default debugging socket (`befunge.debug`).
#[macro_export]
macro_rules! socket_debug_default {
    ($($tt:tt)*) => {
        $crate::befunge_pm::socket_debug! {
            tokens: [$($tt)*],
            socket: "befunge.debug",
        }
    };
}

#[cfg(not(feature = "socket_debug_default"))]
/// Redefinition of `socket_debug_default` for when debugging is not desired. This simply consumes
/// all input tokens and expands to an empty tree.
#[macro_export]
macro_rules! socket_debug_default {
    ($($tt:tt)*) => {};
}
