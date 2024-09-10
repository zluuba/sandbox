use rand::seq::SliceRandom;
use rand::thread_rng;


pub trait Scrambler {
    fn scramble(&self, string: &String) -> String;
}


pub struct ShuffleScrambler;

impl Scrambler for ShuffleScrambler {
    fn scramble(&self, string: &String) -> String {
        // replases chars with shuffle
        let mut chars: Vec<char> = string.chars().collect();
        chars.shuffle(&mut thread_rng());

        chars.into_iter().collect()
    }
}


pub struct DummyScrambler;

impl Scrambler for DummyScrambler {
    fn scramble(&self, string: &String) -> String {
        // just reverses string 
        string.chars().rev().collect::<String>()
    }
}
