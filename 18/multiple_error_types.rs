fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // generate error 1
    2 * first.parse::<i32>().unwrap() // generate error 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    //let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    //println!("The first doubled is {}", double_first(empty));
    // error 1: thread main panicked at called Option::unwrap() on None value.
    println!("The first doubled is {}", double_first(strings));
    // error 2: Result::unrap() on an Err value: ParseIntError
}