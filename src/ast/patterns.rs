use crate::{
    ast::{ExprID, Ident, Mutability, PatID, TyID, string::StringID},
    source::Span,
};

#[derive(Debug, PartialEq)]
pub struct PatField {
    pub name: Ident,
    pub pattern: PatID,
}

#[derive(Debug, PartialEq)]
pub enum PatKind {
    Wildcard,

    Binding {
        name: Ident,
        mutable: Mutability,
        ty: Option<TyID>,
    },

    Expr(ExprID),

    Tuple(Vec<PatID>),

    Struct {
        name: Ident,
        fields: Vec<PatField>,
    },

    Enum {
        enum_name: Ident,
        variant: Ident,
        payload: Option<PatID>,
        fields: Vec<StringID>,
    },

    Or(Vec<PatID>),

    Range {
        start: PatID,
        end: PatID,
        inclusive: bool,
    },

    Guard {
        pattern: PatID,
        condition: ExprID,
    },

    Ref {
        pattern: PatID,
        mutability: Mutability,
    },
}

#[derive(Debug, PartialEq)]
pub struct Pat {
    pub kind: PatKind,
    pub span: Span,
}

impl Pat {
    pub fn new(kind: PatKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn wildcard(span: Span) -> Self {
        Self::new(PatKind::Wildcard, span)
    }

    pub fn binding(name: Ident, mutable: Mutability, ty: Option<TyID>, span: Span) -> Self {
        Self::new(PatKind::Binding { name, mutable, ty }, span)
    }

    pub fn expr(expr: ExprID, span: Span) -> Self {
        Self::new(PatKind::Expr(expr), span)
    }
}
