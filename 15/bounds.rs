use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// Ref contains a reference to a generic type T
// that has an unknown lifetime 'a.
// T is bounded such that any reference in T
// must outlibe 'a.
// Additionally the lifetime of Ref may not exceed 'a.

fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// a reference to T is taken
// where T implements Debug and all references in T outlibe 'a.
// In addition 'a must outlive the function
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
}

