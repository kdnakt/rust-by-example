fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    //println!("outer short: {}", short_lived_binding);//error: cannot find value in this scope

    println!("outer long: {}", long_lived_binding);

    let long_lived_binding ='a';// shadows the previous binding

    println!("outer long: {}", long_lived_binding);
}