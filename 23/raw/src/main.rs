extern crate foo;

fn main() {
    // error: expected identifier, found reserved keyword `try`
    //foo::try();
    foo::r#try();
}
