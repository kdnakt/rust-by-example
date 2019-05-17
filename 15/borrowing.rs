fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This is is: {}", borrowed_i32)
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // cannot destroy while inner value is borrowed.
        //eat_box_i32(boxed_i32);// value moved here

        // _ref_to_i32 goes out of scope and no longer borrowed
    }

    eat_box_i32(boxed_i32);// error: use of moved value
}