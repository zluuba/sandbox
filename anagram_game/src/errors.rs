use std::fmt;


#[derive(Debug, Clone)]
pub struct EmptyVecError;

impl fmt::Display for EmptyVecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The word sequence is empty, cannot retrieve a word.")
    }
}
