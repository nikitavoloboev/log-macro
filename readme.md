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

Then use like so:

```rust
let animals = vec!["cat", "dog", "horse", "zebra"];
let set: std::collections::HashSet<_> = animals.into_iter().collect();
log!(set);
```

Will print:

```
set: {"horse", "dog", "zebra", "cat"}
```

## Implementation

Exported macro code is simply this:

```rust
#[macro_export]
macro_rules! log {
    ($var:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("{}: {:?}", stringify!($var), $var);
        }
    };
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

### ♥️

[Support on GitHub](https://github.com/sponsors/nikitavoloboev) or look into [other projects](https://nikiv.dev/projects).

[![MIT](http://bit.ly/mitbadge)](https://choosealicense.com/licenses/mit/) [![Twitter](http://bit.ly/nikitatweet)](https://twitter.com/nikitavoloboev)
