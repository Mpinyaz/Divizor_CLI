use color_eyre::eyre::Result;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    DivisionByZero,
    // Custom(String),
}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            MyError::DivisionByZero => {
                write!(f, "Division by zero is not allowed")
            } // MyError::Custom(ref s) => write!(f, "my error: {}", s),
        }
    }
}

pub fn division(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    dividend.checked_div(divisor).ok_or(MyError::DivisionByZero)
}
