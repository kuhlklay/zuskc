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

    Mod(ModuleDecItem),
    // Use(UseDecItem),
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub publicity: Option<Publicity>,
    pub span: Span,
}

impl Item {
    pub fn new(kind: ItemKind, publicity: Option<Publicity>, span: Span) -> Self {
        Item {
            kind,
            publicity,
            span,
        }
    }

    pub fn const_dec(
        ident: Ident,
        publicity: Publicity,
        ty: Option<TyID>,
        init: ExprID,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Const(ConstDecItem { ident, ty, init }),
            Some(publicity),
            span,
        )
    }

    pub fn type_dec(
        ident: Ident,
        publicity: Publicity,
        generics: Option<Vec<GenericParam>>,
        target: TyID,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Type(TyDecItem {
                ident,
                generics,
                target,
            }),
            Some(publicity),
            span,
        )
    }

    pub fn struct_dec(
        ident: Ident,
        publicity: Publicity,
        generics: Option<Vec<GenericParam>>,
        fields: Vec<StructField>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Struct(StructDecItem {
                ident,
                generics,
                fields,
            }),
            Some(publicity),
            span,
        )
    }

    pub fn union_dec(
        ident: Ident,
        publicity: Publicity,
        generics: Option<Vec<GenericParam>>,
        variants: Vec<UnionVariant>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Union(UnionDecItem {
                ident,
                generics,
                variants,
            }),
            Some(publicity),
            span,
        )
    }

    pub fn enum_dec(
        ident: Ident,
        publicity: Publicity,
        variants: Vec<EnumVariant>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Enum(EnumDecItem { ident, variants }),
            Some(publicity),
            span,
        )
    }

    pub fn fn_dec(
        ident: Ident,
        publicity: Publicity,
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
                constant,
                generics,
                params,
                ty,
                body,
            }),
            Some(publicity),
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
            None,
            span,
        )
    }

    pub fn compound_trait_dec(
        ident: Ident,
        publicity: Publicity,
        items: Vec<Ident>,
        span: Span,
    ) -> Self {
        Self::new(
            ItemKind::Trait(TraitDecItem {
                ident,
                kind: TraitKind::Compound { items },
            }),
            Some(publicity),
            span,
        )
    }

    pub fn trait_dec(ident: Ident, publicity: Publicity, items: Vec<ItemID>, span: Span) -> Self {
        Self::new(
            ItemKind::Trait(TraitDecItem {
                ident,
                kind: TraitKind::Itemed { items },
            }),
            Some(publicity),
            span,
        )
    }

    pub fn mod_dec(ident: Ident, publicity: Publicity, inline: Inline, span: Span) -> Self {
        Self::new(
            ItemKind::Mod(ModuleDecItem { ident, inline }),
            Some(publicity),
            span,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDecItem {
    ident: Ident,
    ty: Option<TyID>,
    init: ExprID,
}

#[derive(Debug, PartialEq)]
pub struct TyDecItem {
    ident: Ident,
    generics: Option<Vec<GenericParam>>,
    target: TyID,
}

#[derive(Debug, PartialEq)]
pub struct StructField {
    pub ident: Ident,
    pub publicity: Publicity,
    pub ty: TyID,
}

#[derive(Debug, PartialEq)]
pub struct StructDecItem {
    ident: Ident,
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
    kind: TraitKind,
}

#[derive(Debug, PartialEq)]
pub enum Inline {
    Inline(Vec<ItemID>),
    Extern,
}

#[derive(Debug, PartialEq)]
pub struct ModuleDecItem {
    pub ident: Ident,
    inline: Inline,
}

impl ModuleDecItem {
    pub fn is_inline(&self) -> bool {
        matches!(self.inline, Inline::Inline(_))
    }
}
