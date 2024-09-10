pub mod ui;
pub mod scrambler;
pub mod word_library;
pub mod constructor;
pub mod constants;

use ui::GameUI;
use scrambler::Scrambler;
use word_library::Library;


fn main() {
    let word_library = Box::new(word_library::SimpleWordLibrary);
    let scrambler = Box::new(scrambler::ShuffleScrambler);
    let ui = Box::new(ui::SimpleUI);

    let anagrams = constructor::AnagramsConstructor::new(
        word_library, 
        scrambler, 
        ui,
    );

    anagrams.ui.show_rules();

    loop {
        let word = anagrams.word_library.get_word();
        let anagram = anagrams.scrambler.scramble(&word);

        anagrams.ui.show_anagram(&anagram);

        let user_answer = anagrams.ui.get_user_answer();
        let is_user_win = user_answer == word;

        anagrams.ui.show_results(is_user_win, word);

        println!();
    }
}