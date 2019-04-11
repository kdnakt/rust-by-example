use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first elem of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));// 20 bytes

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    //println!("{}", xs[5])// error: index out of bounds
}