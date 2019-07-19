use std::env;

fn inc(n: i32) {
    println!("{}", n + 1);
}

fn dec(n: i32) {
    println!("{}", n - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no argument passed
        1 => {
            println!("My name is `match_args`. Try passing some arguments!");
        },
        // one argument passed
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            match &cmd[..] {
                "increase" => inc(number),
                "decrease" => dec(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        _ => {
            help();
        }
    }
}