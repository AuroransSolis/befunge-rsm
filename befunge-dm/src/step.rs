#[macro_export]
/// Interpreter step macro. You probably shouldn't be calling this directly.
///
/// Behavioural notes:
/// - The only easy direction to move is right. Everything else sucks in its own special way. In
///   descending order of difficulty:
///   1. Right
///   2. Left, down
///   3. Up
/// - Whenever an operation requires values from the stack, if that value doesn't exist, a 0 is
///   provided to the operation. For instance if your stack is empty and you execute `:`, then
///   you will now have two `0`s on the stack.
/// - The stack may contain only numeric values (see next point for what I mean by this), but the
///   program memory may contain character literals or numeric values.
/// - This interpreter uses signed magnitude base 1 numbers. I have chosen to represent this with
///   the notation `[[sign] [value]]`, where 0 is always positive, though an empty sign token tree
///   is treated as positive. To clarify: `[[pos] [[] [] []]]` is the number `3`, which is
///   equivalent to `[[] [[] [] []]]]`. It then follows that `[[neg] [[] [] []]]` is `-3`.
/// - Execution moves through this program in roughly three steps:
///   1. Special states (such as stringmode or bridging) are handled (though this does occur at the
///      same time as step 2)
///   2. Instructions are executed
///   3. Movement occurs
/*
    Comments in this macro are formatted as:

    <ASCII ART HEADER>
    plaintext header
    [Optional description of operation]

    This is because it's a pretty large macro, and I wanted to be able to see in the sidebar (in
    VSCode) whereabouts in the macro I am more easily. The ASCII art text is readable in the preview
    there. However, for those who may be using other assistive tools for reading what's on the
    screen, there's a plaintext copy of the same text just below it.

    ASCII art was generated using https://patorjk.com/software/taag/ with the "Banner" font, and
    some additional modifications made by me.
*/
macro_rules! befunge_step {
    (
        @init
        program: [
            [$hh:tt $($ht:tt)+]
            $($t:tt)+
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("init");
        $crate::befunge_step! {
            @instr
            stack: [],
            dir: [right],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: [],
                cur: [
                    pre: [],
                    cur: [$hh],
                    pst: [$($ht)+],
                ],
                pst: [$($t)+],
            ],
            debug: $debug,
        }
    };
    /*
         #####  ####### ######  ### #     #  #####  #     # ####### ######  #######  #     ####### #     #
        #     #    #    #     #  #  ##    # #     # ##   ## #     # #     # #       ###    #     # ##    #
        #          #    #     #  #  # #   # #       # # # # #     # #     # #        #     #     # # #   #
         #####     #    ######   #  #  #  # #  #### #  #  # #     # #     # #####          #     # #  #  #
              #    #    #   #    #  #   # # #     # #     # #     # #     # #        #     #     # #   # #
        #     #    #    #    #   #  #    ## #     # #     # #     # #     # #       ###    #     # #    ##
         #####     #    #     # ### #     #  #####  #     # ####### ######  #######  #     ####### #     #

        STRINGMODE: ON
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [true],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['"'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("stringmode: off");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['"'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Numeric values
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [true],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: [[[$($sgn:tt)?] [$($val:tt)*]]],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("stringmode: numeric");
        $crate::befunge_step! {
            @move
            stack: [[] $($stack)*],
            dir: $dir,
            stringmode: $stringmode,
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: [[[$($sgn)?] [$($val)*]]],
                    pst: $cpst:tt,
                ],
                pst: $pst:tt,
            ],
            debug: $debug,
        }
    };
    // Character literals must be converted to numbers before pushing to stack.
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [true],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: [$char:tt],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("stringmode: char");
        $crate::char_to_code! {
            @match
            char: $char,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @char_to_code
                    stack: $stack,
                    dir: $dir,
                    stringmode: [true],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: [$char],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    // all numbers
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [true],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: [[[$cursgn:tt] [$($curnum:tt)*]]],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("stringmode: other num???");
        $crate::befunge_step! {
            @move
            stack: [[[$cursgn] [$($curnum)*]] $($stack)*],
            dir: $dir,
            stringmode: [true],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: [[[$cursgn] [$($curnum)*]]],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
        ### #     #  #####  ####### ######   #####
         #  ##    # #     #    #    #     # #     #
         #  # #   # #          #    #     # #
         #  #  #  #  #####     #    ######   #####
         #  #   # #       #    #    #   #         #
         #  #    ## #     #    #    #    #  #     #
        ### #     #  #####     #    #     #  #####

        INSTRS
    */
    // catch bridges
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [true],
        progstate: $progstate:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("bridge: jumping over instruction");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    /*
                  #
                 ###    ###### #    # #####
                  #     #      ##  ## #    #
                        #####  # ## # #    #
                  #     #      #    # #####
                 ###    #      #    # #
                  #     ###### #    # #

          : EMP
        Spaces are no-ops.
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: [' '],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("empty cell");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: [' '],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
                    #        #    ######  ######
           #       ###      # #   #     # #     #
           #        #      #   #  #     # #     #
         #####            #     # #     # #     #
           #        #     ####### #     # #     #
           #       ###    #     # #     # #     #
                    #     #     # ######  ######

        + : ADD
        push(stack[0] + stack[1])
    */
    (
        @instr
        stack: [
            $(
                [[$($stack0sgn:tt)?] [$($stack0val:tt)*]]
                $(
                    [[$($stack1sgn:tt)?] [$($stack1val:tt)*]]
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['+'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "add",
            $($($stack0sgn)? ${count($stack0val)}, )?
            $($($($stack1sgn)? ${count($stack1val)})?)?
        );
        $crate::arith_add! {
            @add
            a: [[$($($stack0sgn)?)?] [$($($stack0val)*)?]],
            b: [[$($($($stack1sgn)?)?)?] [$($($($stack1val)*)?)?]],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @arith
                    stack: [$($($($stackrest)*)?)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['+'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
                    #      #####  #     # ######
                   ###    #     # #     # #     #
                    #     #       #     # #     #
         #####             #####  #     # ######
                    #           # #     # #     #
                   ###    #     # #     # #     #
                    #      #####   #####  ######

        - : SUB
        push(stack[1] - stack[0])
    */
    (
        @instr
        stack: [
            $(
                [[$($stack0sgn:tt)?] [$($stack0val:tt)*]]
                $(
                    [[$($stack1sgn:tt)?] [$($stack1val:tt)*]]
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['-'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "sub",
            $($($stack0sgn)? ${count($stack0val)}, )?
            $($($($stack1sgn)? ${count($stack1val)})?)?
        );
        $crate::arith_sub! {
            @sub
            a: [[$($($($stack1sgn)?)?)?] [$($($($stack1val)*)?)?]],
            b: [[$($($stack0sgn)?)?] [$($($stack0val)*)?]],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @arith
                    stack: [$($($($stackrest)*)?)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['-'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
                    #     #     # #     # #
         #   #     ###    ##   ## #     # #
          # #       #     # # # # #     # #
        #######           #  #  # #     # #
          # #       #     #     # #     # #
         #   #     ###    #     # #     # #
                    #     #     #  #####  #######

        * : MUL
        push(stack[0] * stack[1])
    */
    (
        @instr
        stack: [
            $(
                [[$($stack0sgn:tt)?] [$($stack0val:tt)*]]
                $(
                    [[$($stack1sgn:tt)?] [$($stack1val:tt)*]]
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['*'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "mul",
            $($($stack0sgn)? ${count($stack0val)}, )?
            $($($($stack1sgn)? ${count($stack1val)})?)?
        );
        $crate::arith_mul! {
            @mul
            a: [[$($($stack0sgn)?)?] [$($($stack0val)*)?]],
            b: [[$($($($stack1sgn)?)?)?] [$($($($stack1val)*)?)?]],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @arith
                    stack: [$($($($stackrest)*)?)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['*'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
              #     #     ######  ### #     #
             #     ###    #     #  #  #     #
            #       #     #     #  #  #     #
           #              #     #  #  #     #
          #         #     #     #  #   #   #
         #         ###    #     #  #    # #
        #           #     ######  ###    #

        / : DIV
        push(stack[1] / stack[0])
    */
    (
        @instr
        stack: [
            $(
                [[$stack0sgn:tt] [$($stack0val:tt)*]]
                $(
                    [[$stack1sgn:tt] [$($stack1val:tt)*]]
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['/'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "div",
            $($($stack0sgn)? ${count($stack0val)}, )?
            $($($($stack1sgn)? ${count($stack1val)})?)?
        );
        $crate::arith_div! {
            @div
            a: [[$($($stack1sgn)?)?] [$($($($stack1val)*)?)?]],
            b: [[$($stack0sgn)?] [$($($stack0val)*)?]],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @arith
                    stack: [$($($($stackrest)*)?)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['/'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
        ###   #     #     #     # ####### ######
        # #  #     ###    ##   ## #     # #     #
        ### #       #     # # # # #     # #     #
           #              #  #  # #     # #     #
          # ###     #     #     # #     # #     #
         #  # #    ###    #     # #     # #     #
        #   ###     #     #     # ####### ######

        % : MOD
        push(stack[1] % stack[0])
    */
    (
        @instr
        stack: [
            $(
                [[$stack0sgn:tt] [$($stack0val:tt)*]]
                $(
                    [[$stack1sgn:tt] [$($stack1val:tt)*]]
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['%'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "div",
            $($($stack0sgn)? ${count($stack0val)}, )?
            $($($($stack1sgn)? ${count($stack1val)})?)?
        );
        $crate::arith_mod! {
            @mod
            a: [[$($($stack1sgn)?)?] [$($($($stack1val)*)?)?]],
            b: [[$($stack0sgn)?] [$($($stack0val)*)?]],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @arith
                    stack: [$($($($stackrest)*)?)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['%'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
          ###       #     #     # ####### #######
          ###      ###    ##    # #     #    #
          ###       #     # #   # #     #    #
           #              #  #  # #     #    #
                    #     #   # # #     #    #
          ###      ###    #    ## #     #    #
          ###       #     #     # #######    #

        ! : NOT
        if stack[0] == 0 {
            push(1)
        } else {
            push(0)
        }
    */
    (
        @instr
        stack: [
            $(
                [[$($stack0sgn:tt)?] []]
                $($stackrest:tt)*
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['!'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("not0 (stack head is zero)");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[]]] $($($stackrest)*)?],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['!'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        @instr
        stack: [
            [[$stack0sgn:tt] [$($stack0val:tt)+]]
            $($stackrest:tt)*
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['!'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("not1 (stack head is nonzero)");
        $crate::befunge_step! {
            @move
            stack: [[[pos] []] $($stackrest)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['!'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
          ###       #      #####  ######  #######
          ###      ###    #     # #     #    #
           #        #     #       #     #    #
            #             #  #### ######     #
                    #     #     # #   #      #
                   ###    #     # #    #     #
                    #      #####  #     #    #

        ` : GRT
        if stack[1] > stack[0] {
            push(1)
        } else {
            push(0)
        }
    */
    (
        // Cover two cases:
        //   - stack = []
        //   - head(stack) = 0
        @instr
        stack: [$([[$($sgn:tt)?] []])?],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['`'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("grt0 (empty stack or head is zero)");
        $crate::befunge_step! {
            @move
            stack: [[[pos] []]],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['`'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        // Cover one case:
        //   - stack = [n] where n > 0
        @instr
        stack: [[[$(pos)?] [$($topval:tt)+]]],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['`'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("grt1", ${count($topval)});
        $crate::befunge_step! {
            @move
            stack: [[[pos] []]],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['`'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        // Cover one case:
        //   - stack = [n] where n is negative
        // This would have stack[0] = n and stack[1] = 0, so unconditionally push 1 to the stack.
        @instr
        stack: [[[neg] [$($topval:tt)*]]],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['`'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("grt2", -${count($topval)});
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[]]]],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['`'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        // Cover one case:
        //   - stack = [a, b, ...] a is positive and b is negative
        // This checks if `b > a`, so unconditionally push 0 to the stack.
        @instr
        stack: [
            [[$(pos)?] [$($topval:tt)*]]
            [[neg] [$($botval:tt)*]]
            $($stackrest:tt)*
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['`'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("grt3", -${count($botval)}, ${count($topval)});
        $crate::befunge_step! {
            @move
            stack: [[[pos] []] $($stackrest)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['`'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        // Cover one case:
        //   - stack = [a, b, ...] a is negative and b is positive
        // This checks if `b > a`, so unconditionally push 1 to the stack.
        @instr
        stack: [
            [[neg] [$($topval:tt)*]]
            [[$(pos)?] [$($botval:tt)*]]
            $($stackrest:tt)*
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['`'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("grt4", ${count($botval)}, -${count($topval)});
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[]]] $($stackrest)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['`'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        // Cover all cases where the top two values are positive
        @instr
        stack: [
            [[$(pos)?] [$($topval:tt)*]]
            [[$(pos)?] [$($botval:tt)*]]
            $($stackrest:tt)*
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['`'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("grt5", ${count($botval)}, ${count($topval)});
        macro_rules! befunge_step_grt_exec {
            ($($topval)* $$($$_:tt)+) => {
                $crate::socket_debug_default!("    => true");
                $crate::befunge_step! {
                    @move
                    stack: [[[pos] [[]]] $($stackrest)*],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['`'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                    debug: $debug,
                }
            };
            ($$($$_:tt)*) => {
                $crate::socket_debug_default!("    => false");
                $crate::befunge_step! {
                    @move
                    stack: [[[pos] []] $($stackrest)*],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['`'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                    debug: $debug,
                }
            }
        }
        befunge_step_grt_exec! {
            $($botval)*
        }
    };
    (
        // Cover all cases where the top two values are negative
        @instr
        stack: [
            [[$topsgn:tt] [$($topval:tt)*]]
            [[$botsgn:tt] [$($botval:tt)*]]
            $($stackrest:tt)*
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['`'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("grt6", -${count($botval)}, -${count($topval)});
        macro_rules! befunge_step_lt_exec {
            ($($topval)* $$($$_:tt)+) => {
                $crate::befunge_step! {
                    @move
                    stack: [[[pos] []] $($stackrest)*],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['`'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                    debug: $debug,
                }
            };
            ($$($$_:tt)*) => {
                $crate::befunge_step! {
                    @move
                    stack: [[[pos] [[]]] $($stackrest)*],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['`'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                    debug: $debug,
                }
            }
        }
        befunge_step_lt_exec! {
            $($botval)*
        }
    };
    /*
          #         #     ######   #####  ######
           #       ###    #     # #     # #     #
            #       #     #     # #       #     #
             #            ######  #       ######
            #       #     #       #       #   #
           #       ###    #       #     # #    #
          #         #     #        #####  #     #

        > : PCR
        pc = right
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['>'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("pcr");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: [right],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['>'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
             #      #     ######   #####  #
            #      ###    #     # #     # #
           #        #     #     # #       #
          #               ######  #       #
           #        #     #       #       #
            #      ###    #       #     # #
             #      #     #        #####  #######

        < : PCL
        pc = left
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['<'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("pcl");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: [left],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['<'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
           #        #     ######   #####  #     #
          # #      ###    #     # #     # #     #
         #   #      #     #     # #       #     #
                          ######  #       #     #
                    #     #       #       #     #
                   ###    #       #     # #     #
                    #     #        #####   #####

        ^ : PCU
        pc = up
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['^'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("pcu");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: [up],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['^'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
                    #     ######   #####  ######
         #    #    ###    #     # #     # #     #
         #    #     #     #     # #       #     #
         #    #           ######  #       #     #
         #    #     #     #       #       #     #
          #  #     ###    #       #     # #     #
           ##       #     #        #####  ######

        v : PCD
        pc = up
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['v'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("pcd");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: [down],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['v'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #####      #     ######   #####
        #     #    ###    #     # #     #
              #     #     #     # #
           ###            ######  #
           #        #     #       #
                   ###    #       #     #
           #        #     #        #####
                                          #######

        ? : PC_
        pc = random(up, down, left, right)
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['?'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("pc_");
        $crate::befunge_pm::choose_random! {
            choices: [[left] [right] [up] [down]],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @pc_
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['?'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
                    #     ### ####### #     #
                   ###     #  #       #     #
                    #      #  #       #     #
                           #  #####   #######
                    #      #  #       #     #
                   ###     #  #       #     #
                    #     ### #       #     #
        #######

        _ : IFH
        if stack[0] == 0 {
            pc = right
        } else {
            pc = left
        }
    */
    (
        @instr
        stack: [$(
            [$zerosgn:tt []]
            $($stackrest:tt)*
        )?],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['_'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("ifh0 (right)");
        $crate::befunge_step! {
            @move
            stack: [$($($stackrest)*)?],
            dir: [right],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['_'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        @instr
        stack: [$nonzero:tt $($stacktail:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['_'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("ifh1 (left)");
        $crate::befunge_step! {
            @move
            stack: [$($stacktail)*],
            dir: [left],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['_'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
           #        #     ### ####### #     #
           #       ###     #  #       #     #
           #        #      #  #       #     #
           #               #  #####   #     #
           #        #      #  #        #   #
           #       ###     #  #         # #
           #        #     ### #          #

        | : IFV
        if stack[0] == 0 {
            pc = down
        } else {
            pc = up
        }
    */
    (
        @instr
        stack: [$(
            [$zerosgn:tt []]
            $($stackrest:tt)*
        )?],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['|'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("ifv0 (down)");
        $crate::befunge_step! {
            @move
            stack: [$($($stackrest)*)?],
            dir: [down],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['|'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        @instr
        stack: [
            $nonzero:tt
            $($stacktail:tt)*
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['|'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("ifv1 (up)");
        $crate::befunge_step! {
            @move
            stack: [$($stacktail)*],
            dir: [up],
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['|'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
        ### ###     #      #####  #######  #####
        ### ###    ###    #     #    #    #     #
         #   #      #     #          #    #
                           #####     #    #  ####
                    #           #    #    #     #
                   ###    #     #    #    #     #
                    #      #####     #     #####

        " : STG
        enable stringmode
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['"'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("stringmode enabled");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: $dir,
            stringmode: [true],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['"'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #      #     ######  #     # ######
        ###    ###    #     # #     # #     #
         #      #     #     # #     # #     #
                      #     # #     # ######
         #      #     #     # #     # #
        ###    ###    #     # #     # #
         #      #     ######   #####  #

        : : DUP
        duplicate head of stack
    */
    (
        @instr
        stack: [
            $(
                [[$($stack0sgn:tt)?] [$($stack0val:tt)*]]
                $($stackrest:tt)*
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: [':'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "dup",
            $($($stack0sgn)? ${count($stack0val)})?
        );
        $crate::befunge_step! {
            @move
            stack: [
                [[$($($stack0sgn)?)?] [$($($stack0val)*)?]]
                [[$($($stack0sgn)?)?] [$($($stack0val)*)?]]
                $($($stackrest)*)?
            ],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: [':'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
        #           #      #####  #     # ######
         #         ###    #     # #  #  # #     #
          #         #     #       #  #  # #     #
           #               #####  #  #  # ######
            #       #           # #  #  # #
             #     ###    #     # #  #  # #
              #     #      #####   ## ##  #

        \ : SWP
        swap the values at the top of the stack
    */
    (
        @instr
        stack: [
            $(
                [[$($stack0sgn:tt)?] [$($stack0val:tt)*]]
                $(
                    [[$($stack1sgn:tt)?] [$($stack1val:tt)*]]
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['\\'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "swp",
            $($($stack0sgn)? ${count($stack0val)}, )?
            $($($($stack1sgn)? ${count($stack1val)})?)?
        );
        $crate::befunge_step! {
            @move
            stack: [
                [[$($($stack1sgn)?)?] [$($($($stack1val)*)?)?]]
                [[$($stack0sgn)?] [$($($stack0val)*)?]]
                $($($($stackrest)*)?)?
            ],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['\\'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #####      #     ######  ####### ######
        #  #  #    ###    #     # #     # #     #
        #  #        #     #     # #     # #     #
         #####            ######  #     # ######
           #  #     #     #       #     # #
        #  #  #    ###    #       #     # #
         #####      #     #       ####### #

        $ : POP
        discard the value at the top of the stack
    */
    (
        @instr
        stack: [$($stackhead:tt $($stackrest:tt)*)?],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['$'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!(
            "pop",
            $($($stack0sgn)? ${count($stack0val)})?
        );
        $crate::befunge_step! {
            @move
            stack: [$($($stackrest)*)?],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['$'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
                    #     ### #     # #######
                   ###     #  ##    #    #
                    #      #  # #   #    #
                           #  #  #  #    #
          ###       #      #  #   # #    #
          ###      ###     #  #    ##    #
          ###       #     ### #     #    #

        . : INT
        output head of stack as an integer
    */
    (
        @instr
        stack: [$([[$(pos)?] [$($stack0val:tt)*]] $($stackrest:tt)*)?],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['.'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("int (pos)", $(${count($stack0val)})?);
        $crate::befunge_pm::print_integer! {
            number: ${count($stack0val)},
            socket: "befunge.output",
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move
                    stack: [$($($stackrest)*)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['.'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                    debug: $debug,
                ],
                pst: [],
            ],
        }
    };
    (
        @instr
        stack: [$([[neg] [$($stack0val:tt)*]] $($stackrest:tt)*)?],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['.'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("int (neg)", $(${count($stack0val)})?);
        $crate::befunge_pm::print_integer! {
            number: -${count($stack0val)},
            socket: "befunge.output",
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move
                    stack: [$($($stackrest)*)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['.'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                    debug: $debug,
                ],
                pst: [],
            ],
        }
    };
    /*
                    #      #####  #     # ######
                   ###    #     # #     # #     #
                    #     #       #     # #     #
                          #       ####### ######
          ###       #     #       #     # #   #
          ###      ###    #     # #     # #    #
           #        #      #####  #     # #     #
          #

        , : CHR
        output head of stack as a character
    */
    (
        @instr
        stack: [
            $(
                [[$($stack0sgn:tt)?] [$($stack0val:tt)*]]
                $($stackrest:tt)*
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: [','],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("chr" $(, $($stack0sgn)? ${count($stack0val)})?);
        $crate::code_to_char! {
            @match
            num: [[$($($stack0sgn)?)?] [$($($stack0val)*)?]],
            callback: [
                name: $crate::befunge_pm::print_ascii,
                pre: [],
                pst: [
                    socket: "befunge.output",
                    callback: [
                        name: $crate::befunge_step,
                        pre: [
                            @move
                            stack: [$($($stackrest)*)?],
                            dir: $dir,
                            stringmode: [false],
                            bridge: [false],
                            progstate: [
                                pre: $pre,
                                cur: [
                                    pre: $cpre,
                                    cur: [','],
                                    pst: $cpst,
                                ],
                                pst: $pst,
                            ],
                            debug: $debug,
                        ],
                        pst: [],
                    ],
                ],
            ],
        }
    };
    /*
          # #       #     ######  ######   #####
          # #      ###    #     # #     # #     #
        #######     #     #     # #     # #
          # #             ######  #     # #  ####
        #######     #     #     # #     # #     #
          # #      ###    #     # #     # #     #
          # #       #     ######  ######   #####

        # : BDG
        set bridge to true
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['#'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("bridge: set to true");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: $dir,
            stringmode: [false],
            bridge: [true],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['#'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
                    #      #####  ####### #######
                   ###    #     # #          #
                    #     #       #          #
          #### #          #  #### #####      #
         #    #     #     #     # #          #
         #    #    ###    #     # #          #
          #####     #      #####  #######    #
              #
         #    #
          ####

        g : GET
        push(progmem(x = stack[1], y = stack[0]))
    */
    (
        @instr
        stack: [
            [[neg] [$($stack0val:tt)*]]
            $(
                $stack1:tt
                $($stackrest:tt)*
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['g'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("get0");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[getdbg]],
            expand: [
                const _: &str = concat!("Y index was out of bounds! Pushed 0 to stack.");
            ],
        }
        $crate::befunge_step! {
            @move
            stack: [[[pos] []] $($($stackrest)*)?],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['g'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        @instr
        stack: [
            [[$(pos)?] [$($stack0val:tt)*]]
            [[neg] [$($stack1val:tt)*]]
            $($stackrest:tt)*
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['g'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("get1");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[getdbg]],
            expand: [
                const _: &str = concat!("X index was out of bounds! Pushed 0 to stack.");
            ],
        }
        $crate::befunge_step! {
            @move
            stack: [[[pos] []] $($($stackrest)*)?],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['g'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        @instr
        stack: [
            $(
                [[$(pos)?] [$($y:tt)*]]
                $(
                    [[$(pos)?] [$($x:tt)*]]
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: ['g'],
                pst: [$($cpst:tt)*],
            ],
            pst: [$($pst:tt)*],
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("get2", ${count($x)}, ${count($y)});
        macro_rules! sanitise_coords_for_dbg {
            (
                x: [$$($$xdbg:tt)*],
                y: [$$($$ydbg:tt)*],
            ) => [
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[getdbg]],
                    expand: [
                        const _: &str = concat!(
                            "Retrieving value at: (",
                            $${count($$ydbg)},
                            ", ",
                            $${count($$xdbg)},
                            ")",
                        );
                    ],
                }
            ]
        }
        sanitise_coords_for_dbg! {
            x: [$($($($x)*)?)?],
            y: [$($($y)*)?],
        }
        macro_rules! befunge_step_get_coord_check {
            (
                xcheck: [$($($($x)*)?)? $$([])*],
                ycheck: [$($($y)*)? $$([])*],
            ) => {
                $crate::list_split_at_length_of! {
                    @init
                    lenof: [$($($y)*)?],
                    split: [$($pre)* [$($cpre)* 'g' $($cpst)*] $($pst)*],
                    callback: [
                        name: $crate::befunge_step,
                        pre: [
                            @catch @get @splitrow
                            stack: [$($($($stackrest)*)?)?],
                            dir: $dir,
                            stringmode: [false],
                            bridge: [false],
                            progstate: [
                                pre: [$($pre)*],
                                cur: [
                                    pre: [$($cpre)*],
                                    cur: ['g'],
                                    pst: [$($cpst)*],
                                ],
                                pst: [$($pst)*],
                            ],
                            x: [$($($($x)*)?)?],
                        ],
                        pst: [
                            debug: $debug,
                        ],
                    ],
                }
            };
            ($$($$_:tt)*) => {
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[getdbg]],
                    expand: [
                        const _: &str = concat!("Index was out of bounds! Pushed 0 to stack.");
                    ],
                }
                $crate::befunge_step! {
                    @move
                    stack: [[[pos] []] $($($($stackrest)*)?)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: $progstate,
                    debug: $debug,
                }
            };
        }
        befunge_step_get_coord_check! {
            xcheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
            ycheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
        }
    };
    /*
                    #     ######  #     # #######
                   ###    #     # #     #    #
                    #     #     # #     #    #
        # ####            ######  #     #    #
         #    #     #     #       #     #    #
         #    #    ###    #       #     #    #
         #####      #     #        #####     #
         #
         #
         #

        p : PUT
        set_progmem(val = stack[2], x = stack[1], y = stack[0])
    */
    (
        @instr
        stack: [
            [[neg] [$($stack0val:tt)*]]
            $(
                $stack1:tt
                $(
                    $stack2:tt
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['p'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("put0");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[getdbg]],
            expand: [
                const _: &str = concat!("Y index was out of bounds! Abandoning put attempt.");
            ],
        }
        $crate::befunge_step! {
            @move
            stack: [$($($($stackrest)*)?)?],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['p'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        @instr
        stack: [
            $stack0:tt
            $(
                [[neg] [$($stack1val:tt)*]]
                $(
                    $stack2:tt
                    $($stackrest:tt)*
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['p'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("put1");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[getdbg]],
            expand: [
                const _: &str = concat!("X index was out of bounds! Abandoning put attempt.");
            ],
        }
        $crate::befunge_step! {
            @move
            stack: [$($($($stackrest)*)?)?],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['p'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    (
        @instr
        stack: [
            $(
                $stack0:tt
                $(
                    $stack1:tt
                    $(
                        [[$($stack2sgn:tt)?] [$($stack2val:tt)*]]
                        $($stackrest:tt)*
                    )?
                )?
            )?
        ],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['p'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("put2");
        $crate::code_to_char_pretty! {
            @match
            num: [[$($($($($stack2sgn)?)?)?)?] [$($($($($stack2val)*)?)?)?]],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @put @code_to_char_pretty
                    stack: [$($($($($stackrest)*)?)?)?],
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['p'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                    y: $($stack0)?,
                    x: $($($stack1)?)?,
                ],
                pst: [
                    orig: [[$($($($($stack2sgn)?)?)?)?] [$($($($($stack2val)*)?)?)?]],
                    debug: $debug,
                ],
            ],
        }
    };
    /*
          ##        #     ### #     # ###
         #  #      ###     #  ##    #  #
          ##        #      #  # #   #  #
         ###               #  #  #  #  #
        #   # #     #      #  #   # #  #
        #    #     ###     #  #    ##  #
         ###  #     #     ### #     # ###

        & : INI
        request single digit integer input from user, push to stack
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['&'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("ini");
        $crate::befunge_pm::get_integer! {
            socket: "befunge.input",
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @ini
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['&'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
         ##         #     ### #     #  #####
        #  #  #    ###     #  ##    # #     #
            ##      #      #  # #   # #
                           #  #  #  # #
                    #      #  #   # # #
                   ###     #  #    ## #     #
                    #     ### #     #  #####

        ~ : INC
        request single ASCII character input from user, push to stack
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['~'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("inc");
        $crate::befunge_pm::get_ascii! {
            socket: "befunge.input",
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @inc @get_ascii
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: [
                            pre: $cpre,
                            cur: ['~'],
                            pst: $cpst,
                        ],
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    /*
         #####      #     ####### #     # ######
        #     #    ###    #       ##    # #     #
        # ### #     #     #       # #   # #     #
        # ### #           #####   #  #  # #     #
        # ####      #     #       #   # # #     #
        #          ###    #       #    ## #     #
         #####      #     ####### #     # ######

        @ : END
        end program execution
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['@'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("end");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[closeonend]],
            expand: [
                $crate::befunge_pm::close_ui! {
                    socket: "befunge.output",
                }
                $crate::befunge_pm::close_ui! {
                    socket: "befunge.input",
                }
                #[cfg(feature = "socket_debug_default")]
                $crate::befunge_pm::close_ui! {
                    socket: "befunge.debug",
                }
            ],
            orelse: [
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[noflush]],
                    expand: [
                        const _: &str = "Program terminated successfully!";
                    ],
                    orelse: [
                        const _: &str = "Flushing program output.";
                        $crate::befunge_pm::flush_output! {
                            socket: "befunge.output",
                        }
                    ],
                }
            ],
        }
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[poststack]],
            expand: [
                const _: &str = "Stack at program '@':";
                $crate::dbg_print_stack! {
                    @printstack
                    stack: $stack,
                }
            ],
        }
    };
    /*
          ###       #     #     # #     #   ###
         #   #     ###    ##    # ##   ##  #   #
        #     #     #     # #   # # # # # #     #
        #     #           #  #  # #  #  # #     #
        #     #     #     #   # # #     # #     #
         #   #     ###    #    ## #     #  #   #
          ###       #     #     # #     #   ###

        0 : NM0
        push number 0 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['0'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm0");
        $crate::befunge_step! {
            @move
            stack: [[[pos] []] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['0'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
           #        #     #     # #     #   #
          ##       ###    ##    # ##   ##  ##
         # #        #     # #   # # # # # # #
           #              #  #  # #  #  #   #
           #        #     #   # # #     #   #
           #       ###    #    ## #     #   #
         #####      #     #     # #     # #####

        1 : NM1
        push number 1 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['1'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm1");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[]]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['1'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #####      #     #     # #     #  #####
        #     #    ###    ##    # ##   ## #     #
              #     #     # #   # # # # #       #
         #####            #  #  # #  #  #  #####
        #           #     #   # # #     # #
        #          ###    #    ## #     # #
        #######     #     #     # #     # #######

        2 : NM2
        push number 2 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['2'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm2");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['2'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #####      #     #     # #     #  #####
        #     #    ###    ##    # ##   ## #     #
              #     #     # #   # # # # #       #
         #####            #  #  # #  #  #  #####
              #     #     #   # # #     #       #
        #     #    ###    #    ## #     # #     #
         #####      #     #     # #     #  #####

        3 : NM3
        push number 3 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['3'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm3");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] [] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['3'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
        #           #     #     # #     # #
        #    #     ###    ##    # ##   ## #    #
        #    #      #     # #   # # # # # #    #
        #    #            #  #  # #  #  # #    #
        #######     #     #   # # #     # #######
             #     ###    #    ## #     #      #
             #      #     #     # #     #      #

        4 : NM4
        push number 4 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['4'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm4");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] [] [] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['4'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
        #######     #     #     # #     # #######
        #          ###    ##    # ##   ## #
        #           #     # #   # # # # # #
        ######            #  #  # #  #  # ######
              #     #     #   # # #     #       #
        #     #    ###    #    ## #     # #     #
         #####      #     #     # #     #  #####

        5 : NM5
        push number 5 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['5'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm5");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] [] [] [] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['5'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #####      #     #     # #     #  #####
        #     #    ###    ##    # ##   ## #     #
        #           #     # #   # # # # # #
        ######            #  #  # #  #  # ######
        #     #     #     #   # # #     # #     #
        #     #    ###    #    ## #     # #     #
         #####      #     #     # #     #  #####

        6 : NM6
        push number 6 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['6'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm6");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] [] [] [] [] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['6'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
        #######     #     #     # #     # #######
        #    #     ###    ##    # ##   ## #    #
            #       #     # #   # # # # #     #
           #              #  #  # #  #  #    #
          #         #     #   # # #     #   #
          #        ###    #    ## #     #   #
          #         #     #     # #     #   #

        7 : NM7
        push number 7 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['7'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm7");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] [] [] [] [] [] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['7'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #####      #     #     # #     #  #####
        #     #    ###    ##    # ##   ## #     #
        #     #     #     # #   # # # # # #     #
         #####            #  #  # #  #  #  #####
        #     #     #     #   # # #     # #     #
        #     #    ###    #    ## #     # #     #
         #####      #     #     # #     #  #####

        8 : NM8
        push number 8 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['8'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm8");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] [] [] [] [] [] [] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['8'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
         #####      #     #     # #     #  #####
        #     #    ###    ##    # ##   ## #     #
        #     #     #     # #   # # # # # #     #
         ######           #  #  # #  #  #  ######
              #     #     #   # # #     #       #
        #     #    ###    #    ## #     # #     #
         #####      #     #     # #     #  #####

        9 : NM9
        push number 9 to the stack
    */
    (
        @instr
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: ['9'],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("nm9");
        $crate::befunge_step! {
            @move
            stack: [[[pos] [[] [] [] [] [] [] [] [] []]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: $pre,
                cur: [
                    pre: $cpre,
                    cur: ['9'],
                    pst: $cpst,
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    /*
        #     # #     # #    # #     # ####### #     # #     #
        #     # ##    # #   #  ##    # #     # #  #  # ##    #
        #     # # #   # #  #   # #   # #     # #  #  # # #   #
        #     # #  #  # ###    #  #  # #     # #  #  # #  #  #
        #     # #   # # #  #   #   # # #     # #  #  # #   # #
        #     # #    ## #   #  #    ## #     # #  #  # #    ##
         #####  #     # #    # #     # #######  ## ##  #     #

        UNKNOWN
    */
    (
        @instr
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: $bridge:tt,
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: [$unknown:tt],
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("unk");
        $crate::befunge_error! {
            @unknowninstr
            instr: $unknown,
            row: ${count($pre)},
            col: ${count($cpre)},
            stack: $stack,
            dir: $dir,
        }
    };
    /*
         #####     #    #######  #####  #     #    ######  ######     #    #     #  #####  #     # #######  #####
        #     #   # #      #    #     # #     #    #     # #     #   # #   ##    # #     # #     # #       #     #
        #        #   #     #    #       #     #    #     # #     #  #   #  # #   # #       #     # #       #
        #       #     #    #    #       #######    ######  ######  #     # #  #  # #       ####### #####    #####
        #       #######    #    #       #     #    #     # #   #   ####### #   # # #       #     # #             #
        #     # #     #    #    #     # #     #    #     # #    #  #     # #    ## #     # #     # #       #     #
         #####  #     #    #     #####  #     #    ######  #     # #     # #     #  #####  #     # #######  #####

        CATCH BRANCHES
    */
    (
        @catch @arith
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: $stringmode:tt,
        bridge: [false],
        progstate: $progstate:tt,
        res: $res:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: arith");
        $crate::befunge_step! {
            @move
            stack: [$res $($stack)*],
            dir: $dir,
            stringmode: $stringmode,
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    (
        @catch @char_to_code
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: $stringmode:tt,
        bridge: [false],
        progstate: $progstate:tt,
        num: $num:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: char_to_code");
        $crate::befunge_step! {
            @move
            stack: [$num $($stack)*],
            dir: $dir,
            stringmode: $stringmode,
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    /*
         #####      #     ######   #####
        #     #    ###    #     # #     #
              #     #     #     # #
           ###            ######  #
           #        #     #       #
                   ###    #       #     #
           #        #     #        #####
                                          #######

        ? : PC_
    */
    (
        @catch @pc_
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        rand: $newdir:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: pc_");
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: $newdir,
            stringmode: [false],
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    /*
                    #      #####  ####### #######
                   ###    #     # #          #
                    #     #       #          #
          #### #          #  #### #####      #
         #    #     #     #     # #          #
         #    #    ###    #     # #          #
          #####     #      #####  #######    #
              #
         #    #
          ####

        g : GET
    */
    (
        @catch @get @splitrow
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        x: [$($x:tt)*],
        l: $l:tt,
        r: [$rh:tt $($rt:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: get0", ${count($x)});
        $crate::list_split_at_length_of! {
            @init
            lenof: [$($x)*],
            split: $rh,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @get @push
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: $progstate,
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    // Push numbers directly to the stack
    (
        @catch @get @push
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        l: $l:tt,
        r: [[[$($numsgn:tt)?] [$($numval:tt)*]] $($rt:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: get1", $($numsgn,)? ${count($numval)});
        macro_rules! sanitise_num_for_dbg {
            (
                num: [[$$(pos)?] [$$($$nv:tt)*]],
            ) => {
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[getdbg]],
                    expand: [
                        const _: &str = concat!(
                            "Conversion to number successful! Pushing ",
                            stringify!($${count($$nv)}),
                            " to the stack.",
                        );
                    ],
                }
            };
            (
                num: [[neg] [$$($$nv:tt)*]],
            ) => {
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[getdbg]],
                    expand: [
                        const _: &str = concat!(
                            "Conversion to number successful! Pushing ",
                            stringify!(-$${count($$nv)}),
                            " to the stack.",
                        );
                    ],
                }
            };
        }
        sanitise_num_for_dbg! {
            num: [[$($numsgn)?] [$($numval)*]],
        }
        $crate::befunge_step! {
            @move
            stack: [[[$($numsgn)?] [$($numval)*]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    // Translate instructions to numbers.
    (
        @catch @get @push
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        l: $l:tt,
        r: [$rh:tt $($rt:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: get2", $rh);
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[getdbg]],
            expand: [
                const _: &str = concat!("Indexing succesful! Got char: ", $rh);
            ],
        }
        $crate::char_to_code! {
            @match
            char: $rh,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @get @char_to_code
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: $progstate,
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @catch @get @char_to_code
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        num: [[$numsgn:tt] [$($numval:tt)*]],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: get3", ${count($numval)});
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[getdbg]],
            expand: [
                const _: &str = concat!(
                    "Conversion to number successful! Pushing ",
                    stringify!($numsgn),
                    " ",
                    ${count($numval)},
                    " to the stack.",
                );
            ],
        }
        $crate::befunge_step! {
            @move
            stack: [[[$numsgn] [$($numval)*]] $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    /*
                    #     ######  #     # #######
                   ###    #     # #     #    #
                    #     #     # #     #    #
        # ####            ######  #     #    #
         #    #     #     #       #     #    #
         #    #    ###    #       #     #    #
         #####      #     #        #####     #
         #
         #
         #

        p : PUT
    */
    (
        @catch @put @code_to_char_pretty
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: [$cur:tt],
                pst: [$($cpst:tt)*],
            ],
            pst: [$($pst:tt)*],
        ],
        y: [[$($ysgn:tt)?] [$($y:tt)*]],
        x: [[$($xsgn:tt)?] [$($x:tt)*]],
        char: [-$fst:tt],
        orig: $orig:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: put0");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[putdbg]],
            expand: [
                const _: &str = concat!(
                    "Putting value at: (",
                    ${count($y)},
                    ", ",
                    ${count($x)},
                    ")",
                );
            ],
        }
        macro_rules! befunge_step_put_coord_check {
            (
                xcheck: [$($x)* $$([])*],
                ycheck: [$($y)* $$([])*],
            ) => {
                $crate::list_split_at_length_of! {
                    @init
                    lenof: [$($y)*],
                    split: [$($pre)* [$($cpre)* $cur $($cpst)*] $($pst)*],
                    callback: [
                        name: $crate::befunge_step,
                        pre: [
                            @catch @put @splitrow @place
                            stack: $stack,
                            dir: $dir,
                            stringmode: [false],
                            bridge: [false],
                            progstate: [
                                pre: [$($pre)*],
                                cur: [
                                    pre: [$($cpre)*],
                                    cur: [$cur],
                                    pst: [$($cpst)*],
                                ],
                                pst: [$($pst)*],
                            ],
                            x: [$($x)*],
                            put: $orig,
                        ],
                        pst: [
                            debug: $debug,
                        ],
                    ],
                }
            };
            ($$($$_:tt)*) => {
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[putdbg]],
                    expand: [
                        const _: &str = concat!("Index was out of bounds!");
                    ],
                }
            };
        }
        befunge_step_put_coord_check! {
            xcheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
            ycheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
        }
    };
    (
        @catch @put @code_to_char_pretty
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: [$cur:tt],
                pst: [$($cpst:tt)*],
            ],
            pst: [$($pst:tt)*],
        ],
        y: [[$($ysgn:tt)?] [$($y:tt)*]],
        x: [[$($xsgn:tt)?] [$($x:tt)*]],
        char: [$fst:tt],
        orig: $orig:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: put0");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[putdbg]],
            expand: [
                const _: &str = concat!(
                    "Putting value at: (",
                    ${count($y)},
                    ", ",
                    ${count($x)},
                    ")",
                );
            ],
        }
        macro_rules! befunge_step_put_coord_check {
            (
                xcheck: [$($x)* $$([])*],
                ycheck: [$($y)* $$([])*],
            ) => {
                $crate::list_split_at_length_of! {
                    @init
                    lenof: [$($y)*],
                    split: [$($pre)* [$($cpre)* $cur $($cpst)*] $($pst)*],
                    callback: [
                        name: $crate::befunge_step,
                        pre: [
                            @catch @put @splitrow @place
                            stack: $stack,
                            dir: $dir,
                            stringmode: [false],
                            bridge: [false],
                            progstate: [
                                pre: [$($pre)*],
                                cur: [
                                    pre: [$($cpre)*],
                                    cur: [$cur],
                                    pst: [$($cpst)*],
                                ],
                                pst: [$($pst)*],
                            ],
                            x: [$($x)*],
                            put: $orig,
                        ],
                        pst: [
                            debug: $debug,
                        ],
                    ],
                }
            };
            ($$($$_:tt)*) => {
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[putdbg]],
                    expand: [
                        const _: &str = concat!("Index was out of bounds!");
                    ],
                }
            };
        }
        befunge_step_put_coord_check! {
            xcheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
            ycheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
        }
    };
    (
        @catch @put @code_to_char_pretty
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: [$cur:tt],
                pst: [$($cpst:tt)*],
            ],
            pst: [$($pst:tt)*],
        ],
        y: [[$($ysgn:tt)?] [$($y:tt)*]],
        x: [[$($xsgn:tt)?] [$($x:tt)*]],
        char: [$fst:tt, $snd:tt],
        orig: $orig:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: put0");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[putdbg]],
            expand: [
                const _: &str = concat!(
                    "Putting value at: (",
                    ${count($y)},
                    ", ",
                    ${count($x)},
                    ")",
                );
            ],
        }
        macro_rules! befunge_step_put_coord_check {
            (
                xcheck: [$($x)* $$([])*],
                ycheck: [$($y)* $$([])*],
            ) => {
                $crate::list_split_at_length_of! {
                    @init
                    lenof: [$($y)*],
                    split: [$($pre)* [$($cpre)* $cur $($cpst)*] $($pst)*],
                    callback: [
                        name: $crate::befunge_step,
                        pre: [
                            @catch @put @splitrow @place
                            stack: $stack,
                            dir: $dir,
                            stringmode: [false],
                            bridge: [false],
                            progstate: [
                                pre: [$($pre)*],
                                cur: [
                                    pre: [$($cpre)*],
                                    cur: [$cur],
                                    pst: [$($cpst)*],
                                ],
                                pst: [$($pst)*],
                            ],
                            x: [$($x)*],
                            put: $fst,
                        ],
                        pst: [
                            debug: $debug,
                        ],
                    ],
                }
            };
            ($$($$_:tt)*) => {
                $crate::dbg_maybe_expand! {
                    @dbg
                    debug: $debug,
                    lookfor: [[putdbg]],
                    expand: [
                        const _: &str = concat!("Index was out of bounds!");
                    ],
                }
            };
        }
        befunge_step_put_coord_check! {
            xcheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
            ycheck: [[] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] [] []],
        }
    };
    (
        @catch @put @splitrow @place
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        x: $x:tt,
        put: $put:tt,
        l: $l:tt,
        r: [$rh:tt $($rt:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: put1");
        $crate::list_split_at_length_of! {
            @init
            lenof: $x,
            split: $rh,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @put @splitcol @place
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: $progstate,
                    put: $put,
                    putpre: $l,
                    putpst: [$($rt)*],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @catch @put @splitcol @place
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: $cur:tt,
            pst: $pst:tt,
        ],
        put: $newcur:tt,
        putpre: [$($putpre:tt)*],
        putpst: [$($putpst:tt)*],
        l: [$($putcpre:tt)*],
        r: [$_cur:tt $($putcpst:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: put2");
        $crate::list_split_at_length_of! {
            @init
            lenof: $pre,
            split: [$($putpre)* [$($putcpre)* $newcur $($putcpst)*] $($putpst)*],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @put @splitrow @newps
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: [
                        pre: $pre,
                        cur: $cur,
                        pst: $pst,
                    ],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @catch @put @splitrow @newps
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: $cpre:tt,
                cur: $cur:tt,
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        l: $newpre:tt,
        r: [$newcur:tt $($newpst:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: put3");
        $crate::list_split_at_length_of! {
            @init
            lenof: $cpre,
            split: $newcur,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @put @newps
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    pre: $newpre,
                    pst: [$($newpst)*],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @catch @put @newps
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        pre: [$($pre:tt)*],
        pst: [$($pst:tt)*],
        l: [$($cpre:tt)*],
        r: [$cur:tt $($cpst:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: put4");
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[putdbg]],
            expand: [
                const _: &str = "Successfully reassembled program memory! Result:";
                $crate::befunge_stringify! {
                    @stringify @raw
                    lines: [$($pre)* [$($cpre)* $cur $($cpst)*] $($pst)*],
                }
            ],
        }
        $crate::befunge_step! {
            @move
            stack: $stack,
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: [
                pre: [$($pre)*],
                cur: [
                    pre: [$($cpre)*],
                    cur: [$cur],
                    pst: [$($cpst)*],
                ],
                pst: [$($pst)*],
            ],
            debug: $debug,
        }
    };
    /*
          ##        #     ### #     # ###
         #  #      ###     #  ##    #  #
          ##        #      #  # #   #  #
         ###               #  #  #  #  #
        #   # #     #      #  #   # #  #
        #    #     ###     #  #    ##  #
         ###  #     #     ### #     # ###

        & : INI
    */
    (
        @catch @ini
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        integer: $int:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: ini");
        $crate::befunge_step! {
            @move
            stack: [$int $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    /*
         ##         #     ### #     #  #####
        #  #  #    ###     #  ##    # #     #
            ##      #      #  # #   # #
                           #  #  #  # #
                    #      #  #   # # #
                   ###     #  #    ## #     #
                    #     ### #     #  #####

        ~ : INC
    */
    (
        @catch @inc @get_ascii
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        ascii: $ascii:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: inc0");
        $crate::char_to_code! {
            @match
            char: $ascii,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @catch @inc @char_to_code
                    stack: $stack,
                    dir: $dir,
                    stringmode: [false],
                    bridge: [false],
                    progstate: $progstate,
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @catch @inc @char_to_code
        stack: [$($stack:tt)*],
        dir: $dir:tt,
        stringmode: [false],
        bridge: [false],
        progstate: $progstate:tt,
        num: $num:tt,
        debug: $debug:tt,
    ) => {
        $crate::socket_debug_default!("catch: inc1");
        $crate::befunge_step! {
            @move
            stack: [$num $($stack)*],
            dir: $dir,
            stringmode: [false],
            bridge: [false],
            progstate: $progstate,
            debug: $debug,
        }
    };
    /*
        #     # ####### #     # ####### #     # ####### #     # #######
        ##   ## #     # #     # #       ##   ## #       ##    #    #
        # # # # #     # #     # #       # # # # #       # #   #    #
        #  #  # #     # #     # #####   #  #  # #####   #  #  #    #
        #     # #     #  #   #  #       #     # #       #   # #    #
        #     # #     #   # #   #       #     # #       #    ##    #
        #     # #######    #    ####### #     # ####### #     #    #

        MOVEMENT
    */
    // Move right
    (
        @move
        stack: $stack:tt,
        dir: [right],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: [$cur:tt],
                pst: [$cph:tt $($cpt:tt)*],
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: right => ", $cph);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: [right],
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: [$($pre)*],
                cur: [
                    pre: [$($cpre)* $cur],
                    cur: [$cph],
                    pst: [$($cpt)*],
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Move right (wrap)
    (
        @move
        stack: $stack:tt,
        dir: [right],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$cph:tt $($cpt:tt)*],
                cur: [$cur:tt],
                pst: [],
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: right => ", $cph);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: [right],
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: [$($pre)*],
                cur: [
                    pre: [],
                    cur: [$cph],
                    pst: [$($cpt)* $cur],
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Move left
    (
        @move
        stack: $stack:tt,
        dir: [left],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: [$($cpre:tt)+],
                cur: $cur:tt,
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::list_init_last! {
            @init
            list: [$($cpre)+],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @left
                    stack: $stack,
                    dir: [left],
                    stringmode: $stringmode,
                    bridge: $bridge,
                    pre: $pre,
                ],
                pst: [
                    cur: $cur,
                    cpst: $cpst,
                    pst: $pst,
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @left
        stack: $stack:tt,
        dir: [left],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        pre: [$($pre:tt)*],
        init: [$($init:tt)*],
        last: [$last:tt],
        cur: [$cur:tt],
        cpst: [$($cpst:tt)*],
        pst: $pst:tt,
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: left => ", $last);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: [left],
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: [$($pre)*],
                cur: [
                    pre: [$($init)*],
                    cur: [$last],
                    pst: [$cur $($cpst)*],
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Move left (wrap)
    (
        @move
        stack: $stack:tt,
        dir: [left],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: $pre:tt,
            cur: [
                pre: [],
                cur: $cur:tt,
                pst: $cpst:tt,
            ],
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::list_init_last! {
            @init,
            list: $cpst,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @leftwrap
                    stack: $stack,
                    dir: [left],
                    stringmode: $stringmode,
                    bridge: $bridge,
                    pre: $pre,
                ],
                pst: [
                    cur: $cur,
                    pst: $pst,
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @leftwrap
        stack: $stack:tt,
        dir: $dir:tt,
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        pre: [$($pre:tt)*],
        cur: [$cur:tt],
        init: [$($init:tt)+],
        last: [$last:tt],
        pst: $pst:tt,
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: left => ", $last);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: $dir,
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: [$($pre)*],
                cur: [
                    pre: [$cur $($init)+],
                    cur: [$last],
                    pst: [],
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Move down
    (
        @move
        stack: $stack:tt,
        dir: [down],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: [$($pre:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: [$cur:tt],
                pst: [$($cpst:tt)*],
            ],
            pst: [$psth:tt $($pstt:tt)*],
        ],
        debug: $debug:tt,
    ) => {
        $crate::list_split_at_length_of! {
            @init
            lenof: [$($cpre)*],
            split: $psth,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @down
                    stack: $stack,
                    dir: [down],
                    stringmode: $stringmode,
                    bridge: $bridge,
                    pre: [$($pre)* [$($cpre)* $cur $($cpst)*]],
                ],
                pst: [
                    pst: [$($pstt)*],
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @down
        stack: $stack:tt,
        dir: [down],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        pre: [$($pre:tt)*],
        l: [$($cpre:tt)*],
        r: [$cur:tt $($cpst:tt)*],
        pst: $pst:tt,
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: down => ", $cur);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: [down],
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: [$($pre)*],
                cur: [
                    pre: [$($cpre)*],
                    cur: [$cur],
                    pst: [$($cpst)*],
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Move down (wrap)
    (
        @move
        stack: $stack:tt,
        dir: [down],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: [$preh:tt $($pret:tt)*],
            cur: [
                pre: [$($cpre:tt)*],
                cur: [$cur:tt],
                pst: [$($cpst:tt)*],
            ],
            pst: [],
        ],
        debug: $debug:tt,
    ) => {
        $crate::list_split_at_length_of! {
            @init
            lenof: [$($cpre)*],
            split: $preh,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @downwrap
                    stack: $stack,
                    dir: [down],
                    stringmode: $stringmode,
                    bridge: $bridge,
                ],
                pst: [
                    pst: [$($pret)* [$($cpre)* $cur $($cpst)*]],
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @downwrap
        stack: $stack:tt,
        dir: [down],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        l: [$($cpre:tt)*],
        r: [$cur:tt $($cpst:tt)*],
        pst: $pst:tt,
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: down => ", $cur);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: [down],
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: [],
                cur: [
                    pre: [$($cpre)*],
                    cur: [$cur],
                    pst: [$($cpst)*],
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Move up
    (
        @move
        stack: $stack:tt,
        dir: [up],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: [$($pre:tt)+],
            cur: $cur:tt,
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::list_init_last! {
            @init
            list: [$($pre)+],
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @up0
                    stack: $stack,
                    dir: [up],
                    stringmode: $stringmode,
                    bridge: $bridge,
                    cur: $cur,
                ],
                pst: [
                    pst: $pst,
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @up0
        stack: $stack:tt,
        dir: [up],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        cur: [
            pre: [$($cpre:tt)*],
            cur: [$cur:tt],
            pst: [$($cpst:tt)*],
        ],
        init: $init:tt,
        last: [$last:tt],
        pst: [$($pst:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::list_split_at_length_of! {
            @init
            lenof: [$($cpre)*],
            split: $last,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @up1
                    stack: $stack,
                    dir: [up],
                    stringmode: $stringmode,
                    bridge: $bridge,
                    pre: $init,
                ],
                pst: [
                    pst: [[$($cpre)* $cur $($cpst)*] $($pst)*],
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @up1
        stack: $stack:tt,
        dir: [up],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        pre: [$($pre:tt)*],
        l: [$($cpre:tt)*],
        r: [$cur:tt $($cpst:tt)*],
        pst: $pst:tt,
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: up => ", $cur);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: [up],
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: [$($pre)*],
                cur: [
                    pre: [$($cpre)*],
                    cur: [$cur],
                    pst: [$($cpst)*],
                ],
                pst: $pst,
            ],
            debug: $debug,
        }
    };
    // Move up (wrap)
    (
        @move
        stack: $stack:tt,
        dir: [up],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        progstate: [
            pre: [],
            cur: $cur:tt,
            pst: $pst:tt,
        ],
        debug: $debug:tt,
    ) => {
        $crate::list_init_last! {
            @init
            list: $pst,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @upwrap0
                    stack: $stack,
                    dir: [up],
                    stringmode: $stringmode,
                    bridge: $bridge,
                ],
                pst: [
                    cur: $cur,
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @upwrap0
        stack: $stack:tt,
        dir: [up],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        init: [$($init:tt)*],
        last: [$last:tt],
        cur: [
            pre: [$($cpre:tt)*],
            cur: [$cur:tt],
            pst: [$($cpst:tt)*],
        ],
        debug: $debug:tt,
    ) => {
        $crate::list_split_at_length_of! {
            @init
            lenof: [$($cpre)*],
            split: $last,
            callback: [
                name: $crate::befunge_step,
                pre: [
                    @move @upwrap1
                    stack: $stack,
                    dir: [up],
                    stringmode: $stringmode,
                    bridge: $bridge,
                    pre: [[$($cpre)* $cur $($cpst)*] $($init)*],
                ],
                pst: [
                    debug: $debug,
                ],
            ],
        }
    };
    (
        @move @upwrap1
        stack: $stack:tt,
        dir: [up],
        stringmode: $stringmode:tt,
        bridge: $bridge:tt,
        pre: $pre:tt,
        l: [$($l:tt)*],
        r: [$rh:tt $($rt:tt)*],
        debug: $debug:tt,
    ) => {
        $crate::dbg_maybe_expand! {
            @dbg
            debug: $debug,
            lookfor: [[tracemove]],
            expand: [
                const _: &str = concat!("newcur: up => ", $rh);
            ],
        }
        $crate::befunge_step! {
            @instr
            stack: $stack,
            dir: [up],
            stringmode: $stringmode,
            bridge: $bridge,
            progstate: [
                pre: $pre,
                cur: [
                    pre: [$($l)*],
                    cur: [$rh],
                    pst: [$($rt)*],
                ],
                pst: [],
            ],
            debug: $debug,
        }
    };
}
