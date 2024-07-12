//! Error module: very basic code to catch and display program errors (to avoid panic errors)

use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Custom implementation of the error system trait
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

/// Custom macro to catch and display an error in the terminal 
/// (usually used with map_err method)
/// 
/// # Example
/// 
/// ```
/// use mongodb_repo::{err, errors::MyError};
///
/// let s: String = "42".to_string();
/// 
/// match s.parse::<i32>().map_err(err!()) {
///     Ok(number) => assert_eq!(42, number),
///     Err(_) => {}
/// }
/// ```
#[macro_export]
macro_rules! err {
    () => {
        |e| MyError::show(&e.to_string())
    }
}

pub(crate) use err;