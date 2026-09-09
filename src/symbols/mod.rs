pub mod resolve;

use std::collections::HashMap;

use crate::{ast::string::StringID, define_id, utils::arena::Arena};

define_id!(SymbolID);
define_id!(ScopeID);

pub struct Scope {
    pub parent: Option<ScopeID>,
    pub symbols: HashMap<StringID, SymbolID>,
}

impl Scope {
    pub fn new(parent: Option<ScopeID>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: StringID, symbol_id: SymbolID) -> Result<(), ()> {
        if self.symbols.contains_key(&name) {
            return Err(());
        }
        self.symbols.insert(name, symbol_id);
        Ok(())
    }

    pub fn lookup_current(&self, name: StringID) -> Option<SymbolID> {
        self.symbols.get(&name).cloned()
    }
}

pub struct SymbolInterner {
    arena: Arena<StringID, SymbolID>,
}

impl SymbolInterner {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }

    pub fn intern(&mut self, name: StringID) -> SymbolID {
        if let Some(&id) = self.arena.get(name) {
            return id;
        }

        let id = SymbolID::from(self.arena.len());

        self.arena.push(id);

        id
    }

    pub fn get(&self, id: SymbolID) -> SymbolID {
        self.arena[id.index()]
    }

    pub fn get_name(&self, id: SymbolID) -> StringID {
        StringID::from(self.get(id).0)
    }

    pub fn contains(&self, sym: SymbolID) -> bool {
        self.arena.contains(sym)
    }
}
