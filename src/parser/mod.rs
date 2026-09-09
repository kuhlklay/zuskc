mod expr;
mod items;
mod operator;
mod patterns;
mod restrictions;
mod stmt;
mod types;

use std::collections::VecDeque;

use bumpalo::Bump;

use crate::{
    ast::{
        AST, Ident, ItemID, Publicity, PublicityKind,
        items::{Item, ItemKind},
        string::StringPool,
    },
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::restrictions::Restrictions,
    source::SourceFile,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    buffer: VecDeque<Token>,
    restrictions: Restrictions,
    strings: StringPool<'a>,
    ast: AST,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a SourceFile, bump: &'a Bump) -> Self {
        Self {
            lexer: Lexer::new(src),
            buffer: VecDeque::with_capacity(8),
            restrictions: Restrictions::empty(),
            strings: StringPool::new(bump),
            ast: AST::new(),
        }
    }

    fn pos(&self) -> usize {
        self.lexer.pos
    }

    fn file_id(&self) -> usize {
        self.lexer.file.file_id
    }

    #[allow(dead_code)]
    pub fn ast(&self) -> &AST {
        &self.ast
    }

    #[allow(dead_code)]
    pub fn ast_mut(&mut self) -> &mut AST {
        &mut self.ast
    }

    fn with_restrictions<T>(
        &mut self,
        restrictions: Restrictions,
        f: impl FnOnce(&mut Self) -> Result<T, ()>,
    ) -> Result<T, ()> {
        let old = self.restrictions;

        self.restrictions |= restrictions;

        let result = f(self);

        self.restrictions = old;

        result
    }

    fn make_ident(&mut self, token: &Token) -> Ident {
        let id = {
            let text = self
                .lexer
                .file
                .content
                .get(token.span.as_range())
                .unwrap_or("")
                .as_bytes();
            self.strings.intern(text)
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

    fn peek(&mut self, offset: usize) -> Token {
        self.ensure_buffered(offset);

        self.buffer[offset]
    }

    fn advance(&mut self, len: usize) {
        for _ in 0..len {
            if self.buffer.pop_front().is_none() {
                let _ = self.lexer.next();
            }
        }
    }

    fn consume(&mut self) -> Token {
        self.ensure_buffered(0);

        self.buffer.pop_front().unwrap()
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ()> {
        if self.peek(0).kind == kind {
            return Ok(self.consume());
        }

        Err(())
    }

    pub fn next(&mut self) -> Result<ItemID, ()> {
        let current = self.peek(0);

        if current.kind == TokenKind::EOF {
            return Ok(self
                .ast
                .items()
                .push(Item::new(ItemKind::EOF, current.span)));
        }

        self.parse_item()
    }

    fn parse_publicity(&mut self) -> Result<Option<Publicity>, ()> {
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
            TokenKind::KwSelf => {
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
        delimiter: Option<(TokenKind, Trailing)>,
        end: TokenKind,
        mut parse: impl FnMut(&mut Self) -> Result<T, ()>,
    ) -> Result<Vec<T>, ()> {
        let mut items = Vec::new();

        if self.peek(0).kind == end {
            return Ok(items);
        }

        loop {
            items.push(parse(self)?);

            if self.peek(0).kind == end {
                break;
            }

            let Some((delimiter_kind, ref trailing)) = delimiter else {
                return Err(());
            };

            if self.peek(0).kind != delimiter_kind {
                return Err(());
            }

            self.advance(1);

            if self.peek(0).kind == end {
                match trailing {
                    Trailing::Allowed => break,
                    Trailing::Forbidden => return Err(()),
                }
            }
        }

        Ok(items)
    }
}

#[allow(dead_code)]
enum Trailing {
    Allowed,
    Forbidden,
}
