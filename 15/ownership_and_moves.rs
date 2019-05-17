fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // c is destroyed and the memory freed
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // Copy x into y - no resources are moved
    let y = x;

    println!("x is {}, and y is {}", x, y);

    // a is a pointer to a heap allocated integer
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // Move a into b
    let b = a;// now b owns the pointer to the heap
    //println!("a contains: {}", a);// error: value used here after move

    destroy_box(b);

    //println!("b contains: {}", b);// error: values used here after move
}