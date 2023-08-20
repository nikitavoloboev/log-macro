/// Macro to print nicely formatted debug info. Macro is stripped from release builds.
///
/// print string only:
/// ```rust
/// log!("hello"); // -> hello
/// ```
///
/// print variable + its value:
/// ```rust
/// let animals = vec!["cat", "dog"];
/// log!(animals); // -> animals: ["cat", "dog"]
/// ```
/// print multiple variables (each variable printed on new line):
/// ```rust
/// let animals = vec!["cat", "dog"];
/// let fish = vec!["salmon", "tuna"];
/// log!(animals, fish); -> animals: ["cat", "dog"]
/// ```
/// credit for improvements goes to Icarium-Lifestealer: https://www.reddit.com/r/rust/comments/15wd5u6/comment/jx0pl1o/
#[macro_export]
macro_rules! log {
    // Single literal string case
    ( $val:expr $(,)? ) => {{
        if ::std::stringify!($val).starts_with("\"") {
            // Remove quotes for string literals
            ::std::eprintln!("{}", ::std::stringify!($val).trim_matches('\"'));
        } else {
            // Print using a reference to avoid moving the value
            ::std::eprintln!("{}: {:?}", ::std::stringify!($val), &$val);
        }
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

    // TODO: actually test that output is correctly logged
    // not sure how, https://crates.io/crates/duct pehaps?
    #[test]
    fn print_string() {
        log!("hello"); // -> hello
        assert_eq!("hello", "hello.");
    }

    #[test]
    fn print_variable() {
        let animals = vec!["cat", "dog"];
        log!(animals); // -> animals: ["cat", "dog"]
        assert_eq!("fail", "f");
    }

    #[test]
    fn print_multiple_variables() {
        let animals = vec!["cat", "dog"];
        let fish = vec!["salmon", "tuna"];
        log!(animals, fish);
        // -> animals: ["cat", "dog"]
        // -> fish: ["salmon", "tuna"]
        assert_eq!("fail", "f");
    }
}
