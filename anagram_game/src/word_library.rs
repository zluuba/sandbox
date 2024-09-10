use rand::seq::SliceRandom;

use crate::constants::SIMPLE_WORDS;
use crate::constants::COMPLICATED_WORDS;


pub trait Library {
    fn get_word(&self) -> String;
}


pub struct SimpleWordLibrary;

impl Library for SimpleWordLibrary {
    fn get_word(&self) -> String {
        // easy level
        let word: Vec<_> = SIMPLE_WORDS
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();

        word.iter().map(|s| **s).collect()
    }
}


pub struct ComplicatedWordLibrary;

impl Library for ComplicatedWordLibrary {
    fn get_word(&self) -> String {
        // hard level
        let word: Vec<_> = COMPLICATED_WORDS
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();

        word.iter().map(|s| **s).collect()
    }
}
