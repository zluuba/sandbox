use crate::lookup::AbstractLookup;
use crate::content::InstanceContent;
use crate::modified::ModifiedImpl;


pub struct Component;
pub struct Graphics;


// extends Icon, Lookup.Provider ??
pub trait ExtIcon {
    fn new(&self) -> Box<&dyn ExtIcon>;
    fn paint_icon(&self, c: Option<Component>, g: Graphics, x: i32, y: i32);
    fn get_icon_width(&self) -> u32;
    fn get_icon_height(&self) -> u32;
    fn mark_modified(&self);
    fn get_lookup(&self) -> Box<AbstractLookup>;
}


struct ModifiableIcon {
    lookup: Box<&AbstractLookup>,
    ic: Box<&InstanceContent>,
}

impl ExtIcon for ModifiableIcon {
    fn new(&self) -> Self {
        let ic = InstanceContent;
        let lookup = AbstractLookup;

        ModifiableIcon { ic: Box::new(&ic), lookup: Box::new(&lookup) }
    }

    fn get_lookup(&self) -> &AbstractLookup {
        &*self.lookup
    }

    fn mark_modified(&self) {
        if self.lookup.lookup(&self) == None {
            // self.ic.add(ModifiedImpl);
            self.ic.add();
        }
    }
}
