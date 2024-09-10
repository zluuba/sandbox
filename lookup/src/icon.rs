// extends Icon, Lookup.Provider ??
trait ExtIcon {
    fn paint_icon(&self, c: Component, g: Graphics, x: i32, y: i32);
    fn get_icon_width(&self) -> u32;
    fn get_icon_height(&self) -> u32;

    fn get_lookup(&self) -> Lookup;
}


struct ModifiableIcon for ExtIcon {
    lookup: Box<dyn AbstractLookup>,
    ic: Box<dyn InstanceContent>,
}

impl ModifiableIcon {
    pub fn new() -> Self {
        let ic = InstanceContent();
        let lookup = AbstractLookup(ic);

        ModifiableIcon { ic, lookup }
    }

    pub get_lookup(&self) {
        &*self.lookup
    }

    pub mark_modified(&self) {
        if self.lookup(&self) == None {
            self.ic.add(ModifiedImpl());
        }
    }
}