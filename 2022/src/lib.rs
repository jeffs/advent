use std::error::Error;
use std::fmt;

pub type BoxedError = Box<dyn Error>;

#[derive(Debug)]
pub struct StaticError {
    what: &'static str,
}

impl StaticError {
    pub fn boxed(what: &'static str) -> BoxedError {
        Box::new(StaticError { what })
    }
}

impl fmt::Display for StaticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl Error for StaticError {}
