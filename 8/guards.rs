fn main() {
    //let pair = (2, -2);
    //let pair = (3, 3);
    //let pair = (5, 7);
    let pair = (2, 1);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ if condirion part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}