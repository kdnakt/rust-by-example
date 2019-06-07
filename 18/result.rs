fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first = first_number_str.parse::<i32>().unwrap();
    let second = second_number_str.parse::<i32>().unwrap();
    first * second
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);// panicked ParseIntError: InvalidDigit
}