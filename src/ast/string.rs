use std::collections::HashMap;

use bumpalo::Bump;

use crate::define_id;

define_id!(StringID);

impl StringID {
    pub const EMPTY: StringID = StringID(0);
}

pub struct StringPool<'a> {
    arena: &'a Bump,
    indices: HashMap<&'a [u8], StringID>,
    strings: Vec<&'a [u8]>,
}

impl<'a> StringPool<'a> {
    pub fn new(arena: &'a Bump) -> Self {
        Self {
            arena,
            indices: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn intern(&mut self, text: &[u8]) -> StringID {
        if let Some(&id) = self.indices.get(text) {
            return id;
        }

        let id = StringID(self.strings.len());

        let string = self.arena.alloc_slice_copy(text);

        self.indices.insert(string, id);
        self.strings.push(string);

        id
    }

    pub fn get(&self, id: StringID) -> &[u8] {
        self.strings[id.0 as usize]
    }
}
