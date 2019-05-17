fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);// 5

    //*immutable_box = 4;
    // error: cannot assign to immutable `Box` content `*immutable_box`
    // cannot borrow as mutable

    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);// 5

    *mutable_box = 4;

    println!("mutable_box contains {}", mutable_box);// 4
}