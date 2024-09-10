trait Modified {
    fn save(&self);
    fn discard(&self);
}

struct ModifiedImpl for Modified {
    fn save(&self) {
        // save somehow
    }

    fn discard(&self) {
        // discard changes
    }
}
