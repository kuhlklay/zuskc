mod cursor;
mod expr;
mod items;
mod operator;
mod patterns;
mod restrictions;
mod stmt;
mod types;

use crate::{
    ast::{
        AST, Ident, ItemID, Publicity,
        items::{Item, ItemKind},
    },
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::{
        cursor::{TokenCursor, TokenStream},
        restrictions::Restrictions,
    },
    session::Session,
    source::{FileID, SourceFile},
    utils::sync::Lrc,
};

pub struct Parser<'s> {
    session: &'s Session,
    file: Lrc<SourceFile>,
    cursor: TokenCursor,
    restrictions: Restrictions,
    ast: AST,
}

impl<'s> Parser<'s> {
    pub fn new(session: &'s Session, file_id: FileID) -> Self {
        let file = session.file(file_id);
        let tokens = Lexer::new(Lrc::clone(&file)).collect();
        let stream = TokenStream::new(tokens);
        Self {
            session,
            file,
            cursor: TokenCursor::new(stream),
            restrictions: Restrictions::empty(),
            ast: AST::new(),
        }
    }

    pub fn pos(&self) -> usize {
        self.cursor.pos()
    }

    pub fn file_id(&self) -> FileID {
        self.file.id
    }

    pub fn peek(&self, n: usize) -> &Token {
        self.cursor.peek(n)
    }

    pub fn at_eof(&self) -> bool {
        self.cursor.at_eof()
    }

    pub fn bump(&mut self) -> Token {
        self.cursor.bump()
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, ()> {
        if self.peek(0).kind == kind {
            Ok(self.bump())
        } else {
            Err(())
        }
    }

    pub fn checkpoint(&self) -> usize {
        self.cursor.checkpoint()
    }

    pub fn rollback(&mut self, cp: usize) {
        self.cursor.rollback(cp);
    }

    pub fn ast(&self) -> &AST {
        &self.ast
    }

    pub fn take_ast(self) -> AST {
        self.ast
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

    pub fn make_ident(&mut self, tok: &Token) -> Ident {
        let text = &self.file.content[tok.span.as_range()];
        let id = self.session.intern(text);
        Ident::new(id, tok.span)
    }

    pub fn advance(&mut self, n: usize) {
        for _ in 0..n {
            self.bump();
        }
    }

    pub fn consume(&mut self) -> Token {
        let token = *self.peek(0);
        self.advance(1);
        token
    }

    pub fn next(&mut self) -> Result<ItemID, ()> {
        let current = *self.peek(0);

        if self.at_eof() {
            return Ok(self
                .ast
                .items()
                .push(Item::new(ItemKind::EOF, None, current.span)));
        }

        self.parse_item()
    }

    fn parse_publicity(&mut self) -> Result<Publicity, ()> {
        let first = *self.peek(0);

        if first.kind != TokenKind::KwPub {
            return Ok(Publicity::Private);
        }

        self.advance(1);

        if self.peek(0).kind != TokenKind::LeftParen {
            return Ok(Publicity::Public { span: first.span });
        }

        let _ = self.consume();

        match self.peek(0).kind {
            TokenKind::KwSelf => {
                self.advance(1);
                let close = self.expect(TokenKind::RightParen)?;

                Ok(Publicity::Self_ {
                    span: first.span.merge(close.span),
                })
            }

            TokenKind::KwModule => {
                self.advance(1);
                let close = self.expect(TokenKind::RightParen)?;

                Ok(Publicity::Module {
                    span: first.span.merge(close.span),
                })
            }

            TokenKind::KwSuper => {
                self.advance(1);
                let close = self.expect(TokenKind::RightParen)?;

                Ok(Publicity::Super {
                    span: first.span.merge(close.span),
                })
            }

            TokenKind::KwRoot => {
                self.advance(1);
                let close = self.expect(TokenKind::RightParen)?;

                Ok(Publicity::Root {
                    span: first.span.merge(close.span),
                })
            }

            _ => Err(()),
        }
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

            if let Some((kind, ref trailing)) = delimiter {
                if self.peek(0).kind != kind {
                    return Err(());
                }
                self.advance(1);
                if self.peek(0).kind == end {
                    match trailing {
                        Trailing::Allowed => break,
                        Trailing::Forbidden => return Err(()),
                    }
                }
            };
        }
        Ok(items)
    }
}

enum Trailing {
    Allowed,
    Forbidden,
}
