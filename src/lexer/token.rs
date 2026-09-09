use crate::source::Span;
use phf::phf_map;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    EOF,
    Err,

    Ident,
    Int,
    Float,
    Str,
    ByteStr,
    CStr,
    RawStr,
    RawByteStr,
    Char,
    ByteChar,

    KwDec,
    KwMut,
    KwPub,
    KwConst,

    KwStruct,
    KwEnum,
    KwUnion,
    KwType,
    KwTrait,

    KwFn,
    KwImpl,

    KwComptime,

    KwAs,

    KwImport,
    KwSelf,
    KwSelfType,
    KwModule,
    KwSuper,
    KwRoot,

    KwTrue,
    KwFalse,

    KwIf,
    KwElse,
    KwSwitch,

    KwFor,
    KwIn,
    KwWhile,
    KwLoop,

    KwBreak,
    KwContinue,
    KwReturn,

    KwDefer,
    KwErrdefer,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Eq,

    Tilde,
    Caret,
    Amp,
    Pipe,

    Bang,

    LeftAngle,
    RightAngle,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,

    Dot,
    Comma,
    Colon,
    Semicolon,
    Question,
    // Dollar,
    Arrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing {
    Alone,
    Joint,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub spacing: Spacing,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, spacing: Spacing) -> Self {
        Self {
            kind,
            span,
            spacing,
        }
    }

    pub fn is_joint(&self) -> bool {
        self.spacing == Spacing::Joint
    }
}

pub static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "dec" => TokenKind::KwDec,
    "mut" => TokenKind::KwMut,
    "pub" => TokenKind::KwPub,
    "const" => TokenKind::KwConst,

    "struct" => TokenKind::KwStruct,
    "enum" => TokenKind::KwEnum,
    "union" => TokenKind::KwUnion,
    "type" => TokenKind::KwType,
    "trait" => TokenKind::KwTrait,

    "fn" => TokenKind::KwFn,
    "impl" => TokenKind::KwImpl,

    "comptime" => TokenKind::KwComptime,

    "as" => TokenKind::KwAs,

    "import" => TokenKind::KwImport,
    "self" => TokenKind::KwSelf,
    "Self" => TokenKind::KwSelfType,
    "module" => TokenKind::KwModule,
    "super" => TokenKind::KwSuper,
    "root" => TokenKind::KwRoot,

    "true" => TokenKind::KwTrue,
    "false" => TokenKind::KwFalse,

    "if" => TokenKind::KwIf,
    "else" => TokenKind::KwElse,
    "switch" => TokenKind::KwSwitch,

    "for" => TokenKind::KwFor,
    "in" => TokenKind::KwIn,
    "while" => TokenKind::KwWhile,
    "loop" => TokenKind::KwLoop,

    "break" => TokenKind::KwBreak,
    "continue" => TokenKind::KwContinue,
    "return" => TokenKind::KwReturn,

    "defer" => TokenKind::KwDefer,
    "errdefer" => TokenKind::KwErrdefer,
};
