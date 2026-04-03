/// `dlog!` — debug-only logging macro.
///
/// Expands to `eprintln!` in debug builds and compiles away entirely in
/// release builds, so you can leave calls in place when submitting to
/// LeetCode or any other judge without paying any runtime cost.
///
/// # Usage
/// ```rust
/// use learning::dlog;
/// dlog!("robot {} fires left, range {}", 1, 34);
/// ```
#[macro_export]
macro_rules! dlog {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*);
    };
}
