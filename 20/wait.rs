use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let result = child.wait().unwrap();
    println!("{:?}", result);// ExitStatus(ExitStatus(0))
    println!("reached end of main");
}