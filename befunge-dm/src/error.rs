/// The home for the more complex [`compile_error!`] invocations in this project. The internal rules
/// here are for the following errors:
///
/// - `@initerr @rows`: Initialisation failed due to too many rows being read
/// - `@initerr @cols`: Initialisation failed due to too many columns being read
/// - `@unknowninstr`: Unknown instruction encountered
///
/// Anything else is a helper rule for one of the above.
///
/// You probably shouldn't be calling this.
#[macro_export]
macro_rules! befunge_error {
    (
        @initerr @rows
        program: [$($line:tt)*],
        left: [$($left:tt)+],
    ) => {
        compile_error! {
            concat! {
                "Too many rows in program! Read in so far:\n",
                $crate::befunge_stringify! {
                    @stringify
                    lines: [$($line)*],
                },
                "\nLeft to read:\n",
                $crate::befunge_stringify! {
                    @stringify @raw
                    lines: [[$($left)+]],
                }
            }
        }
    };
    (
        @initerr @cols
        program: [$($line:tt)*],
        left: [$($left:tt)+],
    ) => {
        compile_error! {
            concat! {
                "Too many columns in program! Read in so far:\n",
                $crate::befunge_stringify! {
                    @stringify
                    lines: [$($line)*],
                }
                "\nLeft to read:\n",
                $crate::befunge_stringify! {
                    @stringify @raw
                    lines: [[$($left)+]],
                }
            }
        }
    };
    (
        @unknowninstr
        instr: $instr:tt,
        row: $row:tt,
        col: $col:tt,
        stack: $stack:tt,
        dir: $dir:tt,
    ) => {
        $crate::befunge_error! {
            @unknowninstr @loop
            instr: $instr,
            row: $row,
            col: $col,
            dir: $dir,
            stack: $stack,
            tokens: [],
        }
    };
    (
        @unknowninstr @loop
        instr: $instr:tt,
        row: $row:tt,
        col: $col:tt,
        dir: $dir:tt,
        stack: [],
        tokens: [$([$hfst:tt$(, $hsnd:tt)?] $([$tfst:tt$(, $tsnd:tt)?])*)?],
    ) => {
        compile_error! {
            concat! {
                "Encountered unknown instruction `",
                stringify!($instr),
                "` at location (",
                stringify!($row),
                ", ",
                stringify!($col),
                ") while stringmode was disabled.\nCurrent stack:\n",
                $(
                    "top: ",
                    stringify!($hfst),
                    $(
                        " (",
                        stringify!($hsnd),
                        ")",
                    )?
                    "\n",
                    $(
                        "     ",
                        stringify!($tfst),
                        $(
                            " (",
                            stringify!($tsnd),
                            ")",
                        )?
                        "\n",
                    )*
                )?
                "Current direction: ",
                stringify!($dir),
            }
        }
    };
    (
        @unknowninstr @loop
        instr: $instr:tt,
        row: $row:tt,
        col: $col:tt,
        dir: $dir:tt,
        stack: [$stackh:tt $($stackt:tt)*],
        tokens: $tokens:tt,
    ) => {
        $crate::code_to_char_pretty! {
            @match
            num: $stackh,
            callback: [
                name: $crate::befunge_error,
                pre: [
                    @unknowninstr @loopcatch
                    instr: $instr,
                    row: $row,
                    col: $col,
                    dir: $dir,
                    stack: [$($stackt)*],
                    tokens: $tokens,
                ],
                pst: [],
            ],
        }
    };
    (
        @unknowninstr @loopcatch
        instr: $instr:tt,
        row: $row:tt,
        col: $col:tt,
        dir: $dir:tt,
        stack: $stack:tt,
        tokens: [$($token:tt)*],
        char: $char:tt,
    ) => {
        $crate::befunge_error! {
            @unknowninstr @loop
            instr: $instr,
            row: $row,
            col: $col,
            dir: $dir,
            stack: $stack,
            tokens: [$($token)* $char],
        }
    };
}
