mod context;
mod deftable;
mod passes;
mod scope;

use crate::ast::{AST, string::StringPool};

pub struct Resolver<'pool> {
    defs: DefTable,
    tys: TyInterner,
    pool: &'pool StringPool<'pool>,
}

impl<'pool> Resolver<'pool> {
    pub fn new(pool: &'pool StringPool<'pool>) -> Self {
        Self {
            defs: DefTable::new(),
            tys: TyInterner::new(),
            pool,
        }
    }

    pub fn resolve(&mut self, ast: &AST, tree: &mut ModuleTree) {
        let mut ctx = ResolveContext::new(self.pool, &mut self.defs, &mut self.tys);

        let mut collect = passes::CollectPass::new(&mut ctx, &mut self.errors);
        collect.run(ast, tree);

        let mut imports = passes::ImportPass::new(&mut ctx, &mut self.errors);
        imports.run(ast, tree);

        let mut resolve = passes::ResolvePass::new(&mut ctx, &mut self.errors);
        resolve.run(ast, tree);
    }
}
