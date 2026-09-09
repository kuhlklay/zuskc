use crate::{
    ast::{ExprID, Ident, TyID},
    source::Span,
};

#[derive(Debug, PartialEq)]
pub enum TyKind {
    Named {
        ident: Ident,
        args: Option<GenericArgs>,
    },

    Slice {
        inner: TyID,
    },

    Array {
        inner: TyID,
        size: ExprID,
    },

    Tuple {
        inner: Vec<TyID>,
    },

    Ref {
        inner: TyID,
    },

    MutRef {
        inner: TyID,
    },

    Ptr {
        inner: TyID,
    },

    Self_,
    Unit,
}

#[derive(Debug, PartialEq)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(kind: TyKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn named(ident: Ident, args: Option<GenericArgs>, span: Span) -> Self {
        Self::new(TyKind::Named { ident, args }, span)
    }

    pub fn slice(inner: TyID, span: Span) -> Self {
        Self::new(TyKind::Slice { inner }, span)
    }

    pub fn array(inner: TyID, size: ExprID, span: Span) -> Self {
        Self::new(TyKind::Array { inner, size }, span)
    }

    pub fn tuple(inner: Vec<TyID>, span: Span) -> Self {
        Self::new(TyKind::Tuple { inner }, span)
    }

    pub fn reference(inner: TyID, span: Span) -> Self {
        Self::new(TyKind::Ref { inner }, span)
    }

    pub fn mutable_reference(inner: TyID, span: Span) -> Self {
        Self::new(TyKind::MutRef { inner }, span)
    }

    pub fn pointer(inner: TyID, span: Span) -> Self {
        Self::new(TyKind::Ptr { inner }, span)
    }

    pub fn self_(span: Span) -> Self {
        Self::new(TyKind::Self_, span)
    }

    pub fn unit(span: Span) -> Self {
        Self::new(TyKind::Unit, span)
    }
}

#[derive(Debug, PartialEq)]
pub struct GenericParam {
    pub name: Ident,
    pub bounds: Vec<TyID>,
    pub default: Option<TyID>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct GenericArgs {
    pub args: Vec<TyID>,
    pub span: Span,
}
