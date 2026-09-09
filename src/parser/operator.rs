use crate::{
    ast::expr::{
        BinaryOperator, BinaryOperatorKind, PostfixOperator, PostfixOperatorKind, PrefixOperator,
        PrefixOperatorKind,
    },
    lexer::token::TokenKind,
    parser::Parser,
    source::Span,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
        let start = self.pos();
        let arr = [self.peek(0), self.peek(1), self.peek(2)];

        let (kind, len) = match arr[0].kind {
            TokenKind::Plus => (BinaryOperatorKind::Add, 1),

            TokenKind::Minus => (BinaryOperatorKind::Subtract, 1),

            TokenKind::Asterisk => (BinaryOperatorKind::Multiply, 1),

            TokenKind::Slash => (BinaryOperatorKind::Divide, 1),

            TokenKind::Percent => (BinaryOperatorKind::Remainder, 1),

            TokenKind::Eq if arr[0].is_joint() && arr[1].kind == TokenKind::Eq => {
                (BinaryOperatorKind::Equal, 2)
            }

            TokenKind::Bang if arr[0].is_joint() && arr[1].kind == TokenKind::Eq => {
                (BinaryOperatorKind::NotEqual, 2)
            }

            TokenKind::LeftAngle => match arr[1].kind {
                TokenKind::LeftAngle => (BinaryOperatorKind::LeftShift, 2),

                TokenKind::Eq => (BinaryOperatorKind::LessEqual, 2),

                _ => (BinaryOperatorKind::Less, 1),
            },

            TokenKind::RightAngle => match arr[1].kind {
                TokenKind::RightAngle => (BinaryOperatorKind::RightShift, 2),

                TokenKind::Eq => (BinaryOperatorKind::GreaterEqual, 2),

                _ => (BinaryOperatorKind::Greater, 1),
            },

            TokenKind::Amp => {
                if arr[0].is_joint() && arr[1].kind == TokenKind::Amp {
                    (BinaryOperatorKind::LogicAnd, 2)
                } else {
                    (BinaryOperatorKind::BitAnd, 1)
                }
            }

            TokenKind::Pipe => {
                if arr[0].is_joint() && arr[1].kind == TokenKind::Pipe {
                    (BinaryOperatorKind::LogicOr, 2)
                } else {
                    (BinaryOperatorKind::BitOr, 1)
                }
            }

            TokenKind::Caret => (BinaryOperatorKind::BitXor, 1),

            TokenKind::Dot if arr[0].is_joint() && arr[1].kind == TokenKind::Dot => {
                if arr[1].is_joint() && arr[2].kind == TokenKind::Eq {
                    (BinaryOperatorKind::RangeInclusive, 3)
                } else {
                    (BinaryOperatorKind::Range, 2)
                }
            }

            _ => return None,
        };

        Some(BinaryOperator::new(
            kind,
            Span::new(start, len, self.file_id()),
        ))
    }

    pub(super) fn parse_prefix_operator(&mut self) -> Option<PrefixOperator> {
        let start = self.pos();

        let kind = match self.peek(0).kind {
            TokenKind::Minus => PrefixOperatorKind::Negate,
            TokenKind::Bang => PrefixOperatorKind::Not,
            TokenKind::Tilde => PrefixOperatorKind::BitNot,
            _ => return None,
        };

        Some(PrefixOperator::new(
            kind,
            Span::new(start, 1, self.file_id()),
        ))
    }

    pub(super) fn parse_postfix_operator(&mut self) -> Option<PostfixOperator> {
        let start = self.pos();

        match self.peek(0).kind {
            TokenKind::Question => Some(PostfixOperator::new(
                PostfixOperatorKind::Unwrap,
                Span::new(start, 1, self.file_id()),
            )),

            _ => None,
        }
    }
}

impl BinaryOperatorKind {
    pub fn precedence(&self) -> u8 {
        use BinaryOperatorKind::*;

        match self {
            // a..b
            // a..=b
            Range | RangeInclusive => 1,

            // a || b
            LogicOr => 2,

            // a && b
            LogicAnd => 3,

            // a | b
            BitOr => 4,

            // a ^ b
            BitXor => 5,

            // a & b
            BitAnd => 6,

            // == !=
            Equal | NotEqual => 7,

            // < <= > >=
            Less | LessEqual | Greater | GreaterEqual => 8,

            // << >>
            LeftShift | RightShift => 9,

            // + -
            Add | Subtract => 10,

            // * / %
            Multiply | Divide | Remainder => 11,
        }
    }
}
