//! Macros for doing signed magnitude base 1 arithmetic.
//!
//! # Conventions
//!
//! First, numbers are represented as `[[sign] [magnitude]]`. The sign of a number can be empty,
//! `pos`, or `neg`. Empty and `pos` are treated equivalently. The value of `[magnitude]` is
//! equivalent to the number of `:tt`s in it, though `[]`s were used throughout this crate as the
//! only "digit" in these base 1 numbers. Additionally, zero should always be positive - that is,
//! `[[neg] []]` should never be allowed to occur. Where possible, this crate attempts to correct
//! such occurrences, though it may still cause errors. Here are some examples of valid numbers:
//!   - `[[pos] []]`: `0`
//!   - `[[] []]`: `0`
//!   - `[[pos] [[] [] [] [] []]]`: `5`
//!   - `[[neg] [[] []]]`: `-2`
//!
//! Second, the calling convention for these macros is generally consistent, except for
//! [`crate::arith_div_mod`]. These chould be called as:
//! ```ignore
//! befunge_dm::arith_opname! {
//!     @opname
//!     a: [[asgn] [amag]],
//!     b: [[bsgn] [bmag]],
//!     callback: [
//!         name: callback_name,
//!         pre: [anything you want before the result],
//!         pst: [anything you want after the result],
//!     ],
//! }
//! ```
//! This will result in this callback:
//! ```ignore
//! callback_name! {
//!     anything you want before the result
//!     res: [[ressgn] [resmag]],
//!     anything you want after the result
//! }
//! ```
//! The sole exception to this is [`crate::arith_div_mod`]. Please refer to the documentation for
//! that macro if you wish to call it by itself for some reason.

/// Add two signed magnitude base 1 numbers.
///
/// Examples:
/// ```
/// #![feature(macro_metavar_expr)]
///
/// macro_rules! num_to_lit {
///     ([[$(pos)?] [$($num:tt)*]]) => {
///         ${count($num)}
///     };
///     ([[neg] [$($num:tt)*]]) => {
///         -${count($num)}
///     };
/// }
///
/// macro_rules! wrapper {
///     (
///         a: $a:tt,
///         b: $b:tt,
///     ) => {{
///         befunge_dm::arith_add! {
///             @add
///             a: $a,
///             b: $b,
///             callback: [
///                 name: wrapper,
///                 pre: [],
///                 pst: [],
///             ],
///         }
///     }};
///     (
///         res: $res:tt,
///     ) => {
///         num_to_lit!($res)
///     };
/// }
///
/// const _: () = {
///     // Cases that should be handled by `arith_add!` directly:
///     // 0 + 0 = 0
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[pos] []],
///     );
///     assert!(tmp == 0);
///     // 0 + 3 = 3
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[pos] [[] [] []]],
///     );
///     assert!(tmp == 3);
///     // 2 + 0 = 2
///     let tmp = wrapper!(
///         a: [[pos] [[] []]],
///         b: [[pos] []],
///     );
///     assert!(tmp == 2);
///     // 5 + 6 = 11
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] [] []]],
///         b: [[pos] [[] [] [] [] [] []]],
///     );
///     assert!(tmp == 11);
///     // 0 + (-3) = -3
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[neg] [[] [] []]],
///     );
///     assert!(tmp == -3);
///     // -2 + 0 = -2
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[pos] []],
///     );
///     assert!(tmp == -2);
///     // -5 + (-6) = -11
///     let tmp = wrapper!(
///         a: [[neg] [[] [] [] [] []]],
///         b: [[neg] [[] [] [] [] [] []]],
///     );
///     assert!(tmp == -11);
///
///     // Cases that should be deferred to `arith_sub`:
///     // 1 + (-2) = -1
///     let tmp = wrapper!(
///         a: [[pos] [[]]],
///         b: [[neg] [[] []]],
///     );
///     assert!(tmp == -1);
///     // -2 + 3 = 1
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[pos] [[] [] []]],
///     );
///     assert!(tmp == 1);
/// };
/// ```
///
/// Execution strategy:
///   1. Check for easy cases (adding 0)
///   2. Defer cases of differing signs to [`crate::arith_sub`]
///   3. Result is equal to appending the magnitude of `a` to the magnitude of `b`. Keep the sign of
///      either number.
///   4. Expand callback with result.
#[macro_export]
macro_rules! arith_add {
    // a + 0
    (
        @add
        a: [$asgn:tt $a:tt],
        b: [$bsgn:tt []],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [$asgn $a],
            $($pst)*
        }
    };
    // 0 + b
    (
        @add
        a: [$asgn:tt []],
        b: [$bsgn:tt $b:tt],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [$bsgn $b],
            $($pst)*
        }
    };
    // a + b
    (
        @add
        a: [[$(pos)?] [$($a:tt)*]],
        b: [[$(pos)?] [$($b:tt)*]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[pos] [$($a)* $($b)*]],
            $($pst)*
        }
    };
    // a + (-b) = a - b
    (
        @add
        a: [[$(pos)?] $a:tt],
        b: [[neg] $b:tt],
        callback: $callback:tt,
    ) => {
        $crate::arith_sub! {
            @sub
            a: [[pos] $a],
            b: [[pos] $b],
            callback: $callback,
        }
    };
    // (-a) + b = b + (-a) = b - a
    (
        @add
        a: [[neg] $a:tt],
        b: [[$(pos)?] $b:tt],
        callback: $callback:tt,
    ) => {
        $crate::arith_sub! {
            @sub
            a: [[pos] $b],
            b: [[pos] $a],
            callback: $callback,
        }
    };
    // (-a) + (-b) = -(a + b)
    (
        @add
        a: [[neg] [$($a:tt)*]],
        b: [[neg] [$($b:tt)*]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[neg] [$($a)* $($b)*]],
            $($pst)*
        }
    };
}

/// Subtract two signed magnitude base 1 numbers.
///
/// Examples:
/// ```
/// #![feature(macro_metavar_expr)]
///
/// macro_rules! num_to_lit {
///     ([[$(pos)?] [$($num:tt)*]]) => {
///         ${count($num)}
///     };
///     ([[neg] [$($num:tt)*]]) => {
///         -${count($num)}
///     };
/// }
///
/// macro_rules! wrapper {
///     (
///         a: $a:tt,
///         b: $b:tt,
///     ) => {{
///         befunge_dm::arith_sub! {
///             @sub
///             a: $a,
///             b: $b,
///             callback: [
///                 name: wrapper,
///                 pre: [],
///                 pst: [],
///             ],
///         }
///     }};
///     (
///         res: $res:tt,
///     ) => {
///         num_to_lit!($res)
///     };
/// }
///
/// const _: () = {
///     // Cases that should be handled by `arith_add!` directly:
///     // 0 - 0 = 0
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[pos] []],
///     );
///     assert!(tmp == 0);
///     // 0 - 3 = -3
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[pos] [[] [] []]],
///     );
///     assert!(tmp == -3);
///     // 2 - 0 = 2
///     let tmp = wrapper!(
///         a: [[pos] [[] []]],
///         b: [[pos] []],
///     );
///     assert!(tmp == 2);
///     // -2 - 0 = -2
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[pos] []],
///     );
///     assert!(tmp == -2);
///     // 5 - 6 = -1
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] [] []]],
///         b: [[pos] [[] [] [] [] [] []]],
///     );
///     assert!(tmp == -1);
///     // 6 - 5 = 1
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] [] [] []]],
///         b: [[pos] [[] [] [] [] []]],
///     );
///     assert!(tmp == 1);
///
///     // Cases that should be deferred to `arith_add`:
///     // 0 - (-3) = 3
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[neg] [[] [] []]],
///     );
///     assert!(tmp == 3);
///     // 1 - (-2) = 3
///     let tmp = wrapper!(
///         a: [[pos] [[]]],
///         b: [[neg] [[] []]],
///     );
///     assert!(tmp == 3);
///     // -2 - 3 = -5
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[pos] [[] [] []]],
///     );
///     assert!(tmp == -5);
///     // -5 - (-6) = 1
///     let tmp = wrapper!(
///         a: [[neg] [[] [] [] [] []]],
///         b: [[neg] [[] [] [] [] [] []]],
///     );
///     assert!(tmp == 1);
/// };
/// ```
///
/// Execution strategy:
///   1. Check for easy cases (subtracting 0, subtracting from 0)
///   2. Defer to [`arith_add`] where possible.
///   3. Define an ad-hoc appropriate for the remaining cases:
///      a. For `a - b`, define a macro with two branches. The first branch assumes that `a >= b`,
///         and as such `a`'s magnitude can be matched as `b` repetitions of `[]`, plus an unknown
///         (`a - b`) number of additional token trees. This expands to a result of
///         `[[pos] [a - b]]`. The second branch assumes `a < b`, and as such `b` can be matched as
///         `a` repetitions of `[]` plus an unknown (`b - a`) number of additional token trees. This
///         expands to a result of `[[neg] [b - a]]`.
///      b. For `(-a) - (-b) = (-a) + b = b - a`, define a macro very similar to the one described
///         just above, but flip the branches.
///   4. Expand callback with result.
#[macro_export]
macro_rules! arith_sub {
    // a - 0
    (
        @sub
        a: [$asgn:tt $a:tt],
        b: [$bsgn:tt []],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [$asgn $a],
            $($pst)*
        }
    };
    // 0 - b
    (
        @sub
        a: [$asgn:tt []],
        b: [[$(pos)?] $b:tt],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[neg] $b],
            $($pst)*
        }
    };
    // 0 - (-b)
    (
        @sub
        a: [$asgn:tt []],
        b: [[neg] $b:tt],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[pos] $b],
            $($pst)*
        }
    };
    // a - b
    (
        @sub
        a: [[$(pos)?] [$($a:tt)*]],
        b: [[$(pos)?] [$($b:tt)*]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        macro_rules! exec_sub {
            (
                @sub
                a: [$($b)* $$($$diff:tt)*],
                b: $$_:tt,
            ) => {
                $name! {
                    $($pre)*
                    res: [[pos] [$$($$diff)*]],
                    $($pst)*
                }
            };
            (
                @sub
                a: $$_:tt,
                b: [$($a)* $$($$diff:tt)*],
            ) => {
                $name! {
                    $($pre)*
                    res: [[neg] [$$($$diff)*]],
                    $($pst)*
                }
            };
        }
        exec_sub! {
            @sub
            a: [$($a)*],
            b: [$($b)*],
        }
    };
    // a - (-b) = a + b
    (
        @sub
        a: [[$(pos)?] $a:tt],
        b: [[neg] $b:tt],
        callback: $callback:tt,
    ) => {
        $crate::arith_add! {
            @add
            a: [[pos] $a],
            b: [[pos] $b],
            callback: $callback,
        }
    };
    // (-a) - b = (-a) + (-b)
    (
        @sub
        a: [[neg] $a:tt],
        b: [[$(pos)?] $b:tt],
        callback: $callback:tt,
    ) => {
        $crate::arith_add! {
            @add
            a: [[neg] $a],
            b: [[neg] $b],
            callback: $callback,
        }
    };
    // (-a) - (-b) = (-a) + b = b - a
    (
        @sub
        a: [[neg] [$($a:tt)*]],
        b: [[neg] [$($b:tt)*]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        macro_rules! exec_sub {
            (
                @sub
                a: [$($b)* $$($$diff:tt)*],
                b: $$_:tt,
            ) => {
                $name! {
                    $($pre)*
                    res: [[neg] [$$($$diff)*]],
                    $($pst)*
                }
            };
            (
                @sub
                a: $$_:tt,
                b: [$($a)* $$($$diff:tt)*],
            ) => {
                $name! {
                    $($pre)*
                    res: [[pos] [$$($$diff)*]],
                    $($pst)*
                }
            };
        }
        exec_sub! {
            @sub
            a: [$($a)*],
            b: [$($b)*],
        }
    };
}

/// Multiplies two signed magnitude base 1 numbers
///
/// Examples:
/// ```
/// #![feature(macro_metavar_expr)]
///
/// macro_rules! num_to_lit {
///     ([[$(pos)?] [$($num:tt)*]]) => {
///         ${count($num)}
///     };
///     ([[neg] [$($num:tt)*]]) => {
///         -${count($num)}
///     };
/// }
///
/// macro_rules! wrapper {
///     (
///         a: $a:tt,
///         b: $b:tt,
///     ) => {{
///         befunge_dm::arith_mul! {
///             @mul
///             a: $a,
///             b: $b,
///             callback: [
///                 name: wrapper,
///                 pre: [],
///                 pst: [],
///             ],
///         }
///     }};
///     (
///         res: $res:tt,
///     ) => {
///         num_to_lit!($res)
///     };
/// }
///
/// const _: () = {
///     // 0 * 0 = 0
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[pos] []],
///     );
///     assert!(tmp == 0);
///     // 0 * 4 = 0
///     let tmp = wrapper!(
///         a: [[pos] []],
///         b: [[pos] [[] [] [] []]],
///     );
///     assert!(tmp == 0);
///     // 3 * 4 = 12
///     let tmp = wrapper!(
///         a: [[pos] [[] [] []]],
///         b: [[pos] [[] [] [] []]],
///     );
///     assert!(tmp == 12);
///     // (-2) * (-6) = 12
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[neg] [[] [] [] [] [] []]],
///     );
///     assert!(tmp == 12);
///     // (-3) * 3 = -9
///     let tmp = wrapper!(
///         a: [[neg] [[] [] []]],
///         b: [[pos] [[] [] []]],
///     );
///     assert!(tmp == -9);
///     // 4 * (-2) = -8
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] []]],
///         b: [[neg] [[] []]],
///     );
///     assert!(tmp == -8);
/// };
/// ```
///
/// Execution strategy:
///   1. Using features from `macro_metavar_expr`, we can simply repeat the magnitude of one number
///      a number of times equal to the number of token trees in the other number's magnitude. The
///      only thing we have to handle manually is the sign of the result. No special casing is
///      necessary - all cases are equally easy to handle.
///   2. Expand the callback with the result.
#[macro_export]
macro_rules! arith_mul {
    (
        @mul
        a: [[$(pos)?] [$($a:tt)*]],
        b: [[$(pos)?] $b:tt],
        callback: $callback:tt,
    ) => {
        $crate::arith_mul! {
            @catch
            res: [[pos] $(${ignore($a)}$b)*],
            callback: $callback,
        }
    };
    (
        @mul
        a: [[neg] [$($a:tt)*]],
        b: [[neg] $b:tt],
        callback: $callback:tt,
    ) => {
        $crate::arith_mul! {
            @catch
            res: [[pos] $(${ignore($a)}$b)*],
            callback: $callback,
        }
    };
    (
        @mul
        a: [[$($asgn:tt)?] [$($a:tt)*]],
        b: [[$($bsgn:tt)?] $b:tt],
        callback: $callback:tt,
    ) => {
        $crate::arith_mul! {
            @catch
            res: [[neg] $(${ignore($a)}$b)*],
            callback: $callback,
        }
    };
    (
        @catch
        res: [[$sgn:tt] $([$($val:tt)*])*],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[$sgn] [$($($val)*)*]],
            $($pst)*
        }
    };
}

// macro_rules! num_to_lit {
//     ([[$(pos)?] [$($num:tt)*]]) => {
//         ${count($num)}
//     };
//     ([[neg] [$($num:tt)*]]) => {
//         -${count($num)}
//     };
// }
// macro_rules! wrapper {
//     (
//         a: $a:tt,
//         b: $b:tt,
//     ) => {{
//         $crate::arith_div! {
//             @div
//             a: $a,
//             b: $b,
//             callback: [
//                 name: wrapper,
//                 pre: [],
//                 pst: [],
//             ],
//         }
//     }};
//     (
//         res: $res:tt,
//     ) => {
//         num_to_lit!($res)
//     };
// }
// const _: () = {
//     // 2 / 5 = 0
//     let tmp = wrapper!(
//         a: [[pos] [[] []]],
//         b: [[pos] [[] [] [] [] []]],
//     );
//     assert!(tmp == 0);
//     // (-2) / 5 = 0
//     let tmp = wrapper!(
//         a: [[neg] [[] []]],
//         b: [[pos] [[] [] [] [] []]],
//     );
//     assert!(tmp == 0);
//     // 2 / (-5) = 0
//     let tmp = wrapper!(
//         a: [[pos] [[] []]],
//         b: [[neg] [[] [] [] [] []]],
//     );
//     assert!(tmp == 0);
//     // (-2) / (-5) = 0
//     let tmp = wrapper!(
//         a: [[neg] [[] []]],
//         b: [[neg] [[] [] [] [] []]],
//     );
//     assert!(tmp == 0);
//     // 5 / 2 = 2
//     let tmp = wrapper!(
//         a: [[pos] [[] [] [] [] []]],
//         b: [[pos] [[] []]],
//     );
//     assert!(tmp == 2);
//     // 5 % (-2) = -2
//     let tmp = wrapper!(
//         a: [[pos] [[] [] [] [] []]],
//         b: [[neg] [[] []]],
//     );
//     assert!(tmp == -2);
//     // (-5) % 2 = -2
//     let tmp = wrapper!(
//         a: [[neg] [[] [] [] [] []]],
//         b: [[pos] [[] []]],
//     );
//     assert!(tmp == -2);
//     // (-5) % (-2) = 2
//     let tmp = wrapper!(
//         a: [[neg] [[] [] [] [] []]],
//         b: [[neg] [[] []]],
//     );
//     assert!(tmp == 2);
// };

/// Divides two signed magnitude base 1 numbers (`a / b` ordering).
///
/// If `a / 0` is attempted, it will defer to the [`befunge_pm::div_by_zero!`] proc macro, which
/// will cause the Befunge interface to prompt for a response. This response will then be used in
/// the expansion of this macro.
///
/// Examples:
/// ```
/// #![feature(macro_metavar_expr)]
///
/// macro_rules! num_to_lit {
///     ([[$(pos)?] [$($num:tt)*]]) => {
///         ${count($num)}
///     };
///     ([[neg] [$($num:tt)*]]) => {
///         -${count($num)}
///     };
/// }
///
/// macro_rules! wrapper {
///     (
///         a: $a:tt,
///         b: $b:tt,
///     ) => {{
///         befunge_dm::arith_div! {
///             @div
///             a: $a,
///             b: $b,
///             callback: [
///                 name: wrapper,
///                 pre: [],
///                 pst: [],
///             ],
///         }
///     }};
///     (
///         res: $res:tt,
///     ) => {
///         num_to_lit!($res)
///     };
/// }
///
/// const _: () = {
///     // 2 / 5 = 0
///     let tmp = wrapper!(
///         a: [[pos] [[] []]],
///         b: [[pos] [[] [] [] [] []]],
///     );
///     assert!(tmp == 0);
///     // (-2) / 5 = 0
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[pos] [[] [] [] [] []]],
///     );
///     assert!(tmp == 0);
///     // 2 / (-5) = 0
///     let tmp = wrapper!(
///         a: [[pos] [[] []]],
///         b: [[neg] [[] [] [] [] []]],
///     );
///     assert!(tmp == 0);
///     // (-2) / (-5) = 0
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[neg] [[] [] [] [] []]],
///     );
///     assert!(tmp == 0);
///     // 5 / 2 = 2
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] [] []]],
///         b: [[pos] [[] []]],
///     );
///     assert!(tmp == 2);
///     // 5 % (-2) = -2
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] [] []]],
///         b: [[neg] [[] []]],
///     );
///     assert!(tmp == -2);
///     // (-5) % 2 = -2
///     let tmp = wrapper!(
///         a: [[neg] [[] [] [] [] []]],
///         b: [[pos] [[] []]],
///     );
///     assert!(tmp == -2);
///     // (-5) % (-2) = 2
///     let tmp = wrapper!(
///         a: [[neg] [[] [] [] [] []]],
///         b: [[neg] [[] []]],
///     );
///     assert!(tmp == 2);
/// };
/// ```
///
/// Execution strategy:
///   1. Handle easy cases (`0 / n`, `n / 1`, `n / (-1)`, `a / b` where `|a| < |b|`).
///   2. Handle div by zero case with [`befunge_pm::div_by_zero`] proc macro.
///   3. Call [`crate::arith_div_mod`] macro using signs of numbers as internal rule labels for
///      callback.
///   4. Handle callback from [`crate::arith_div_mod`], make callback given to this macro call.
#[macro_export]
macro_rules! arith_div {
    // 0 / b = 0
    (
        @div
        a: [$asgn:tt []],
        b: $b:tt,
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[pos] []],
            $($pst)*
        }
    };
    // a / 0
    (
        @div
        a: $a:tt,
        b: [$bsgn:tt []],
        callback: $callback:tt,
    ) => {
        $crate::befunge_pm::div_by_zero! {
            socket: "befunge.input",
            callback: $callback,
        }
    };
    // a / 1
    (
        @div
        a: $a:tt,
        b: [[$(pos)?] [[]]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: $a,
            $($pst)*
        }
    };
    // a / (-1)
    (
        @div
        a: [[$(pos)?] $a:tt],
        b: [[neg] [[]]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[neg] $a],
            $($pst)*
        }
    };
    // (-a) / (-1)
    (
        @div
        a: [[neg] $a:tt],
        b: [[neg] [[]]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[pos] $a],
            $($pst)*
        }
    };
    // if |a| < |b|, then a / b = 0
    (
        @div
        a: [[$($asgn:tt)?] $a:tt],
        b: [[$($bsgn:tt)?] [$($b:tt)+]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        macro_rules! arith_div_lt_check {
            ([$($b)+ $$($$_:tt)*]) => {
                $crate::arith_div_mod! {
                    @divmod
                    a: $a,
                    b: [$($b)+],
                    callback: [
                        name: $crate::arith_div,
                        pre: [@$($asgn)? @$($bsgn)?],
                        pst: [
                            callback: [
                                name: $name,
                                pre: [$($pre)*],
                                pst: [$($pst)*],
                            ],
                        ],
                    ],
                }
            };
            ($$($$_:tt)*) => {
                $name! {
                    $($pre)*
                    res: [[pos] []],
                    $($pst)*
                }
            };
        }
        arith_div_lt_check! {
            $a
        }
    };
    // a / b
    (
        @$(pos)? @$(pos)?
        div: $div:tt,
        mod: $mod:tt,
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[pos] $div],
            $($pst)*
        }
    };
    // -a / b
    (
        @neg @$(pos)?
        div: $div:tt,
        mod: $mod:tt,
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[neg] $div],
            $($pst)*
        }
    };
    // a / (-b)
    (
        @$(pos)? @neg
        div: $div:tt,
        mod: $mod:tt,
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[neg] $div],
            $($pst)*
        }
    };
    // -a / -b
    (
        @neg @neg
        div: $div:tt,
        mod: $mod:tt,
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [[pos] $div],
            $($pst)*
        }
    }
}

/// Takes the modulus of two signed magnitude base 1 numbers.
///
/// If `a / 0` is attempted, it will defer to the [`befunge_pm::mod_by_zero!`] proc macro, which
/// will cause the Befunge interface to prompt for a response. This response will then be used in
/// the expansion of this macro.
///
/// Examples:
/// ```
/// #![feature(macro_metavar_expr)]
///
/// macro_rules! num_to_lit {
///     ([[$(pos)?] [$($num:tt)*]]) => {
///         ${count($num)}
///     };
///     ([[neg] [$($num:tt)*]]) => {
///         -${count($num)}
///     };
/// }
///
/// macro_rules! wrapper {
///     (
///         a: $a:tt,
///         b: $b:tt,
///     ) => {{
///         befunge_dm::arith_mod! {
///             @mod
///             a: $a,
///             b: $b,
///             callback: [
///                 name: wrapper,
///                 pre: [],
///                 pst: [],
///             ],
///         }
///     }};
///     (
///         res: $res:tt,
///     ) => {
///         num_to_lit!($res)
///     };
/// }
///
/// const _: () = {
///     // 2 % 5 = 2
///     let tmp = wrapper!(
///         a: [[pos] [[] []]],
///         b: [[pos] [[] [] [] [] []]],
///     );
///     assert!(tmp == 2);
///     // (-2) % 5 = -2
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[pos] [[] [] [] [] []]],
///     );
///     assert!(tmp == -2);
///     // 2 % (-5) = 2
///     let tmp = wrapper!(
///         a: [[pos] [[] []]],
///         b: [[neg] [[] [] [] [] []]],
///     );
///     assert!(tmp == 2);
///     // (-2) % (-5) = -2
///     let tmp = wrapper!(
///         a: [[neg] [[] []]],
///         b: [[neg] [[] [] [] [] []]],
///     );
///     assert!(tmp == -2);
///     // 5 % 2 = 1
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] [] []]],
///         b: [[pos] [[] []]],
///     );
///     assert!(tmp == 1);
///     // 5 % (-2) = 1
///     let tmp = wrapper!(
///         a: [[pos] [[] [] [] [] []]],
///         b: [[neg] [[] []]],
///     );
///     assert!(tmp == 1);
///     // (-5) % 2 = -1
///     let tmp = wrapper!(
///         a: [[neg] [[] [] [] [] []]],
///         b: [[pos] [[] []]],
///     );
///     assert!(tmp == -1);
///     // (-5) % (-2) = -1
///     let tmp = wrapper!(
///         a: [[neg] [[] [] [] [] []]],
///         b: [[neg] [[] []]],
///     );
///     assert!(tmp == -1);
/// };
/// ```
///
/// Execution strategy:
///   1. Check for `n % 0`. Handle this with [`befunge_pm::mod_by_zero!`].
///   2. Check if `a > b` in `a % b`. If yes, return `a`. Otherwise, call [`crate::arith_div_mod`].
///   3. Handle callback from [`crate::arith_div_mod`] and make callback given to this macro call.
#[macro_export]
macro_rules! arith_mod {
    // a % 0
    (
        @mod
        a: $a:tt,
        b: [$bsgn:tt []],
        callback: $callback:tt,
    ) => {
        $crate::befunge_pm::mod_by_zero! {
            socket: "befunge.input",
            callback: $callback,
        }
    };
    // if |a| < |b|, then a % b = a
    (
        @mod
        a: [$asgn:tt $a:tt],
        b: [$bsgn:tt [$($b:tt)*]],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        macro_rules! arith_mod_lt_check {
            ([$($b)* $$($$_:tt)*]) => {
                $crate::arith_div_mod! {
                    @divmod
                    a: $a,
                    b: [$($b)*],
                    callback: [
                        name: $crate::arith_mod,
                        pre: [
                            @catch
                            asgn: $asgn,
                        ],
                        pst: [
                            callback: [
                                name: $name,
                                pre: [$($pre)*],
                                pst: [$($pst)*],
                            ],
                        ],
                    ],
                }
            };
            (
                @acc
                rules: [$$($$rules:tt)*],
                num: [[] $$($$rest:tt)+],
            ) => {
                arith_mod_lt_check! {
                    @acc
                    rules: [
                        $$($$rules)*
                        ([$$$$($($b)*)+ $$($$rest)+]) => {
                            $name! {
                                $($pre)*
                                res: [$asgn [$$($$rest)+]],
                                $($pst)*
                            }
                        };
                    ],
                    num: [$$($$rest)+],
                }
            };
            (
                @acc
                rules: [$$($$rules:tt)*],
                num: [[]],
            ) => {
                macro_rules! arith_mod_exec {
                    $$($$rules)*
                    ([$$$$($($b)*)+]) => {
                        $name! {
                            $($pre)*
                            res: [$asgn []],
                            $($pst)*
                        }
                    };
                }
                arith_mod_exec! {
                    $a
                }
            };
            ([$$($$_:tt)*]) => {
                $name! {
                    $($pre)*
                    res: [$asgn $a],
                    $($pst)*
                }
            };
        }
        arith_mod_lt_check! {
            $a
        }
    };
    (
        @catch
        asgn: $asgn:tt,
        div: $div:tt,
        mod: $mod:tt,
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            res: [$asgn $mod],
            $($pst)*
        }
    };
}

/// Performs the division and modulus operations on two unsigned base 1 numbers simultaneously.
///
/// The caller must handle the case for division by 0, and must also handle returning the signs to
/// the integers with the callback result. Callback is performed as:
/// ```ignore
/// name! {
///     pre
///     div: [/* result */],
///     mod: [/* result */],
///     pst
/// }
/// ```
///
/// Execution strategy:
///   1. Define an ad-hoc macro named `arith_div_mod_exec` that repeatedly subtracts `b` from `a`
///      while keeping track of how many times it has done so. It will then make a callback with the
///      results.
///   2. Call this macro with `a` and `b` as inputs.
#[macro_export]
macro_rules! arith_div_mod {
    (
        @divmod
        a: $a:tt,
        b: [$($b:tt)*],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        macro_rules! arith_div_mod_exec {
            (
                @divmod
                left: [$($b)* $$($$rest:tt)*],
                div: [$$($$div:tt)*],
            ) => {
                arith_div_mod_exec! {
                    @divmod
                    left: [$$($$rest)*],
                    div: [$$($$div)* []],
                }
            };
            (
                @divmod
                left: [$$($$rest:tt)*],
                div: $$div:tt,
            ) => {
                $name! {
                    $($pre)*
                    div: $$div,
                    mod: [$$($${ignore($$rest)} [])*],
                    $($pst)*
                }
            };
        }
        arith_div_mod_exec! {
            @divmod
            left: $a,
            div: [],
        }
    };
    (
        @divmod
        a: $a:tt,
        b: [],
        callback: [
            name: $name:path,
            pre: $pre:tt,
            pst: $pst:tt,
        ],
    ) => {
        compile_error! {
            concat!(
                "This macro was called with `b = 0`!\n",
                "Callback {\n",
                "    name: ",
                stringify!($name),
                ",\n",
                "    pre: ",
                stringify!($pre),
                ",\n",
                "    pst: ",
                stringify!($pst),
                ",\n",
                "}",
            )
        }
    };
}
