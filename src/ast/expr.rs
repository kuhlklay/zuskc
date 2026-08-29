use std::fmt::{self, Display, Formatter};

use crate::{
    ast::{ExprID, Ident, StmtID, TypeID},
    source::Span,
};

#[derive(Debug)]
pub struct ExprRef {
    pub id: ExprID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn bool(value: bool, span: Span) -> Self {
        Self::new(ExprKind::Bool(value), span)
    }

    pub fn char(span: Span) -> Self {
        Self::new(ExprKind::Char, span)
    }

    pub fn int(span: Span) -> Self {
        Self::new(ExprKind::Int, span)
    }

    pub fn float(span: Span) -> Self {
        Self::new(ExprKind::Float, span)
    }

    pub fn str(span: Span) -> Self {
        Self::new(ExprKind::Str, span)
    }

    pub fn prefix(expr: ExprID, op: PrefixOperator, span: Span) -> Self {
        Self::new(ExprKind::Prefix(PrefixExpr { expr, op }), span)
    }

    pub fn postfix(expr: ExprID, op: PostfixOperator, span: Span) -> Self {
        Self::new(ExprKind::Postfix(PostfixExpr { expr, op }), span)
    }

    pub fn binary(left: ExprID, right: ExprID, operator: BinaryOperator, span: Span) -> Self {
        Self::new(
            ExprKind::Binary(BinaryExpr {
                left,
                right,
                operator,
            }),
            span,
        )
    }

    pub fn variable(ident: Ident) -> Self {
        Self::new(ExprKind::Variable(ident), ident.span)
    }

    pub fn cast(expr: ExprID, ty: TypeID, span: Span) -> Self {
        Self::new(ExprKind::Cast(CastExpr { expr, ty }), span)
    }

    pub fn block(block: Vec<StmtID>, span: Span) -> Self {
        Self::new(ExprKind::Block(BlockExpr { items: block }), span)
    }

    pub fn call(callee: ExprID, args: Vec<ExprID>, generics: Vec<TypeID>, span: Span) -> Self {
        Self::new(
            ExprKind::Call(CallExpr {
                callee,
                args,
                generics,
            }),
            span,
        )
    }

    pub fn index(expr: ExprID, span: Span) -> Self {
        Self::new(ExprKind::Index(IndexExpr { expr }), span)
    }

    pub fn field(callee: ExprID, field: ExprID, span: Span) -> Self {
        Self::new(ExprKind::Field(FieldExpr { callee, field }), span)
    }

    pub fn if_(condition: ExprID, body: ExprID, branch: Option<ExprID>, span: Span) -> Self {
        Self::new(
            ExprKind::If(IfExpr {
                condition,
                body,
                branch,
            }),
            span,
        )
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    // r(ErrorGuaranteed)
    Bool(bool),
    Byte,
    Char,
    Int,
    Float,
    Str,
    StrRaw(u8), // raw string delimited by `n` hash symbols
    ByteStr,
    ByteStrRaw(u8), // raw byte string delimited by `n` hash symbols
    CStr,
    CStrRaw(u8),

    Prefix(PrefixExpr),
    Postfix(PostfixExpr),
    Binary(BinaryExpr),
    Assignment(AssignExpr),

    Variable(Ident),

    Cast(CastExpr),

    Block(BlockExpr),
    Call(CallExpr),
    Index(IndexExpr),
    Field(FieldExpr),
    //
    // Switch(SwitchExpr),
    If(IfExpr),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrefixOperatorKind {
    Negate,
    Not,
    BitNot,
}

impl Display for PrefixOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use PrefixOperatorKind::*;

        match self {
            Negate => write!(f, "Negate (-)"),
            Not => write!(f, "Not (!)"),
            BitNot => write!(f, "Bitwise Not (~)"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrefixOperator {
    pub kind: PrefixOperatorKind,
    pub span: Span,
}

impl PrefixOperator {
    pub fn new(kind: PrefixOperatorKind, span: Span) -> Self {
        PrefixOperator { kind, span }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrefixExpr {
    pub op: PrefixOperator,
    pub expr: ExprID,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOperatorKind {
    Unwrap,
}

impl Display for PostfixOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use PostfixOperatorKind::*;

        match self {
            Unwrap => write!(f, "Unwrap (?)"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostfixOperator {
    pub kind: PostfixOperatorKind,
    pub span: Span,
}

impl PostfixOperator {
    pub fn new(kind: PostfixOperatorKind, span: Span) -> Self {
        PostfixOperator { kind, span }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PostfixExpr {
    pub expr: ExprID,
    pub op: PostfixOperator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    Equal,
    NotEqual,

    Less,
    LessEqual,
    LeftShift,

    Greater,
    GreaterEqual,
    RightShift,

    BitAnd,
    BitOr,
    BitXor,

    LogicAnd,
    LogicOr,

    Range,
    RangeInclusive,
}

impl Display for BinaryOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use BinaryOperatorKind::*;

        match self {
            Add => write!(f, "Add (+)"),
            Subtract => write!(f, "Subtract (-)"),
            Multiply => write!(f, "Multiply (*)"),
            Divide => write!(f, "Divide (/)"),
            Remainder => write!(f, "Remainder (%)"),

            Equal => write!(f, "Equal (==)"),
            NotEqual => write!(f, "Not Equal (!=)"),

            Less => write!(f, "Less (<)"),
            LessEqual => write!(f, "Less or Equal (<=)"),
            LeftShift => write!(f, "Left Bit Shift (<<)"),

            Greater => write!(f, "Greater (>)"),
            GreaterEqual => write!(f, "Greater or Equal (>=)"),
            RightShift => write!(f, "Right Bit Shift (>>)"),

            BitAnd => write!(f, "Bitwise And (&)"),
            BitOr => write!(f, "Bitwise Or (|)"),
            BitXor => write!(f, "Bitwise Xor (^)"),

            LogicAnd => write!(f, "Logical And (&&)"),
            LogicOr => write!(f, "Logical Or (||)"),

            Range => write!(f, "Range (..)"),
            RangeInclusive => write!(f, "Range Inclusive (..=)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub span: Span,
}

impl BinaryOperator {
    pub fn new(kind: BinaryOperatorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BinaryExpr {
    pub left: ExprID,
    pub right: ExprID,
    pub operator: BinaryOperator,
}

impl BinaryExpr {
    pub fn new(left: ExprID, right: ExprID, operator: BinaryOperator) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOperatorKind {
    Default,

    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    LeftShift,
    RightShift,

    BitAnd,
    BitOr,
    BitXor,
    BitNot,
}

impl Display for AssignOperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use AssignOperatorKind::*;

        match self {
            Default => write!(f, "Default"),

            Add => write!(f, "Add (+)"),
            Subtract => write!(f, "Subtract (-)"),
            Multiply => write!(f, "Multiply (*)"),
            Divide => write!(f, "Divide (/)"),
            Remainder => write!(f, "Remainder (%)"),

            LeftShift => write!(f, "Left Bit Shift (<<)"),
            RightShift => write!(f, "Right Bit Shift (>>)"),

            BitAnd => write!(f, "Bitwise And (&)"),
            BitOr => write!(f, "Bitwise Or (|)"),
            BitXor => write!(f, "Bitwise Xor (^)"),
            BitNot => write!(f, "Bitwise Not (~)"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AssignOperator {
    kind: AssignOperatorKind,
    span: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct AssignExpr {
    expr: ExprID,
    op: AssignOperator,
}

#[derive(Debug, Clone, Copy)]
pub struct CastExpr {
    expr: ExprID,
    ty: TypeID,
}

#[derive(Debug, Clone)]
pub struct BlockExpr {
    items: Vec<StmtID>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    callee: ExprID,
    args: Vec<ExprID>,
    generics: Vec<TypeID>,
}

#[derive(Debug, Clone)]
pub struct IndexExpr {
    expr: ExprID,
}

#[derive(Debug, Clone)]
pub struct FieldExpr {
    callee: ExprID,
    field: ExprID,
}

// #[derive(Debug, Clone)]
// struct SwitchExpr {
//     field: Type
// }

#[derive(Debug, Clone)]
pub struct IfExpr {
    condition: ExprID,
    body: ExprID,
    branch: Option<ExprID>,
}
