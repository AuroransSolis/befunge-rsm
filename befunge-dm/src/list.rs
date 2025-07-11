#[macro_export]
/// Gives the last and init of a list
///
/// Gives an empty `init` if the list is of length 1.
///
/// Call examples:
/// ```
/// # use befunge_dm::list_init_last;
/// #
/// # const fn const_str_eq(a: &'static str, b: &'static str) -> bool {
/// #     let a = a.as_bytes();
/// #     let b = b.as_bytes();
/// #     if a.len() != b.len() {
/// #         return false;
/// #     }
/// #     let mut i = 0;
/// #     while i < a.len() {
/// #         if a[i] != b[i] {
/// #             return false;
/// #         } else {
/// #             i += 1;
/// #         }
/// #     }
/// #     true
/// # }
/// #
/// macro_rules! helper {
///     (
///         init: $init:tt,
///         last: [$last:tt],
///     ) => {
///         const INIT: &str = stringify!($init);
///         const LAST: &str = stringify!($last);
///     };
/// }
///
/// // anonymous namespace
/// const _: () = {
///     list_init_last! {
///         @init
///         list: [a b c d e],
///         callback: [
///             name: helper,
///             pre: [],
///             pst: [],
///         ],
///     }
///
///     assert!(const_str_eq(INIT, "[a b c d]"));
///     assert!(const_str_eq(LAST, "e"));
/// };
///
/// const _: () = {
///     list_init_last! {
///         @init
///         list: [a],
///         callback: [
///             name: helper,
///             pre: [],
///             pst: [],
///         ],
///     }
///     
///     assert!(const_str_eq(INIT, "[]"));
///     assert!(const_str_eq(LAST, "a"));
/// };
/// ```
macro_rules! list_init_last {
    (
        @init
        list: [],
        callback: $callback:tt,
    ) => {
        compile_error! {
            concat! {
                "Attempted to 'g' init and last of empty list. Callback:\n",
                stringify($callback),
            }
        }
    };
    (
        @init
        list: $list:tt,
        callback: $callback:tt,
    ) => {
        $crate::list_init_last! {
            @init @inner
            init: [],
            last: $list,
            callback: $callback,
        }
    };
    (
        @init @inner
        init: $init:tt,
        last: [$last:tt],
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            init: $init,
            last: [$last],
            $($pst)*
        }
    };
    (
        @init @inner
        init: [$($init:tt)*],
        last: [$lh:tt $($lt:tt)+],
        callback: $callback:tt,
    ) => {
        $crate::list_init_last! {
            @init @inner
            init: [$($init)* $lh],
            last: [$($lt)+],
            callback: $callback,
        }
    };
}

#[macro_export]
/// Splits one list using the length of another as reference.
///
/// Call examples:
/// ```
/// # use befunge_dm::list_split_at_length_of;
/// #
/// # const fn const_str_eq(a: &'static str, b: &'static str) -> bool {
/// #     let a = a.as_bytes();
/// #     let b = b.as_bytes();
/// #     if a.len() != b.len() {
/// #         return false;
/// #     }
/// #     let mut i = 0;
/// #     while i < a.len() {
/// #         if a[i] != b[i] {
/// #             return false;
/// #         } else {
/// #             i += 1;
/// #         }
/// #     }
/// #     true
/// # }
/// #
/// macro_rules! helper {
///     (
///         l: $lside:tt,
///         r: $rside:tt,
///     ) => {
///         const LSIDE: &str = stringify!($lside);
///         const RSIDE: &str = stringify!($rside);
///     };
/// }
///
/// // anonymous namespace
/// const _: () = {
///     list_split_at_length_of! {
///         @init
///         lenof: [a b c d e],
///         split: [a b c d e f g],
///         callback: [
///             name: helper,
///             pre: [],
///             pst: [],
///         ],
///     }
///
///     assert!(const_str_eq(LSIDE, "[a b c d e]"));
///     assert!(const_str_eq(RSIDE, "[f g]"));
/// };
///
/// const _: () = {
///     list_split_at_length_of! {
///         @init
///         lenof: [a b c d e],
///         split: [a b c d e],
///         callback: [
///             name: helper,
///             pre: [],
///             pst: [],
///         ],
///     }
///     
///     assert!(const_str_eq(LSIDE, "[a b c d e]"));
///     assert!(const_str_eq(RSIDE, "[]"));
/// };
///
/// const _: () = {
///     list_split_at_length_of! {
///         @init
///         lenof: [],
///         split: [a b c d e],
///         callback: [
///             name: helper,
///             pre: [],
///             pst: [],
///         ],
///     }
///     
///     assert!(const_str_eq(LSIDE, "[]"));
///     assert!(const_str_eq(RSIDE, "[a b c d e]"));
/// };
/// ```
macro_rules! list_split_at_length_of {
    (
        @init
        lenof: $lenof:tt,
        split: $split:tt,
        callback: $callback:tt,
    ) => {
        $crate::list_split_at_length_of! {
            @split
            lenof: $lenof,
            l: [],
            r: $split,
            callback: $callback,
        }
    };
    (
        @split
        lenof: [$lenofh:tt $($lenoft:tt)*],
        l: [$($l:tt)*],
        r: [$rh:tt $($rt:tt)*],
        callback: $callback:tt,
    ) => {
        $crate::list_split_at_length_of! {
            @split
            lenof: [$($lenoft)*],
            l: [$($l)* $rh],
            r: [$($rt)*],
            callback: $callback,
        }
    };
    (
        @split
        lenof: [],
        l: $l:tt,
        r: $r:tt,
        callback: [
            name: $name:path,
            pre: [$($pre:tt)*],
            pst: [$($pst:tt)*],
        ],
    ) => {
        $name! {
            $($pre)*
            l: $l,
            r: $r,
            $($pst)*
        }
    };
    (
        @split
        lenof: [$($lenof:tt)+],
        split: [],
        callback: [
            name: $name:path,
            pre: $pre:tt,
            pst: $pst:tt,
        ],
    ) => {
        compile_error! {
            concat! {
                "Failed to split list copying length of another! Callback:\n- Name: ",
                stringify!($name),
                "\n- Pre: ",
                stringify!($pre),
                "\n- Pst: ",
                stringify!($pst),
            }
        }
    };
}
