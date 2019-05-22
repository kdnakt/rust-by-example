struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`:  {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();

    //let owner2 = Owner(4);// error: cannot borrow immutable local variable as mutable
    //owner2.add_one();
    //owner2.print();
}