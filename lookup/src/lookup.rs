use crate::content::InstanceContent;


pub struct Lookup;

pub struct AbstractLookup {
    ic: InstanceContent,
}

impl AbstractLookup {
    pub fn lookup(&self) -> Lookup {
        // some implementation

        Lookup
    }
}
