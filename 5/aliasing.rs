type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)] // commenting out -> warning: type u64_t should have a camel case name
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches);
}