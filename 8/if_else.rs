fn main() {
    let n = 5;
    
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, half the number");

            n / 2 // if n is 15, return 7
            // if comment out, expected integer, found (): error if and else have incompatible types
        };
    
    println!("{} -> {}", n, big_n);
}