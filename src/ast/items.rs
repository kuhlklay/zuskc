use crate::{
    ast::{ExprID, Ident, ItemID, PatID, Publicity, TyID, types::GenericParam},
    source::Span,
};

#[derive(Debug, PartialEq)]
pub enum ItemKind {
    Const(ConstDecItem),
    Type(TyDecItem),
    Struct(StructDecItem),
    Union(UnionDecItem),
    Enum(EnumDecItem),
    Fn(FnDecItem),
    Impl(ImplDecItem),
    Trait(TraitDecItem),
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

impl Item {
    pub fn new(kind: ItemKind, span: Span) -> Self {
        Item { kind, span }
    }

    pub fn const_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        ty: Option<TyID>,
        init: ExprID,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Const(ConstDecItem {
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
        target: TyID,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Type(TyDecItem {
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
            ItemKind::Struct(StructDecItem {
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
            ItemKind::Union(UnionDecItem {
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
            ItemKind::Enum(EnumDecItem {
                ident,
                publicity,
                variants,
            }),
            span,
        )
    }

    pub fn fn_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        constant: Option<Span>,
        generics: Option<Vec<GenericParam>>,
        params: Option<Vec<FnParam>>,
        ty: Option<TyID>,
        body: Option<ExprID>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Fn(FnDecItem {
                ident,
                publicity,
                constant,
                generics,
                params,
                ty,
                body,
            }),
            span,
        )
    }

    pub fn impl_dec(
        target: TyID,
        generics: Option<Vec<GenericParam>>,
        trait_ty: Option<TyID>,
        items: Vec<ItemID>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Impl(ImplDecItem {
                target,
                generics,
                trait_ty,
                items,
            }),
            span,
        )
    }

    pub fn compound_trait_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        items: Vec<Ident>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Trait(TraitDecItem {
                ident,
                publicity,
                kind: TraitKind::Compound { items },
            }),
            span,
        )
    }

    pub fn trait_dec(
        ident: Ident,
        publicity: Option<Publicity>,
        items: Vec<ItemID>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Trait(TraitDecItem {
                ident,
                publicity,
                kind: TraitKind::Itemed { items },
            }),
            span,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDecItem {
    ident: Ident,
    publicity: Option<Publicity>,
    ty: Option<TyID>,
    init: ExprID,
}

#[derive(Debug, PartialEq)]
pub struct TyDecItem {
    ident: Ident,
    publicity: Option<Publicity>,
    generics: Option<Vec<GenericParam>>,
    target: TyID,
}

#[derive(Debug, PartialEq)]
pub struct StructField {
    pub ident: Ident,
    pub ty: TyID,
    pub publicity: Option<Publicity>,
}

#[derive(Debug, PartialEq)]
pub struct StructDecItem {
    ident: Ident,
    publicity: Option<Publicity>,
    generics: Option<Vec<GenericParam>>,
    fields: Vec<StructField>,
}

#[derive(Debug, PartialEq)]
pub enum UnionVariantKind {
    Unit,
    Tuple { ty: Vec<TyID> },
    Struct { ty: Vec<(Ident, TyID)> },
}

#[derive(Debug, PartialEq)]
pub struct UnionVariant {
    pub name: Ident,
    pub kind: UnionVariantKind,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct UnionDecItem {
    ident: Ident,
    publicity: Option<Publicity>,
    generics: Option<Vec<GenericParam>>,
    variants: Vec<UnionVariant>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub ident: Ident,
    pub value: Option<ExprID>,
}

#[derive(Debug, PartialEq)]
pub struct EnumDecItem {
    ident: Ident,
    publicity: Option<Publicity>,
    variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq)]
pub struct FnParam {
    pat: PatID,
    ty: TyID,
}

impl FnParam {
    pub fn new(pat: PatID, ty: TyID) -> Self {
        Self { pat, ty }
    }
}

#[derive(Debug, PartialEq)]
pub struct FnDecItem {
    ident: Ident,
    publicity: Option<Publicity>,
    constant: Option<Span>,
    generics: Option<Vec<GenericParam>>,
    params: Option<Vec<FnParam>>,
    ty: Option<TyID>,
    body: Option<ExprID>,
}

#[derive(Debug, PartialEq)]
pub struct ImplDecItem {
    target: TyID,
    generics: Option<Vec<GenericParam>>,
    trait_ty: Option<TyID>,
    items: Vec<ItemID>,
}

#[derive(Debug, PartialEq)]
pub enum TraitKind {
    Compound { items: Vec<Ident> },
    Itemed { items: Vec<ItemID> },
}

#[derive(Debug, PartialEq)]
pub struct TraitDecItem {
    ident: Ident,
    publicity: Option<Publicity>,
    kind: TraitKind,
}
