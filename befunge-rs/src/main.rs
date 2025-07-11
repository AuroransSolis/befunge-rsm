#![recursion_limit = "16777216"]
#![feature(macro_metavar_expr)]

befunge_dm::befunge! {
    file: "sieve-of-eratosthenes.bfg",
    // debug: [[postinit] [getdbg] [putdbg] [tracemove]],
    // debug: [[postinit] [tracemove]],
    // debug: [[getdbg] [putdbg] [tracemove]],
    // debug: [[closeonend]],
    // debug: [[getdbg] [putdbg]],
}

fn main() {}
