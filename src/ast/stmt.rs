use crate::{
    ast::{ExprID, Ident, Mutability, Publicity, TypeID, types::GenericParam},
    source::Span,
};

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr(ExprID),
    VarDec(VarDecStmt),
    ConstDec(ConstDecStmt),
    TypeDec(TypeDecStmt),
    StructDec(StructDecStmt),
    UnionDec(UnionDecStmt),
    EnumDec(EnumDecStmt),
    // FuncDec(FuncDecStmt),
}

#[derive(Debug, Clone)]
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
        ty: Option<TypeID>,
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

    pub fn const_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        ty: Option<TypeID>,
        init: ExprID,
        span: Span,
    ) -> Self {
        Self::new(
            StmtKind::ConstDec(ConstDecStmt {
                ident,
                publicity,
                ty,
                init,
            }),
            span,
        )
    }

    pub fn type_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        generics: Option<Vec<GenericParam>>,
        target: TypeID,
        span: Span,
    ) -> Self {
        Self::new(
            StmtKind::TypeDec(TypeDecStmt {
                ident,
                publicity,
                generics,
                target,
            }),
            span,
        )
    }

    pub fn struct_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        generics: Option<Vec<GenericParam>>,
        fields: Vec<StructField>,
        span: Span,
    ) -> Self {
        Self::new(
            StmtKind::StructDec(StructDecStmt {
                ident,
                publicity,
                generics,
                fields,
            }),
            span,
        )
    }

    pub fn union_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        generics: Option<Vec<GenericParam>>,
        variants: Vec<UnionVariant>,
        span: Span,
    ) -> Self {
        Self::new(
            StmtKind::UnionDec(UnionDecStmt {
                ident,
                publicity,
                generics,
                variants,
            }),
            span,
        )
    }

    pub fn enum_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        variants: Vec<EnumVariant>,
        span: Span,
    ) -> Self {
        Self::new(
            StmtKind::EnumDec(EnumDecStmt {
                ident,
                publicity,
                variants,
            }),
            span,
        )
    }
}

#[derive(Debug, Clone)]
struct VarDecStmt {
    ident: Ident,
    mutable: Mutability,
    ty: Option<TypeID>,
    init: ExprID,
}

#[derive(Debug, Clone)]
struct ConstDecStmt {
    ident: Ident,
    publicity: Option<Publicity>,
    ty: Option<TypeID>,
    init: ExprID,
}

#[derive(Debug, Clone)]
struct TypeDecStmt {
    ident: Ident,
    publicity: Option<Publicity>,
    generics: Option<Vec<GenericParam>>,
    target: TypeID,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: Ident,
    pub ty: TypeID,
    pub publicity: Option<Publicity>,
}

#[derive(Debug, Clone)]
struct StructDecStmt {
    ident: Ident,
    publicity: Option<Publicity>,
    generics: Option<Vec<GenericParam>>,
    fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub enum UnionVariantKind {
    Unit,
    Tuple { ty: Vec<TypeID> },
    Struct { ty: Vec<(Ident, TypeID)> },
}

#[derive(Debug, Clone)]
pub struct UnionVariant {
    pub name: Ident,
    pub kind: UnionVariantKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
struct UnionDecStmt {
    ident: Ident,
    publicity: Option<Publicity>,
    generics: Option<Vec<GenericParam>>,
    variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub ident: Ident,
    pub value: Option<ExprID>,
}

#[derive(Debug, Clone)]
struct EnumDecStmt {
    ident: Ident,
    publicity: Option<Publicity>,
    variants: Vec<EnumVariant>,
}
