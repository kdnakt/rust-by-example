use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt= vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    let opt = opt.map_or(Ok(None), |r| r.map(Some))?;

    Ok(opt)
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("Thr first doubled is {:?}", double_first(numbers));
    println!("Thr first doubled is {:?}", double_first(empty));
    println!("Thr first doubled is {:?}", double_first(strings));
}