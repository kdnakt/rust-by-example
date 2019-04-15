static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;
    let m = 9;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    println!("{} is {}", m, if is_big(m) { "big" } else { "small" });

    //THRESHOLD = 5;  // error: invalid left-hand side expression
}