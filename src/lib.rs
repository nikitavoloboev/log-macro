// Macro to print variable name and value only (stripped from release builds)
// got from: https://www.reddit.com/r/rust/comments/15wd5u6/comment/jx0pl1o/
#[macro_export]
macro_rules! log {
    ( $val:expr $(,)? ) => {
        match $val {
            tmp => {
                if ::std::stringify!($val).starts_with("\"") {
                    // Remove quotes for string literals
                    ::std::eprintln!("{}", ::std::stringify!($val).trim_matches('\"'));
                } else {
                    ::std::eprintln!("{}: {:?}", ::std::stringify!($val), &tmp);
                }
                tmp
            }
        }
    };
    ( $($val:expr),+ $(,)? ) => {
        ($($crate::log!($val)),+,)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::io::{self, Read, Write};

    #[test]
    fn print_string() {
        log!("hello"); // -> hello
        assert_eq!("hello", "hello.");
    }

    #[test]
    fn print_variable() {
        let animals = vec!["cat", "dog", "horse", "zebra"];
        log!(animals); // -> animals: ["cat", "dog", "horse", "zebra"]
        assert_eq!("fail", "f");
    }
}
