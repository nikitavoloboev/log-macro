// Macro to print nicely formatted debug info to stderr
// can print just a string:
// log!("hello"); // -> hello
// or variable:
// let animals = vec!["cat", "dog", "horse", "zebra"];
// log!(animals); // -> animals: ["cat", "dog", "horse", "zebra"]
// or multiple variables (each variable on new line):
// let animals = vec!["cat", "dog", "horse", "zebra"];
// let humans = vec!["nikita", "edward"];
// log!(animals, humans); // -> animals: ["cat", "dog", "horse", "zebra"] humans: ["nikita", "edward"]
// macros are stripped from release builds too
// credit goes to Icarium-Lifestealer: https://www.reddit.com/r/rust/comments/15wd5u6/comment/jx0pl1o/
#[macro_export]
macro_rules! log {
    // Single literal string case
    ( $val:expr $(,)? ) => {{
        if ::std::stringify!($val).starts_with("\"") {
            // Remove quotes for string literals
            ::std::eprintln!("{}", ::std::stringify!($val).trim_matches('\"'));
        } else {
            ::std::eprintln!("{}: {:?}", ::std::stringify!($val), $val);
        }
        $val
    }};

    // Multiple variables case
    ( $($val:expr),+ $(,)? ) => {{
        $(
            $crate::log!($val);
        )+
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn print_multiple_variables() {
        let animals = vec!["cat", "dog", "horse", "zebra"];
        let humans = vec!["nikita", "edward"];
        log!(animals, humans);
        // -> animals: ["cat", "dog", "horse", "zebra"]
        // -> humans: ["nikita", "edward"]
        assert_eq!("fail", "f");
    }
}
