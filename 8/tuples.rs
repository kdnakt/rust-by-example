fn main() {
    //let pair = (0, 2);// pattern 1
    //let pair = (3, 0);// pattern 2
    //let pair = (1, 4);// pattern 3
    let pair = (0, 0);// pattern 1
    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
    }
}