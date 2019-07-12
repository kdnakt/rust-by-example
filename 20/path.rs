use std::path::Path;

fn main() {
    let path = Path::new(".");

    let display = path.display();
    println!("{}", display);// .

    let new_path = path.join("a").join("b");

    match new_path.to_str() {
        None => panic!("new path is not a valid utf-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}