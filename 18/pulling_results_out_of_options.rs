use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("Thr first doubled is {:?}", double_first(numbers));
    println!("Thr first doubled is {:?}", double_first(empty));
    println!("Thr first doubled is {:?}", double_first(strings));
}