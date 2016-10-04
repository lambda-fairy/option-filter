# option-filter [![Cargo](https://img.shields.io/crates/v/option-filter.svg)](https://crates.io/crates/option-filter)

This crate adds a `.filter()` method to `Option<T>`.

See the [Rust RFCs issue][1] for the motivation behind this crate.

[1]: https://github.com/rust-lang/rfcs/issues/1485

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
