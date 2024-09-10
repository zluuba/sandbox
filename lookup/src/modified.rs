// save/discard what ?
// need to change the signature of all funcs
trait Modified {
    fn save(&self);
    fn discard(&self);
}


// what the diff between Modified and ModifiedImpl ?
pub struct ModifiedImpl;

impl Modified for ModifiedImpl {
    fn save(&self) {
        // save somehow
    }

    fn discard(&self) {
        // discard changes
    }
}
