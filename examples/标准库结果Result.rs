fn main() {
    checked::op(1.0, 10.0);
    // println!("{}", op(1.0, 10.0));
}

mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
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
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
    fn op_(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;
        sqrt(ln)
    }
    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(
                "{}",
                match why {
                    MathError::NegativeLogarithm => "logarithm of negative number",
                    MathError::DivisionByZero => "division by zero",
                    MathError::NegativeSquareRoot => "square root of nagative number",
                }
            ),
            Ok(value) => println!("{}", value),
        }
    }
}
fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(e) => panic!("{:?}", e),
                Ok(sqrt) => sqrt,
            },
        },
    }
}
