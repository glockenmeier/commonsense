//! Commonsense: ergonomic rituals for Rust.

/// Create a `Vec<String>` from string literals or expressions.
///
/// # Examples
/// ```
/// use commonsense::svec;
///
/// let words = svec!["hello", "world"];
/// assert_eq!(words, vec!["hello".to_string(), "world".to_string()]);
/// ```
#[macro_export]
macro_rules! svec {
    ($($x:expr),* $(,)?) => {
        vec![$($x.to_string()),*]
    };
}

