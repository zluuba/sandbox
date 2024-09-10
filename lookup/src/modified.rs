trait Modified {
    fn save(&self);
    fn discard(&self);
}


pub struct ModifiedImpl;

impl Modified for ModifiedImpl {
    fn save(&self) {
        // save somehow
    }

    fn discard(&self) {
        // discard changes
    }
}
