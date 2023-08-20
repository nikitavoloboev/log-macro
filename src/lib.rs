// Macro to print variable name and value only (stripped from release builds)
// got from: https://www.reddit.com/r/rust/comments/15wd5u6/comment/jx0pl1o/
#[macro_export]
macro_rules! log{
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                ::std::eprintln!("{}: {:?}", ::std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::log!($val)),+,)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_string() {
        log!("testing");
        assert_eq!("fail", "test");
    }
}
