# option-filter [![Cargo](https://img.shields.io/crates/v/option-filter.svg)](https://crates.io/crates/option-filter)

This crate adds a `.filter()` method to `Option<T>`, for older versions of Rust that don't provide it.

**Note: [`Option::filter`][std] was added to the standard library in [Rust 1.27][rust]. Unless you need to support older versions of Rust, you do not need to use this crate.**

[std]: https://doc.rust-lang.org/std/option/enum.Option.html#method.filter
[rust]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1270-2018-06-21

## Usage

To use it, add `option-filter` to your `Cargo.toml`:

```toml
[dependencies]
option-filter = "1.0"
```

Then import the extension trait:

```rust,ignore
extern crate option_filter;
use option_filter::OptionFilterExt;
```

Now you can filter your `Option`s!

```rust
let answer = Some(42);
assert_eq!(answer.filter(|x| *x == 42), Some(42));
assert_eq!(answer.filter(|x| *x == 43), None);
```
