#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);
    //error: Secondsdoesnt implement Debug
    //println!("One second looks like: {:?}", _one_second);

    //error: binary operation == cannot be applied to type Seconds
    //let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);
    println!("One foot equals {:?}", foot.to_centimeters());

    let meter = Centimeters(100.0);

    let cmp = 
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter", cmp);
}