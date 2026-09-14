use std::collections::HashMap;

use bumpalo::Bump;

use crate::define_id;

define_id!(StringID);

impl StringID {
    pub const EMPTY: StringID = StringID(0);
}

pub struct StringMap<'a> {
    arena: &'a Bump,
    indices: HashMap<&'a str, StringID>,
    strings: Vec<&'a str>,
}

impl<'a> StringMap<'a> {
    pub fn new(arena: &'a Bump) -> Self {
        Self {
            arena,
            indices: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn intern(&mut self, text: &str) -> StringID {
        if let Some(&id) = self.indices.get(text) {
            return id;
        }

        let id = StringID(self.strings.len());

        let string = self.arena.alloc_str(text);

        self.indices.insert(string, id);
        self.strings.push(string);

        id
    }

    pub fn get(&self, id: StringID) -> &'a str {
        self.strings[id.0 as usize]
    }
}
