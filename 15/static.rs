// static lifetime
static NUM: i32 = 18;

// returns a reference to NUM
// where its static lifetime is coerced to that of input argument
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // when goes out of scope,  the reference can no longer be used
        // but the data remains in the binary
    }
    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accesible!", NUM);
}
