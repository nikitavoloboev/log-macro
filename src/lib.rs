// Macro to print variable name and value only (stripped from release builds)
#[macro_export]
macro_rules! log {
    ($var:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("{}: {:?}", stringify!($var), $var);
        }
    };
}
