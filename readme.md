# log_macro [<img alt="crates.io" src="https://img.shields.io/crates/v/log_macro.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/log_macro)

> Macro to print variable name and value only (stripped from release builds)

## Install

```
cargo add log_macro
```

## Use

Add this to top of file:

```rust
#[macro_use]
extern crate log_macro;
```

Possible uses and outputs:

```rust
// print string only
log!("hello"); // -> hello

// print variable
let animals = vec!["cat", "dog"];
log!(animals); // -> animals: ["cat", "dog"]

// print multiple variables
let animals = vec!["cat", "dog"];
let fish = vec!["salmon", "tuna"];
log!(animals, fish);
// each variable logged on new line
// -> animals: ["cat", "dog"]
// -> fish: ["salmon", "tuna"]
```

## Implementation

Exported macro code is this:

```rust
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
```

## Contribute

The tasks to do are outlined in [existing issues](../../issues) and in [tasks below](#tasks) (sorted by priority).

If issue/idea you have is not there, [open new issue](../../issues/new/choose) or [start discussion](../../discussions).

Any PR with code/doc improvements is welcome. ✨

Join [Discord](https://discord.com/invite/TVafwaD23d) for more indepth discussions on this repo and [others](https://github.com/nikitavoloboev#src).

## Tasks

- [add ability to log multiple variables at once as mentioned](https://www.reddit.com/r/rust/comments/15wd5u6/comment/jx081pk/?utm_source=share&utm_medium=web2x&context=3)
- [add support for multiple values or different ways of formatting](https://www.reddit.com/r/rust/comments/15wd5u6/comment/jx074g9/?utm_source=share&utm_medium=web2x&context=3)
- [get the same level of utilities that Python’s formatted string literals have](https://www.reddit.com/r/rust/comments/15wd5u6/comment/jx109os/?utm_source=share&utm_medium=web2x&context=3)
  - `> print(f”operation: {2+1=}”)` -> `operation: 2+1=3`
- decide if macro should be stripped from release builds or not
  - currently it's stripped similar to `dbg!`
  - potentially provide another near identical macro to keep similar logs but also in release mode?

### ♥️

[Support on GitHub](https://github.com/sponsors/nikitavoloboev) or look into [other projects](https://nikiv.dev/projects).

[![MIT](http://bit.ly/mitbadge)](https://choosealicense.com/licenses/mit/) [![Twitter](http://bit.ly/nikitatweet)](https://twitter.com/nikitavoloboev)
