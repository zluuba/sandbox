use std::io;


pub trait GameUI {
    fn show_rules(&self) {
        println!("Hi there. Decode the anagram and send it back to me.");
    }

    fn show_anagram(&self, anagram: &String) {
        println!("Anagram: {anagram}");
    }

    fn get_user_answer(&self) -> String {
        println!("Please, enter the guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess.trim().to_string()
    }

    fn show_user_win(&self) {
        println!("That's right, you won!");
    }

    fn show_user_loss(&self, correct_word: &str) {
        println!("No :( The right answer is: {correct_word}.");
    }

    fn show_new_line(&self) {
        println!();
    }
}


pub struct SimpleUI;
impl GameUI for SimpleUI {}


pub struct PrettifyUI;

impl GameUI for PrettifyUI {
    fn show_rules(&self) {
        println!("Hello, let's play the Anagram Game!");
        println!("Decode the anagram and send it back to me.");
        println!("Leeet's go!");
    }
}
