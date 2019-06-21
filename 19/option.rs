fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend:i32, divisor:i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    let none: Option<i32> = None;
    let _quivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // panicked at called Option::unwrap() on a None value. src/libcore/option.rs:345:21
    //println!("{:?} unwraps to {:?}", none, none.unwrap());
}