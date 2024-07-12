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
/// match ObjectId::from_str(id).map_err(err!()) {
///     Ok(object_id) => Some(object_id),
///     Err(_) => None
/// }
/// ```
macro_rules! err {
    () => {
        |e| MyError::show(&e.to_string())
    }
}

pub(crate) use err;