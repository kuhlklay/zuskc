use bumpalo::Bump;

use crate::{
    ast::string::{StringID, StringMap},
    source::{FileID, SourceFile, SourceMap},
    utils::sync::{Lock, Lrc},
};

use parking_lot::RwLock;

pub struct Session {
    source_map: RwLock<SourceMap>,
    string_map: RwLock<StringMap<'static>>,
}

impl Session {
    pub fn new() -> Self {
        let bump: &'static Bump = Box::leak(Box::new(Bump::new()));
        Self {
            string_map: RwLock::new(StringMap::new(bump)),
            source_map: RwLock::new(SourceMap::new()),
        }
    }

    pub fn add_file(&self, path: std::path::PathBuf, content: String) -> FileID {
        self.source_map.write().add_file(path, content)
    }

    pub fn file(&self, id: FileID) -> Lrc<SourceFile> {
        self.source_map.read().file(id)
    }

    pub fn intern(&self, text: &str) -> StringID {
        self.string_map.write().intern(text)
    }

    pub fn lookup(&self, id: StringID) -> String {
        self.string_map.read().get(id).to_owned()
    }
}
