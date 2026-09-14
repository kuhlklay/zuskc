use super::{DefKind, DefTable};
use crate::define_id;
use std::collections::HashMap;

define_id!(ScopeID);

/// Ein Scope kennt nur seine Parent-ID und seine Symbole
pub struct Scope {
    pub parent: Option<ScopeID>,
    symbols: HashMap<StringId, (DefKind, bool)>, // (Definition, public)
}

impl Scope {
    pub fn new(parent: Option<ScopeID>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: StringId, kind: DefKind, public: bool) -> bool {
        if self.symbols.contains_key(&name) {
            return false;
        }
        self.symbols.insert(name, (kind, public));
        true
    }

    pub fn lookup(&self, name: StringId) -> Option<(DefKind, bool)> {
        self.symbols.get(&name).copied()
    }

    pub fn is_public(&self, name: StringId) -> bool {
        self.symbols.get(&name).map_or(false, |(_, pub_)| *pub_)
    }
}

pub struct ScopeArena {
    scopes: Vec<Scope>,
}

impl ScopeArena {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(None)],
        }
    }

    pub fn push(&mut self, parent: Option<ScopeID>) -> ScopeID {
        let id = ScopeID::from(self.scopes.len());
        self.scopes.push(Scope::new(parent));
        id
    }

    pub fn get(&self, id: ScopeID) -> &Scope {
        &self.scopes[id.index()]
    }

    pub fn get_mut(&mut self, id: ScopeID) -> &mut Scope {
        &mut self.scopes[id.index()]
    }

    pub fn current(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn lookup_visible(&self, start: ScopeID, name: StringId) -> Option<(DefKind, bool)> {
        let mut current = start;
        loop {
            let scope = self.get(current);
            if let Some(found) = scope.lookup(name) {
                return Some(found);
            }
            if let Some(parent) = scope.parent {
                current = parent;
            } else {
                return None;
            }
        }
    }
}
