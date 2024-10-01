use crate::Library;
use crate::Scrambler;
use crate::GameUI;


pub struct Anagrams {
    pub word_library: Box<dyn Library>,
    pub scrambler: Box<dyn Scrambler>,
    pub ui: Box<dyn GameUI>,
}

impl Anagrams {
    pub fn new(
        word_library: Box<dyn Library>, 
        scrambler: Box<dyn Scrambler>, 
        ui: Box<dyn GameUI>
    ) -> Self {
        Self { word_library, scrambler, ui }
    }

    pub fn run(&self) {
        self.ui.show_rules();

        loop {
            let word = match self.word_library.get_word() {
                Ok(w) => w,
                Err(e) => {
                    println!("An error occurred while getting new word: {e}");
                    return
                },
            };
            let anagram = self.scrambler.scramble(&word);

            self.ui.show_anagram(&anagram);

            let user_answer = match self.ui.get_user_answer() {
                Ok(ans) => ans,
                Err(e) => {
                    println!("An error occurred while reading the line: {e}");
                    return
                },
            };

            match user_answer == word {
                true => self.ui.show_user_win(),
                false => self.ui.show_user_loss(&word),
            }

            self.ui.show_new_line();
        }
    }
}
