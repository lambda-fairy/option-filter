/// This crate adds a `.filter()` method to `Option<T>`.
///
/// To use it, add `option-filter` to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// option-filter = "1.0"
/// ```
///
/// Then import the extension trait:
///
/// ```rust,ignore
/// extern crate option_filter;
/// use option_filter::OptionFilterExt;
/// ```
///
/// Now you can filter your `Option`s!
///
/// ```rust
/// # use option_filter::OptionFilterExt;
/// let answer = Some(42);
/// assert_eq!(answer.filter(|x| *x == 42), Some(42));
/// assert_eq!(answer.filter(|x| *x == 43), None);
/// ```

/// Extension trait for adding a `.filter()` method to `Option<T>`.
///
/// This trait is intended for extending `Option<T>` only, and should
/// not be implemented for other types.
pub trait OptionFilterExt {
    /// The inner type of the `Option`.
    ///
    /// This is given an unwieldy name so that it's unlikely to conflict
    /// with other associated types.
    type OptionFilterInner;
    fn filter<F: FnOnce(&Self::OptionFilterInner) -> bool>(self, F) -> Self;
}

impl<T> OptionFilterExt for Option<T> {
    type OptionFilterInner = T;
    fn filter<F: FnOnce(&T) -> bool>(self, callback: F) -> Option<T> {
        match self {
            None => None,
            Some(x) => if callback(&x) { Some(x) } else { None }
        }
    }
}
