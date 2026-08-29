use crate::{
    ast::{
        Mutability, Publicity, PublicityKind, StmtID,
        stmt::{EnumVariant, Stmt, StructField, UnionVariant, UnionVariantKind},
    },
    lexer::token::TokenKind,
    parser::{Parser, Trailing},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_stmt(&mut self) -> Result<StmtID, ()> {
        let public = self.parse_publicity()?;

        if public.is_some() {
            return match self.peek(0).kind {
                TokenKind::KwConst => self.parse_const_stmt(public),

                TokenKind::KwType => self.parse_type_dec_stmt(public),
                TokenKind::KwStruct => self.parse_struct_dec_stmt(public),
                TokenKind::KwUnion => self.parse_union_dec_stmt(public),
                TokenKind::KwEnum => self.parse_enum_dec_stmt(public),

                _ => Err(()),
            };
        }

        match self.peek(0).kind {
            TokenKind::KwDec => self.parse_var_stmt(),
            TokenKind::KwConst => self.parse_const_stmt(None),

            TokenKind::KwType => self.parse_type_dec_stmt(None),
            TokenKind::KwStruct => self.parse_struct_dec_stmt(None),
            TokenKind::KwUnion => self.parse_union_dec_stmt(None),
            TokenKind::KwEnum => self.parse_enum_dec_stmt(None),

            TokenKind::KwIf => {
                let expr = self.parse_if_expr()?;

                let span = self.ast.expr(expr).span;

                Ok(self.ast.add_stmt(Stmt::expr(expr, span)))
            }

            _ => self.parse_expr_stmt(),
        }
    }

    pub fn parse_var_stmt(&mut self) -> Result<StmtID, ()> {
        // dec [mut] <Ident> [: <Type>] = <Expr>;

        let dec_token = self.consume();

        let mutable = if self.peek(0).kind == TokenKind::KwMut {
            self.advance(1);

            Mutability::Mutable {
                span: self.peek(0).span,
            }
        } else {
            Mutability::Immutable
        };

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let ty = if self.peek(0).kind == TokenKind::Colon {
            self.advance(1);

            Some(self.parse_type()?)
        } else {
            None
        };

        let _ = self.expect(TokenKind::Eq)?;

        let init = self.parse_expr()?;

        let _ = self.expect(TokenKind::Semicolon)?;

        let span = dec_token.span.merge(self.ast.expr(init).span);

        Ok(self
            .ast
            .add_stmt(Stmt::var_dec(ident, mutable, ty, init, span)))
    }

    pub fn parse_const_stmt(&mut self, publicity: Option<Publicity>) -> Result<StmtID, ()> {
        let const_token = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let ty = if self.peek(0).kind == TokenKind::Colon {
            self.advance(1);

            Some(self.parse_type()?)
        } else {
            None
        };

        let _ = self.expect(TokenKind::Eq)?;

        let expr = self.parse_expr()?;

        let _ = self.expect(TokenKind::Semicolon)?;

        let span = if let Some(public) = publicity {
            public.span.merge(self.ast.expr(expr).span)
        } else {
            const_token.span.merge(self.ast.expr(expr).span)
        };

        Ok(self
            .ast
            .add_stmt(Stmt::const_dec(ident, publicity, ty, expr, span)))
    }

    pub fn parse_type_dec_stmt(&mut self, publicity: Option<Publicity>) -> Result<StmtID, ()> {
        let type_kw = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let generics = if self.peek(0).kind == TokenKind::LeftAngle {
            Some(self.parse_generic_params()?)
        } else {
            None
        };

        let _ = self.expect(TokenKind::Eq)?;

        let target = self.parse_type()?;

        let _ = self.expect(TokenKind::Semicolon)?;

        let span = type_kw.span.merge(self.ast.ty(target).span);

        Ok(self
            .ast
            .add_stmt(Stmt::type_dec(ident, publicity, generics, target, span)))
    }

    pub fn parse_struct_dec_stmt(&mut self, publicity: Option<Publicity>) -> Result<StmtID, ()> {
        let struct_kw = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let generics = if self.peek(0).kind == TokenKind::LeftAngle {
            Some(self.parse_generic_params()?)
        } else {
            None
        };

        let _ = self.expect(TokenKind::LeftCurly)?;

        let fields = self.parse_separated(
            TokenKind::Comma,
            TokenKind::RightCurly,
            Trailing::Allowed,
            Self::parse_struct_field,
        )?;

        let close = self.expect(TokenKind::RightCurly)?;

        let span = if let Some(public) = publicity {
            public.span.merge(close.span)
        } else {
            struct_kw.span.merge(close.span)
        };

        Ok(self
            .ast
            .add_stmt(Stmt::struct_dec(ident, publicity, generics, fields, span)))
    }

    fn parse_struct_field(&mut self) -> Result<StructField, ()> {
        let publicity = if self.peek(0).kind == TokenKind::KwPub {
            self.advance(1);
            Some(Publicity::new(PublicityKind::Public, self.peek(0).span))
        } else {
            None
        };

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let _ = self.expect(TokenKind::Colon)?;

        let ty = self.parse_type()?;

        Ok(StructField {
            publicity,
            ident,
            ty,
        })
    }

    pub fn parse_union_dec_stmt(&mut self, publicity: Option<Publicity>) -> Result<StmtID, ()> {
        let union_kw = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let generics = if self.peek(0).kind == TokenKind::LeftAngle {
            Some(self.parse_generic_params()?)
        } else {
            None
        };

        let _ = self.expect(TokenKind::LeftCurly)?;

        let variants = self.parse_separated(
            TokenKind::Comma,
            TokenKind::RightCurly,
            Trailing::Allowed,
            Self::parse_union_variant,
        )?;

        let close = self.expect(TokenKind::RightCurly)?;

        let span = if let Some(public) = publicity {
            public.span.merge(close.span)
        } else {
            union_kw.span.merge(close.span)
        };

        Ok(self
            .ast
            .add_stmt(Stmt::union_dec(ident, publicity, generics, variants, span)))
    }

    fn parse_union_variant(&mut self) -> Result<UnionVariant, ()> {
        let token = self.expect(TokenKind::Ident)?;
        let name = self.make_ident(&token);

        let kind = match self.peek(0).kind {
            TokenKind::LeftParen => {
                self.advance(1);

                let types = self.parse_separated(
                    TokenKind::Comma,
                    TokenKind::RightParen,
                    Trailing::Allowed,
                    Self::parse_type,
                )?;

                self.expect(TokenKind::RightParen)?;

                UnionVariantKind::Tuple { ty: types }
            }

            TokenKind::LeftCurly => {
                // TODO: named fields
                todo!()
            }

            _ => UnionVariantKind::Unit,
        };

        let span = token.span;

        Ok(UnionVariant { name, kind, span })
    }

    pub fn parse_enum_dec_stmt(&mut self, publicity: Option<Publicity>) -> Result<StmtID, ()> {
        let enum_kw = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let _ = self.expect(TokenKind::LeftCurly)?;

        let variants = self.parse_separated(
            TokenKind::Comma,
            TokenKind::RightCurly,
            Trailing::Allowed,
            Self::parse_enum_field,
        )?;

        let close = self.expect(TokenKind::RightCurly)?;

        let span = if let Some(public) = publicity {
            public.span.merge(close.span)
        } else {
            enum_kw.span.merge(close.span)
        };

        Ok(self
            .ast
            .add_stmt(Stmt::enum_dec(ident, publicity, variants, span)))
    }

    fn parse_enum_field(&mut self) -> Result<EnumVariant, ()> {
        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let value = if self.peek(0).kind == TokenKind::Eq {
            self.advance(1);
            Some(self.parse_expr()?)
        } else {
            None
        };

        Ok(EnumVariant { ident, value })
    }

    pub fn parse_expr_stmt(&mut self) -> Result<StmtID, ()> {
        let expr = self.parse_expr()?;
        let span = self.ast.expr(expr).span;

        Ok(self.ast.add_stmt(Stmt::expr(expr, span)))
    }
}
