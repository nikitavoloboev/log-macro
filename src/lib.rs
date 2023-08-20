// Macro to print variable name and value only (stripped from release builds)
// #[macro_export]
// macro_rules! log {
//     ($var:expr) => {
//         #[cfg(debug_assertions)]
//         {
//             println!("{}: {:?}", stringify!($var), $var);
//         }
//     };
// }

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
    use std::panic;

    #[test]
    fn single_value_test() {
        let output = panic::catch_unwind(|| {
            log!(42);
        })
        .err()
        .unwrap()
        .downcast_ref::<&str>()
        .unwrap();

        assert_eq!(*output, "42: 42");
    }

    #[test]
    fn multiple_values_test() {
        let output = panic::catch_unwind(|| {
            log!(42, "hello", true);
        })
        .err()
        .unwrap()
        .downcast_ref::<&str>()
        .unwrap();

        // We expect the tuple to have each value logged.
        assert_eq!(*output, "(42: 42, \"hello\": \"hello\", true: true)");
    }
}
