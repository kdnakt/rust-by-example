use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} +{}i", self.real, self.imag)
    }
}



fn main() {
    let minmax = MinMax(0,14);
    println!("Compare structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",
      big=big_range,
      small=small_range);
    
    let point = Point2D{x:3.3, y:7.2};
    println!("Compare points");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    //println!("in binary: {:b}", point); // error: the trait bound `Point2d: std::fmt::Binary` is not satisfied

    let cp = Complex { real: 3.3, imag: 7.2 };
    println!("Compare complex");
    println!("Display: {}", cp);
    println!("Debug: {:?}", cp);
}
