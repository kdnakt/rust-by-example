fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", n);
        }
    }
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz2");
        } else if n % 5 == 0 {
            println!("buzz2");
        } else if n % 3 == 0 {
            println!("fizz2");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:?}", names);

    let names2 = vec!["Bob", "Frank", "Ferris"];
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    //println!("{:?}", names2);// error: use of moved value
    // once the collection has been consumed it is no longer available for reuse

    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    for name in names3.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:?}", names3);
}