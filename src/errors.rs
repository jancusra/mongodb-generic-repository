use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct MyError {}

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl MyError {
    pub fn show(message: &str) {
        println!("{:?}", message)
    }
}


macro_rules! err {
    () => {
        |e| MyError::show(&e.to_string())
    }
}

pub(crate) use err;