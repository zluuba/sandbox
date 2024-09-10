use crate::Library;
use crate::Scrambler;
use crate::GameUI;

// #[derive(Debug)]
pub struct AnagramsConstructor {
    pub word_library: Box<dyn Library>,
    pub scrambler: Box<dyn Scrambler>,
    pub ui: Box<dyn GameUI>,
}

impl AnagramsConstructor {
    pub fn new(word_library: Box<dyn Library>, 
            scrambler: Box<dyn Scrambler>, 
            ui: Box<dyn GameUI>) -> Self {
        AnagramsConstructor { word_library, scrambler, ui }
    }

    // fn get_word_library(&self) -> &dyn Library {
    //     &*self.word_library
    // }

    // fn get_scrambler(&self) -> &dyn Scrambler {
    //     &*self.scrambler
    // }
}