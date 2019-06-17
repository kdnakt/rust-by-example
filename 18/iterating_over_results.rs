fn main() {
    let strings = vec!["tofu", "93", "18"];
    let possible_numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    // [Err(ParseIntError { kind: InvalidDigit }), Ok(93), Ok(18)]
    println!("Results: {:?}", possible_numbers);
 
    let strings2 = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings2
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    // [93, 18]
    println!("Results2: {:?}", numbers);

    let strings3 = vec!["tofu", "93", "18"];
    let numbers3: Result<Vec<_>, _> = strings3
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    // Err(ParseIntError { kind: InvalidDigit })
    println!("Results3: {:?}", numbers3);

    let strings4 = vec!["tofu", "93", "18"];
    let (numbers4, errors4): (Vec<_>, Vec<_>) = strings4
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    // [Ok(93), Ok(18)]
    println!("Numbers4: {:?}", numbers4);
    // [Err(ParseIntError { kind: InvalidDigit})]
    println!("Errors4: {:?}", errors4);

    let strings5 = vec!["tofu", "93", "18"];
    let (numbers5, errors5): (Vec<_>, Vec<_>) = strings5
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers5: Vec<_> = numbers5.into_iter().map(Result::unwrap).collect();
    let errors5: Vec<_> = errors5.into_iter().map(Result::unwrap_err).collect();
    // [93, 18]
    println!("Numbers5: {:?}", numbers5);
    // [ParseIntError { kind: InvalidDigit}]
    println!("Errors5: {:?}", errors5);
}