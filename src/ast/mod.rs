pub mod expr;
pub mod items;
pub mod patterns;
pub mod stmt;
pub mod string;
pub mod types;

use crate::{
    ast::{expr::Expr, items::Item, patterns::Pat, stmt::Stmt, string::StringID, types::Ty},
    define_id,
    source::Span,
    utils::arena::Arena,
};

#[derive(Debug)]
pub struct AST {
    pub items: Arena<ItemID, Item>,
    exprs: Arena<ExprID, Expr>,
    stmts: Arena<StmtID, Stmt>,
    types: Arena<TyID, Ty>,
    patterns: Arena<PatID, Pat>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            items: Arena::new(),
            exprs: Arena::new(),
            stmts: Arena::new(),
            types: Arena::new(),
            patterns: Arena::new(),
        }
    }

    pub fn items(&mut self) -> &mut Arena<ItemID, Item> {
        &mut self.items
    }

    pub fn exprs(&mut self) -> &mut Arena<ExprID, Expr> {
        &mut self.exprs
    }

    pub fn stmts(&mut self) -> &mut Arena<StmtID, Stmt> {
        &mut self.stmts
    }

    pub fn types(&mut self) -> &mut Arena<TyID, Ty> {
        &mut self.types
    }

    pub fn patterns(&mut self) -> &mut Arena<PatID, Pat> {
        &mut self.patterns
    }
}

define_id!(ItemID);
define_id!(ExprID);
define_id!(StmtID);
define_id!(TyID);
define_id!(PatID);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ident {
    id: StringID,
    span: Span,
}

impl Ident {
    pub fn new(id: StringID, span: Span) -> Self {
        Self { id, span }
    }

    pub fn id(&self) -> StringID {
        self.id
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Publicity {
    Private,
    /// `pub`
    Public {
        span: Span,
    },
    /// `pub(root)`
    Root {
        span: Span,
    },
    /// `pub(crate)`
    Module {
        span: Span,
    },
    /// `pub(super)`
    Super {
        span: Span,
    },
    /// `pub(self)`
    Self_ {
        span: Span,
    },
    // /// `pub(in path)`
    // InPath {
    //     span: Span,
    //     path: ModulePath,
    // },
}

impl Publicity {
    pub fn span(&self) -> Option<Span> {
        match self {
            Publicity::Private => None,
            Publicity::Public { span }
            | Publicity::Root { span }
            | Publicity::Module { span }
            | Publicity::Super { span }
            | Publicity::Self_ { span }
            // | Publicity::InPath { span, .. }
            => Some(*span),
        }
    }

    pub fn is_public(&self) -> bool {
        matches!(
            self,
            Publicity::Public { .. }
                | Publicity::Root { .. }
                | Publicity::Module { .. }
                | Publicity::Super { .. }
                | Publicity::Self_ { .. } // | Publicity::InPath { .. }
        )
    }

    pub fn is_private(&self) -> bool {
        matches!(self, Publicity::Private)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mutability {
    Immutable,
    Mutable { span: Span },
}
