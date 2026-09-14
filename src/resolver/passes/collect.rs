use super::{
    DefKind, ResolveContext, ResolveError,
    structs::{
        ConstDef, EnumDef, FnSig, FunctionDef, StructDef, StructKind, TypeAliasDef, UnionDef,
    },
};
use crate::ast::{AST, ASTItem, ASTStmtKind};
use crate::module::ModuleTree;

pub struct CollectPass<'a, 'pool> {
    ctx: &'a mut ResolveContext<'pool>,
    errors: &'a mut Vec<ResolveError>,
}

impl<'a, 'pool> CollectPass<'a, 'pool> {
    pub fn new(ctx: &'a mut ResolveContext<'pool>, errors: &'a mut Vec<ResolveError>) -> Self {
        Self { ctx, errors }
    }

    pub fn run(&mut self, ast: &AST, tree: &mut ModuleTree) {
        for item in &ast.items {
            let ASTItem::Stmt(stmt) = item else { continue };
            let module_id = tree.module_of(stmt.span.file_id).unwrap_or(tree.root);

            match &stmt.kind {
                ASTStmtKind::StructDec(s) => self.collect_struct(s, module_id, tree),
                ASTStmtKind::EnumDec(e) => self.collect_enum(e, module_id, tree),
                ASTStmtKind::UnionDec(u) => self.collect_union(u, module_id, tree),
                ASTStmtKind::FuncDec(f) => self.collect_function(f, module_id, tree),
                ASTStmtKind::ConstDec(c) => self.collect_const(c, module_id, tree),
                ASTStmtKind::TypeAliasDec(a) => self.collect_type_alias(a, module_id, tree),
                _ => {}
            }
        }
    }

    fn collect_struct(&mut self, s: &ASTStructDec, module_id: ModuleId, tree: &mut ModuleTree) {
        let public = s.public == Publicity::Public;
        let id = self.ctx.defs.add_struct(StructDef {
            name: self.ctx.make_symbol(&s.ident),
            kind: StructKind::Unit,
            span: s.ident.span,
        });
        self.define(
            tree,
            module_id,
            s.ident.id.0,
            DefKind::Struct(id),
            public,
            s.ident.span,
        );
    }

    // ... ähnlich für Enum, Union, Function, Const, TypeAlias

    fn define(
        &mut self,
        tree: &mut ModuleTree,
        module_id: ModuleId,
        name_id: usize,
        kind: DefKind,
        public: bool,
        span: Span,
    ) {
        if !tree.get_mut(module_id).define(name_id, kind, public) {
            self.errors.push(ResolveError::DuplicateName { /* ... */ });
        }
    }
}
