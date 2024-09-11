pub mod ui;
pub mod scrambler;
pub mod word_library;
pub mod constructor;
pub mod constants;

use ui::GameUI;
use scrambler::Scrambler;
use word_library::Library;


/// you can easily change word_library/scrambler/ui classes 
/// without any loss in functionality.
/// for example:
///     changing this line:  let ui = Box::new(ui::SimpleUI);
///     to this line:        let ui = Box::new(ui::PrettifyUI);
/// switches the app's UI, but the interface stays the same 
/// ('cause of `GameUI` trait)


fn main() {
    let word_library = Box::new(word_library::SimpleWordLibrary);
    let scrambler = Box::new(scrambler::ShuffleScrambler);
    let ui = Box::new(ui::SimpleUI);

    let anagrams = constructor::AnagramsConstructor {
        word_library, 
        scrambler, 
        ui,
    };

    anagrams.ui.show_rules();

    loop {
        let word = anagrams.word_library.get_word();
        let anagram = anagrams.scrambler.scramble(&word);

        anagrams.ui.show_anagram(&anagram);

        let user_answer = anagrams.ui.get_user_answer();

        match user_answer == word {
            true => anagrams.ui.show_user_win(),
            false => anagrams.ui.show_user_loss(&word),
        }

        println!();
    }
}
