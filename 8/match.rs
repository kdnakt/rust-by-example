fn main() {
    //let number = 13;// -> A teen
    //let number = 20;// -> Ain't special
    //let number = 1;// -> One!
    //let number = 5;// -> This is a prime
    let number = 4;// -> Ain't special
    println!("Tell me about {}", number);

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,// commenting out -> error: non-exhaustive patterns: true not covered
    };

    println!("{} -> {}", boolean, binary);
}