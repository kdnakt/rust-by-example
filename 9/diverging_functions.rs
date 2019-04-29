//#![feature(never_type)]// error: feature may not be used on the stable release channel
//fn main() {
//    let x: ! = panic!("This call never returns");
//    println!("You will never see this line");// warning: unreachable statement
//}


fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    let mut target = 9;
    println!("Sum of odd numbers up to {} (excluding): {}",
        target, sum_odd_numbers(target));
    target += 1;
    println!("Sum of odd numbers up to {} (excluding): {}",
        target, sum_odd_numbers(target));
    target += 1;
    println!("Sum of odd numbers up to {} (excluding): {}",
        target, sum_odd_numbers(target));
    target += 1;
    println!("Sum of odd numbers up to {} (excluding): {}",
        target, sum_odd_numbers(target));
    target += 1;
    println!("Sum of odd numbers up to {} (excluding): {}",
        target, sum_odd_numbers(target));
}