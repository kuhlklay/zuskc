use crate::{
    ast::{ExprID, Ident, Publicity, TypeID},
    source::Span,
};

#[derive(Debug, Clone)]
pub(super) enum TypeKind {
    Named {
        ident: Ident,
        args: Option<GenericArgs>,
    },

    Slice {
        inner: TypeID,
    },

    Array {
        inner: TypeID,
        size: ExprID,
    },

    Tuple {
        inner: Vec<TypeID>,
    },

    Ref {
        inner: TypeID,
    },

    Ptr {
        inner: TypeID,
    },

    Unit,
    Err,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

impl Type {
    pub fn new(kind: TypeKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn named(ident: Ident, args: Option<GenericArgs>, span: Span) -> Self {
        Self::new(TypeKind::Named { ident, args }, span)
    }

    pub fn slice(inner: TypeID, span: Span) -> Self {
        Self::new(TypeKind::Slice { inner }, span)
    }

    pub fn array(inner: TypeID, size: ExprID, span: Span) -> Self {
        Self::new(TypeKind::Array { inner, size }, span)
    }

    pub fn tuple(inner: Vec<TypeID>, span: Span) -> Self {
        Self::new(TypeKind::Tuple { inner }, span)
    }

    pub fn reference(inner: TypeID, span: Span) -> Self {
        Self::new(TypeKind::Ref { inner }, span)
    }

    pub fn pointer(inner: TypeID, span: Span) -> Self {
        Self::new(TypeKind::Ptr { inner }, span)
    }

    pub fn unit(span: Span) -> Self {
        Self::new(TypeKind::Unit, span)
    }
}

#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: Ident,
    pub bounds: Vec<TypeID>,
    pub default: Option<TypeID>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct GenericArgs {
    pub args: Vec<TypeID>,
    pub span: Span,
}
