mod expr;
mod operator;
mod stmt;
mod types;

use std::collections::VecDeque;

use bumpalo::Bump;

use crate::{
    ast::{
        AST, Ident, Item, Publicity, PublicityKind,
        string::{StringID, StringPool},
    },
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    source::SourceFile,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    strings: StringPool<'a>,
    pub ast: AST,
    buffer: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(src: &'a SourceFile, bump: &'a Bump) -> Self {
        Self {
            lexer: Lexer::new(src),
            strings: StringPool::new(bump),
            ast: AST::new(),
            buffer: VecDeque::with_capacity(8),
        }
    }

    pub(super) fn pos(&self) -> usize {
        self.lexer.pos
    }

    pub(super) fn file_id(&self) -> usize {
        self.lexer.file.file_id
    }

    pub fn make_ident(&mut self, token: &Token) -> Ident {
        let id = match &token.kind {
            TokenKind::Ident => {
                let text = self
                    .lexer
                    .file
                    .content
                    .get(token.span.as_range())
                    .unwrap_or("")
                    .as_bytes();
                self.strings.intern(text)
            }

            _ => StringID::EMPTY,
        };

        Ident::new(id, token.span)
    }

    fn ensure_buffered(&mut self, len: usize) {
        while self.buffer.len() <= len {
            let token = self.lexer.next();

            println!("{:?} @ {:?}", token.kind, token.span);

            self.buffer.push_back(token);
        }
    }

    pub(self) fn peek(&mut self, offset: usize) -> Token {
        self.ensure_buffered(offset);

        self.buffer[offset]
    }

    pub(self) fn advance(&mut self, len: usize) {
        for _ in 0..len {
            if self.buffer.pop_front().is_none() {
                let _ = self.lexer.next();
            }
        }
    }

    pub(self) fn consume(&mut self) -> Token {
        self.ensure_buffered(0);

        self.buffer.pop_front().unwrap()
    }

    pub(self) fn expect(&mut self, kind: TokenKind) -> Result<Token, ()> {
        if self.peek(0).kind == kind {
            return Ok(self.consume());
        }

        Err(())
    }

    pub fn next(&mut self) -> Result<Item, ()> {
        if self.peek(0).kind == TokenKind::EOF {
            return Ok(Item::EOF);
        }

        let stmt = self.parse_stmt()?;

        Ok(Item::Stmt(stmt))
    }

    pub(self) fn parse_publicity(&mut self) -> Result<Option<Publicity>, ()> {
        let first = self.peek(0);

        if first.kind != TokenKind::KwPub {
            return Ok(None);
        }

        self.advance(1);

        if self.peek(0).kind != TokenKind::LeftParen {
            return Ok(Some(Publicity::new(PublicityKind::Public, first.span)));
        }

        let _ = self.consume();

        let kind = match self.peek(0).kind {
            TokenKind::KwSelfRef => {
                self.advance(1);
                PublicityKind::Self_
            }

            TokenKind::KwModule => {
                self.advance(1);
                PublicityKind::Module
            }

            TokenKind::KwSuper => {
                self.advance(1);
                PublicityKind::Super
            }

            TokenKind::KwRoot => {
                self.advance(1);
                PublicityKind::Root
            }

            _ => return Err(()),
        };

        let close = self.expect(TokenKind::RightParen)?;

        Ok(Some(Publicity::new(kind, first.span.merge(close.span))))
    }

    fn parse_separated<T>(
        &mut self,
        delimiter: TokenKind,
        end: TokenKind,
        trailing: Trailing,
        mut parse: impl FnMut(&mut Self) -> Result<T, ()>,
    ) -> Result<Vec<T>, ()> {
        let mut items = Vec::new();

        if self.peek(0).kind == end {
            return Ok(items);
        }

        loop {
            items.push(parse(self)?);

            match self.peek(0).kind {
                kind if kind == delimiter => {
                    self.advance(1);

                    // trailing comma
                    if self.peek(0).kind == end {
                        match trailing {
                            Trailing::Allowed => break,
                            Trailing::Forbidden => return Err(()),
                        }
                    }
                }

                kind if kind == end => break,

                _ => return Err(()),
            }
        }

        Ok(items)
    }
}

pub enum Trailing {
    Allowed,
    Forbidden,
}
