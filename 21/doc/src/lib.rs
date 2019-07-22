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

/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compileable.
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = doc::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #     try_main().unwrap(); // calling try_main and unwrapping
/// #                          // so that test will panic in case of error
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division-by-zero"))
    } else {
        Ok(a / b)
    }
}