use super::structs::{DefTable, Symbol};
use super::{TyID, TyInterner};
use crate::ast::Ident;
use crate::ast::string::{StringID, StringPool};

pub struct ResolveContext<'pool> {
    pub pool: &'pool mut StringPool<'pool>,
    pub defs: &'pool mut DefTable,
    pub types: &'pool mut TyInterner,
}

impl<'pool> ResolveContext<'pool> {
    pub fn new(
        pool: &'pool mut StringPool,
        defs: &'pool mut DefTable,
        types: &'pool mut TyInterner,
    ) -> Self {
        Self { pool, defs, types }
    }

    pub fn intern_string(&mut self, s: &str) -> StringID {
        self.pool.intern(s.as_bytes())
    }

    pub fn string(&self, id: StringID) -> &[u8] {
        self.pool.get(id)
    }

    pub fn make_symbol(&self, ident: &Ident) -> Symbol {
        Symbol {
            name: ident.id(),
            span: ident.span(),
        }
    }

    pub fn make_type(&self, id: StructID) -> TyID {
        self.types.intern(TyKind::Struct(id))
    }

    // ... weitere Hilfsmethoden
}
