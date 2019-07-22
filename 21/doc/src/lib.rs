/// First line is a short summary describing function.
/// 
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. 
/// 
/// ```
/// let result = doc::add(2,3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usuallly doc comments may include sections "Examples", "Panics" and "Failures".
/// 
/// The next function devides two numbers.
/// 
/// # Examples
/// 
/// ```
/// let result = doc::div(10, 2);
/// assert_eq!(result, 5);
/// ```
/// 
/// # Panics
/// 
/// The functions panics if the second argument is zero.
/// 
/// ```rust,should_panic
/// // panics on division by zero
/// doc::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division-by-zero error");
    }

    a / b
}