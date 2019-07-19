use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Mypath is {}", args[0]);

    // $ ./program_arguments arg1 arg2
    println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
}