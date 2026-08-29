pub mod expr;
pub mod stmt;
pub mod string;
pub mod types;

use crate::{
    ast::{expr::Expr, stmt::Stmt, string::StringID, types::Type},
    source::Span,
};

#[derive(Debug)]
pub struct AST {
    items: Vec<Item>,
    exprs: Vec<Expr>,
    stmts: Vec<Stmt>,
    types: Vec<Type>,
    patterns: Vec<Pattern>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            exprs: Vec::new(),
            stmts: Vec::new(),
            types: Vec::new(),
            patterns: Vec::new(),
        }
    }

    pub fn add_expr(&mut self, expr: Expr) -> ExprID {
        let id = ExprID(self.exprs.len() as u32);
        self.exprs.push(expr);
        id
    }

    pub fn expr(&self, id: ExprID) -> &Expr {
        &self.exprs[id.index()]
    }

    pub fn expr_mut(&mut self, id: ExprID) -> &mut Expr {
        &mut self.exprs[id.index()]
    }

    pub fn add_stmt(&mut self, stmt: Stmt) -> StmtID {
        let id = StmtID(self.stmts.len() as u32);
        self.stmts.push(stmt);
        id
    }

    pub fn stmt(&self, id: StmtID) -> &Stmt {
        &self.stmts[id.index()]
    }

    pub fn stmt_mut(&mut self, id: StmtID) -> &mut Stmt {
        &mut self.stmts[id.index()]
    }

    pub fn add_type(&mut self, ty: Type) -> TypeID {
        let id = TypeID(self.types.len() as u32);
        self.types.push(ty);
        id
    }

    pub fn ty(&self, id: TypeID) -> &Type {
        &self.types[id.index()]
    }

    pub fn ty_mut(&mut self, id: TypeID) -> &mut Type {
        &mut self.types[id.index()]
    }

    pub fn add_pattern(&mut self, pattern: Pattern) -> PatternID {
        let id = PatternID(self.patterns.len() as u32);
        self.patterns.push(pattern);
        id
    }

    pub fn pattern(&self, id: PatternID) -> &Pattern {
        &self.patterns[id.index()]
    }

    pub fn pattern_mut(&mut self, id: PatternID) -> &mut Pattern {
        &mut self.patterns[id.index()]
    }
}

macro_rules! define_id {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u32);

        impl $name {
            pub const fn new(index: u32) -> Self {
                Self(index)
            }

            pub const fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

define_id!(ItemID);
define_id!(ExprID);
define_id!(StmtID);
define_id!(TypeID);

define_id!(PatternID);

#[derive(Debug, PartialEq)]
pub(super) enum Item {
    Include,
    Import,
    Stmt(StmtID),
    EOF,
}

#[derive(Debug)]
pub(super) struct PatternField {
    pub ident: Ident,
    pub pattern: Option<Pattern>, // None = shorthand { x } instead of { x: x }
    pub span: Span,
}

#[derive(Debug)]
pub(super) enum Pattern {
    Wildcard {
        span: Span,
    },

    Binding {
        ident: Ident,
    },

    Literal {
        expr: ExprID,
    },

    Struct {
        // path: TypePath,
        fields: Box<[PatternField]>,
        span: Span,
    },

    Tuple {
        elems: Box<[PatternID]>,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Ident {
    id: StringID,
    span: Span,
}

impl Ident {
    pub fn new(id: StringID, span: Span) -> Self {
        Self { id, span }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PublicityKind {
    Public,
    Root,
    Module,
    Super,
    Self_,
}

#[derive(Debug, Clone, Copy)]
pub struct Publicity {
    pub kind: PublicityKind,
    pub span: Span,
}

impl Publicity {
    pub fn new(kind: PublicityKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone)]
pub enum Mutability {
    Immutable,
    Mutable { span: Span },
}
