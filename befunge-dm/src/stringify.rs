#[macro_export]
/// Prints out the program memory from the Befunge interpreter.
macro_rules! befunge_stringify {
    (
        @stringify
        lines: $lines:tt,
    ) => {
        $crate::befunge_stringify! {
            @stringify @inner
            lines: $lines,
            obuf: [],
        }
    };
    (
        @stringify @inner
        lines: [[]],
        obuf: [$($out:tt)*],
    ) => {
        const _: &str = concat!($($out),*);
    };
    (
        @stringify @inner
        lines: [[] $($lrest:tt)+],
        obuf: [$($out:tt)*],
    ) => {
        const _: &str = concat!($($out),*);
        $crate::befunge_stringify! {
            @stringify @inner
            lines: [$($lrest)+],
            obuf: [],
        }
    };
    (
       @stringify @inner
       lines: [[$lhh:tt $($lht:tt)*] $($lt:tt)*],
       obuf: $obuf:tt,
    ) => {
        $crate::befunge_pm::stringify_with_callback! {
            tokens: [$lhh],
            callback: [
                name: $crate::befunge_stringify,
                pre: [
                    @stringify @inner @makeliteral
                    lines: [[$($lht)*] $($lt)*],
                    obuf: $obuf,
                ]
            ],
        }
    };
    (
        @stringify @raw
        lines: $lines:tt,
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [],
        }
    };
    (
        @stringify @raw @inner
        lines: [[]],
        obuf: [$($out:tt)*],
    ) => {
        const _: &str = concat!($($out),*);
    };
    (
        @stringify @raw @inner
        lines: [[] $($lrest:tt)+],
        obuf: [$($out:tt)*],
    ) => {
        const _: &str = concat!($($out),*);
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: [$($lrest)+],
            obuf: [],
        }
    };
    (
        @stringify @raw @inner
        lines: [[$lhh:tt $($lht:tt)*] $($lt:tt)*],
        obuf: $obuf:tt,
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner @char
            lines: [[$($lht)*] $($lt)*],
            obuf: $obuf,
            char: $lhh,
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '\n',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "\n"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: ' ',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* " "],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '+',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "+"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '-',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "-"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '*',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "*"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '/',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "/"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '%',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "%"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '!',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "!"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '`',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "`"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '>',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* ">"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '<',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "<"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '^',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "^"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: 'v',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "v"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '?',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "?"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '|',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "_"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '|',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "|"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '"',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "\""],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: ':',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* ":"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '\\',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "\\"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '$',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "$"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '.',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "."],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: ',',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* ","],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '#',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "#"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: 'g',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "g"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: 'p',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "p"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '&',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "&"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '~',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "~"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '@',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "@"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '0',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "0"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '1',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "1"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '2',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "2"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '3',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "3"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '4',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "4"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '5',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "5"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '6',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "6"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '7',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "7"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '8',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "8"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        char: '9',
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* "9"],
        }
    };
    (
        @stringify @raw @inner @char
        lines: $lines:tt,
        obuf: $obuf:tt,
        char: $other:tt,
    ) => {
        $crate::befunge_pm::stringify_with_callback! {
            tokens: [$other],
            callback: [
                name: $crate::befunge_stringify,
                pre: [
                    @stringify @raw @inner @catch
                    lines: $lines,
                    obuf: $obuf,
                ],
                pst: [],
            ],
        }
    };
    (
        @stringify @raw @inner @catch
        lines: $lines:tt,
        obuf: [$($obuf:tt)*],
        stringified: $char:tt,
    ) => {
        $crate::befunge_stringify! {
            @stringify @raw @inner
            lines: $lines,
            obuf: [$($obuf)* $char],
        }
    }
}
