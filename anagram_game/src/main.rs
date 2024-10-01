mod ui;
mod scrambler;
mod word_library;
mod anagrams;
mod constants;
mod utils;
mod errors;

use ui::GameUI;
use scrambler::Scrambler;
use word_library::Library;


fn main() {
    let word_library = Box::new(word_library::SimpleWordLibrary);
    let scrambler = Box::new(scrambler::ShuffleScrambler);
    let ui = Box::new(ui::SimpleUI);

    let anagrams = anagrams::Anagrams::new(
        word_library, 
        scrambler, 
        ui,
    );

    anagrams.run();
}
