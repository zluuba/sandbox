use crate::content::InstanceContent;


// should be the base trait ?
pub struct Lookup;


// what the diff between Lookup and AbstractLookup ?
pub struct AbstractLookup {
    ic: InstanceContent,
}

impl AbstractLookup {
    pub fn lookup(&self) -> Lookup {
        // some implementation, but which ?

        Lookup
    }
}
