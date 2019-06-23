mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    //println!("{}", op(1.0, 10.0));// NegativeSquareRoot
    println!("{}", op(10.0, 10.0));// 0
    println!("{}", op(100.0, 10.0));// 1.5174271....
    println!("{}", op(1000.0, 10.0));// 2.145966....
    println!("{}", op(10000.0, 10.0));// 2.6282....
    println!("{}", op(100000.0, 10.0));//3.034854...
    //println!("{}", op(1.0, -5.0));// NonPositiveLogarithm
    //println!("{}", op(1.0, 0.0))// DivisionByZero
}