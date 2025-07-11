# What do you want this time

Sometimes I feel like I've understated my message that declarative macros are ***powerful***. I'm
not sure if that's actually true that it hasn't been received that way, but I still do find many
cases of people pulling in `syn` and `quote` when they aren't needed and could've written a maybe
50 line declarative macro and called it a day. So to try and really drive the point home that
declarative macros are as powerful as I say they are, I made a

# Befunge 93 Interpreter in (mostly) declarative macros

Yes, you read that correctly.

The Befunge 93 specification has 26 instructions, and of these, only 7 of these are "contaminated"
with procedural macros. The first step in initialisation is too, and so are a couple other things,
but I think my record is pretty good overall:

- `/`: requires proc macro for divide by zero case
- `%`: requires proc macro for modulus by zero case
- `?`: requires proc macro for access to RNG
- `.`: requires proc macro to output an integer
- `,`: requires proc macro to output an ASCII character
- `&`: requires proc macro to get single-digit integer input
- `~`: requires proc macro to get single ASCII character input

Additionally, because of macro expansion order (outermost to innermost) we run into the issue that
in order to begin running a program, we need the output of `include!()` or `include_str!()` or
similar to be used as input to another macro. This can only be accomplished with a callback, and
seeing as neither macro offers such capabilities I had to write my own with it.

In total, these are all the proc macros I found useful or necessary for this project:

|                       Name | Necessary? | Notes                                                                                                                                                                                              |
|----------------------------|:----------:|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `stringify_with_callback!` |         No | Does what it says on the tin. Not strictly necessary, but useful for debugging purposes. Specifically in this project it is used to print out the state of program memory when execution finishes. |
| `div_by_zero!`             |        Yes | Required by program specification. When division by zero occurs, the user should be prompted to enter the desired result.                                                                          |
| `mod_by_zero!`             |        Yes | Required for the same reason as `div_by_zero!()` but occurring on modulus by zero.                                                                                                                 |
| `socket_debug!`            |         No | Used to output debugging information during execution.                                                                                                                                             |
| `choose_random!`           |        Yes | Required for the `?` instruction.                                                                                                                                                                  |
| `print_integer!`           |        Yes | Required for the `.` instruction.                                                                                                                                                                  |
| `print_ascii!`             |        Yes | Required for the `,` instruction.                                                                                                                                                                  |
| `get_integer!`             |        Yes | Required for the `&` instruction.                                                                                                                                                                  |
| `get_ascii!`               |        Yes | Required for the `~` instruction.                                                                                                                                                                  |
| `close_ui!`                |         No | Used to close interface programs on `@` with `[closeonend]` debug flag.                                                                                                                            |
| `flush_output!`            |        Yes | Used to force interface programs to flush their output buffers on `@`.                                                                                                                             |
| `befunge_input!`           |        Yes | Used to read a file as a stream of token literals.                                                                                                                                                 |

# Wait hold up just a moment

I think you might be justified in being confused about something I've sort of been dodging: how am
I doing input and output when this is all being run in `macro_rules!`? Well, mon chou, let me tell
you!

The compiler has very limited opportunities to output arbitrary information when parsing the AST of
a crate. You can print things out, but you can't keep a line buffer, and you also have no way to get
input. So to resolve that, whenever input is required for the interpreter I call a proc macro that
attempts to talk to an interface program (`befunge-if`) over a local socket (the exact type
determined by the behaviour of the `interprocess` crate). Compilation is then halted while the proc
macro waits for a response on this socket, and when it eventually receives it, macro expansion can
continue.

On that note, here's

# How you run this pile of garbage

1. Check if your program uses any of the I/O instructions mentioned above. If yes, you will need to
   run `befunge-if` on `befunge.input` and/or `befunge.output`. You can also just do this
   unconditionally. This can be done as `cargo run --bin befunge-if -- --socket=befunge.socketname`
   from the base of this repository.
2. Navigate to `befunge-rs` and edit `src/main.rs` to point to the Befunge file you want to run.
   Also, set the debugging flags.
3. Decide if you want to run with debug I/O. If yes, remember to add
   `--features="socket_debug_default"` to your build/check/expand command.
4. Run `cargo build`, `cargo check`, or `cargo expand`. This will execute the Befunge interpreter.

# How does it work though???

With that out of the way, let's talk a little bit about my rationale and the how-to of some things
within the interpreter.

## Numbers and maths

With macros, you can't just do $1 + 1$ literally - that would just get matched as an `:expr`, a
`:tt :tt :tt`, etc. It wouldn't get evaluated until `const` eval time, and that's after macro
expansion time. So what can you do instead? How can you represent numbers in a way that you can
actually work with them in a `macro_rules!` situation? Well, there's a number of ways, but the one
I chose is signed magnitude base 1 numbers.

In this format, numbers are represented as a single `:tt` that contains a sign token tree and a
magnitude token tree, which should be filled with a number of other token trees equal to the
absolute value of that number. For example, the number 2 would be `[[pos] [[] []]]`, the number
-5 would be `[[neg] [[] [] [] [] []]]`, and so on. I've also generally allowed it such that `[]`
and `[pos]` are treated the same for the sign. Operations are relatively simple from there, except
for a handful of cases:

1. Addition of negative numbers to positive numbers
2. Subtraction of positive numbers from positive numbers
3. Subtraction of negative numbers from negative numbers
3. Division
4. Modulus

Let's talk about how each of these can be accomplished in turn.

### Cases 1, 2, and 3

These are all almost identical cases, but the challenge here is when you actually need to _remove_
something from the magnitude of one number and that amount is variable. There's two ways to do this:
first, you can write a macro that successively removes token trees from the magnitude of the LHS
and RHS until the magnitude of the RHS is 0; second, you can define and immediately call an ad-hoc
macro that does the subtraction. I opted for the second method. This looks like this:
```rs
macro_rules! sub {
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
    // other branches needed
}
```
At its face this is somewhat complicated, but it makes more sense I think if we look at an example
expansion (with extra comments from by me). Let's try, say, $7 - 4$:
```rs
sub! {
    @sub
    a: [[pos] [[] [] [] [] [] [] []]],
    b: [[pos] [[] [] [] []]],
    callback: [
        name: name,
        pre: [],
        pst: [],
    ],
}

// expands to

macro_rules! exec_sub {
    // This branch should happen if a >= b
    (
        @sub
        a: [[] [] [] [] $($diff:tt)*],
        b: $_:tt,
    ) => {
        name! {
            res: [[pos] [$($diff)*]],
        }
    };
    // This branch should happen if a < b. The result is equal to -(b - a)
    (
        @sub
        a: $_:tt,
        b: [[] [] [] [] [] [] [] $($diff:tt)*],
    ) => {
        name! {
            res: [[neg] [$($diff)*]],
        }
    }
}

exec_sub! {
    a: [[] [] [] [] [] [] []],
    b: [[] [] [] []],
}

// expands to

name! {
    res: [[pos] [[] [] []]],
}
```
This reduces subtraction to a fixed depth of 2 expansions. Great! However, it does come with a
memory usage cost. We can just adjust the placements of `[$(pos)?]`/`[pos]` and `[neg]` to suit the
other case ($(-a) - (-b)$), and then we can make the addition case just call the subtraction macro.

### Division and modulus

The ways to do division and modulus is very similar to the ways we can do subtraction, except we
need to remove a variable number of tokens from the magnitude. As such we would need to build up
all the rules for the macro first before defining it, and so either way we do it execution will be
linear expansion time with respect to the divisor/modulus base. Because of that, for my
implementation I decided to go with the looping subtraction approach. That looks like this:
```rs
macro_rules! divmod {
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
    // also needs a `compile_error!()` for div/mod by 0
}
```
Using the same numbers as before, this expands like
```rs
divmod! {
    @divmod
    a: [[] [] [] [] [] [] []],
    b: [[] [] [] []],
    callback: [
        name: name,
        pre: [],
        pst: [],
    ],
}

// expands to

macro_rules! arith_div_mod_exec {
    (
        @divmod
        left: [[] [] [] [] $($rest:tt)*],
        div: [$($div:tt)*],
    ) => {
        airth_div_mod_exec! {
            @divmod
            left: [$($rest)*],
            div: [$($div)* []],
        }
    };
    (
        @divmod
        left: [$($mod:tt)*],
        div: $div:tt,
    ) => {
        name! {
            div: $div,
            mod: [$$($${ignore($$rest)} [])*],
        }
    };
}

arith_div_mod_exec! {
    @divmod
    left: [[] [] [] [] [] [] []],
    div: [],
}

// expands to

arith_div_mod_exec! {
    @divmod
    left: [[] [] []],
    div: [[]],
}

// expands to

name! {
    div: [[]],
    mod: [[] [] []],
}
```

## The interpreter itself

The interpreter keeps track of these things:

- The stack
- Currently in stringmode?
- Bridge this instruction?
- Program memory
- Debugging information

Everything except program memory is tracked in a fairly straightforward way. In order to be able to
walk the grid properly, I keep track of it as such (in struct notation):
```rs
struct ProgMem {
    previous_rows: Vec<Vec<Cell>>,
    previous_in_current_row: Vec<Cell>,
    current_cell: Cell,
    after_in_current_row: Vec<Cell>,
    after_rows: Vec<Vec<Cell>>,
}
```
In `macro_rules!`, this looks like
```
pre: $pre:tt,
cur: [
    pre: $cpre:tt,
    cur: [$cur:tt],
    pst: $cpst:tt,
],
pst: $pst:tt,
```
I can then generally walk the grid by taking heads and tails of various `:tt`s/lists of `:tt`s (see
the `list_*` macros and the movement section of the `befunge_step!` macro). Each cell may contain
either a character literal or a signed magnitude base 1 number, though the stack may only ever
contain the latter.

# And that's it I guess

Cheers for reading all this, if indeed you have. I know that it won't ever be directly useful, but I
do hope at least that the techniques used in here can be instructive.

I highly recommend visiting the Esolang wiki page linked to in the documentation of `befunge-dm` for
programs to run this with, or writing your own! Befunge is a surprisingly fun language to play
around with.

## License

The code in this repository is licensed under the European Union Public License version 1.2 or later
([LICENSE-EUPL](LICENSE-EUPL) or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, the contributor (as defined in the EUPL license), shall be licensed as above without
any additional terms or conditions.
