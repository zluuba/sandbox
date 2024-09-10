use crate::Library;
use crate::Scrambler;
use crate::GameUI;

// #[derive(Debug)]
pub struct AnagramsConstructor {
    pub word_library: Box<dyn Library>,
    pub scrambler: Box<dyn Scrambler>,
    pub ui: Box<dyn GameUI>,
}

// impl AnagramsConstructor {
//     fn get_word_library(&self) -> &dyn Library {
//         &*self.word_library
//     }

//     fn set_word_library(&mut self, new_word_library: Box<dyn Library>) {
//         self.word_library = new_word_library;
//     }

//     fn get_scrambler(&self) -> &dyn Scrambler {
//         &*self.scrambler
//     }

//     fn set_scrambler(&mut self, new_scrambler: Box<dyn Scrambler>) {
//         self.scrambler = new_scrambler;
//     }

//     fn get_ui(&self) -> &dyn GameUI {
//         &*self.ui
//     }

//     fn set_ui(&mut self, new_ui: Box<dyn GameUI>) {
//         self.ui = new_ui;
//     }
// }
