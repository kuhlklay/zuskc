use crate::{
    ast::{ExprID, Ident, Mutability, TyID},
    source::Span,
};

#[derive(Debug, PartialEq)]
pub enum StmtKind {
    Expr(ExprID),
    VarDec(VarDecStmt),
}

#[derive(Debug, PartialEq)]
pub struct Stmt {
    kind: StmtKind,
    span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Stmt { kind, span }
    }

    pub fn expr(expr: ExprID, span: Span) -> Self {
        Self::new(StmtKind::Expr(expr), span)
    }

    pub fn var_dec(
        ident: Ident,
        mutable: Mutability,
        ty: Option<TyID>,
        init: ExprID,
        span: Span,
    ) -> Self {
        Self::new(
            StmtKind::VarDec(VarDecStmt {
                ident,
                mutable,
                ty,
                init,
            }),
            span,
        )
    }
}

#[derive(Debug, PartialEq)]
struct VarDecStmt {
    ident: Ident,
    mutable: Mutability,
    ty: Option<TyID>,
    init: ExprID,
}
