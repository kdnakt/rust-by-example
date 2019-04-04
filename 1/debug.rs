
#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct Deep(Printable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
  println!("{:?} months in a year", 12);

  println!("{1:?} {0:?} is the {actor:?} name",
    "Slater",
    "Christian",
    actor="actor's");

  println!("Now {:?} will print!", Printable(3));

  println!("Now {:?} will print!", Deep(Printable(7)));

  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };
  println!("{:?}", peter);
  println!("{:#?}", peter);
}