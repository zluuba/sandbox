use crate::constants::{SIMPLE_WORDS, COMPLICATED_WORDS};
use crate::errors::EmptyVecError;
use crate::utils::get_word_from_vec;


pub trait Library {
    fn get_word(&self) -> Result<String, EmptyVecError>;
}


pub struct SimpleWordLibrary;

impl Library for SimpleWordLibrary {
    fn get_word(&self) -> Result<String, EmptyVecError> {
        match get_word_from_vec(SIMPLE_WORDS.to_vec()) {
            Some(w) => Ok(w),
            None => Err(EmptyVecError),
        }
    }
}


pub struct ComplicatedWordLibrary;

impl Library for ComplicatedWordLibrary {
    fn get_word(&self) -> Result<String, EmptyVecError> {
        match get_word_from_vec(COMPLICATED_WORDS.to_vec()) {
            Some(w) => Ok(w),
            None => Err(EmptyVecError),
        }
    }
}
