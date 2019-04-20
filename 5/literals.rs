fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;// i32 for integers
    let f = 1.0;// f64 for floating-point numbers

    // &foo used to pass by reference, not by value
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));// 1
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));// 4
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));// 4
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));// 4
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));// 8
}