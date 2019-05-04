include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("Hello, world! from main.rs");
    println!("{}", message());
}
