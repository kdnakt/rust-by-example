#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.3421_f32;

    // no implicit conversion
    //let integer: u8 = decimal;//error mismatched types, expected u8, found f32

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);

    println!("1000 as a u8 is : {}", 1000 as u8);// 1000 - 256 - 256 - 256 = 232
    println!("  -1 as a u8 is : {}", (-1i8) as u8);// -1 + 256 = 255

    println!("1000 mod 256 is : {}", 1000 % 256);// 232

    println!(" 128 as a i16 is: {}", 128 as i16);// 128

    println!(" 128 as a i8 is : {}", 128 as i8);// 128 - 256 = -128

    println!("1000 as a u8 is : {}", 1000 as u8);

    println!(" 232 as a i8 is : {}", 232 as i8);// 232 - 256 = -24
}