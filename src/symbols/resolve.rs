use crate::{
    ast::string::StringID,
    symbols::{Scope, ScopeID, SymbolID},
};

pub struct Resolver {
    scopes: Vec<Scope>,
    next: usize,
}

impl Resolver {
    pub fn new() -> Self {
        let global = Scope::new(None);

        Self {
            scopes: vec![global],
            next: 0,
        }
    }

    pub fn enter_scope(&mut self) -> ScopeID {
        let parent_id = ScopeID::from(self.scopes.len() - 1);
        let new_scope = Scope::new(Some(parent_id));
        self.scopes.push(new_scope);
        ScopeID::from(self.scopes.len() - 1)
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn current_scope_id(&self) -> ScopeID {
        ScopeID::from(self.scopes.len() - 1)
    }

    pub fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn current_scope_ref(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    pub fn get_scope(&self, id: ScopeID) -> &Scope {
        &self.scopes[id.index()]
    }

    pub fn get_scope_mut(&mut self, id: ScopeID) -> &mut Scope {
        &mut self.scopes[id.index()]
    }

    pub fn declare(&mut self, name: StringID) -> Result<SymbolID, ()> {
        let symbol_id = SymbolID::from(self.next);
        self.next += 1;

        let scope = self.current_scope();
        if scope.symbols.contains_key(&name) {
            return Err(());
        }

        scope.symbols.insert(name, symbol_id);
        Ok(symbol_id)
    }

    pub fn resolve(&self, name: StringID) -> Result<SymbolID, ()> {
        let mut scope_id = self.current_scope_id();

        loop {
            let scope = self.get_scope(scope_id);

            if let Some(&id) = scope.symbols.get(&name) {
                return Ok(id);
            }

            if let Some(parent_id) = scope.parent {
                scope_id = parent_id;
            } else {
                return Err(());
            }
        }
    }

    pub fn resolve_from(&self, name: StringID, start_scope: ScopeID) -> Result<SymbolID, ()> {
        let mut scope_id = start_scope;

        loop {
            let scope = self.get_scope(scope_id);

            if let Some(&id) = scope.symbols.get(&name) {
                return Ok(id);
            }

            if let Some(parent_id) = scope.parent {
                scope_id = parent_id;
            } else {
                return Err(());
            }
        }
    }

    pub fn scope_depth(&self) -> usize {
        self.scopes.len()
    }

    pub fn is_global_scope(&self) -> bool {
        self.scopes.len() == 1
    }

    pub fn print(&self) {
        for (i, scope) in self.scopes.iter().enumerate() {
            let symbols: Vec<String> = scope
                .symbols
                .keys()
                .map(|id| format!("{}", id.index()))
                .collect();
            println!(
                "Scope {}: parent={:?}, symbols={:?}",
                i,
                scope.parent.map(|id| id.index()),
                symbols
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver() {
        let mut resolver = Resolver::new();

        let global_var = StringID::from(0);
        let local_var = StringID::from(1);
        let inner_var = StringID::from(2);
        let unknown = StringID::from(99);

        resolver.declare(global_var).unwrap();

        resolver.enter_scope();
        resolver.declare(local_var).unwrap();

        resolver.enter_scope();
        resolver.declare(inner_var).unwrap();

        assert_eq!(resolver.resolve(global_var).unwrap().index(), 0);
        assert_eq!(resolver.resolve(local_var).unwrap().index(), 1);
        assert_eq!(resolver.resolve(inner_var).unwrap().index(), 2);
        assert!(resolver.resolve(unknown).is_err());

        resolver.exit_scope();
        assert!(resolver.resolve(inner_var).is_err());
        assert_eq!(resolver.resolve(local_var).unwrap().index(), 1);
        assert_eq!(resolver.resolve(global_var).unwrap().index(), 0);
    }
}
