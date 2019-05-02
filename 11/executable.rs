extern crate rary;

fn main() {
    rary::public_function();

    // error: function `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}