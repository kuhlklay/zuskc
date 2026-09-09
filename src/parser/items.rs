use core::panic;

use crate::{
    ast::{
        Ident, ItemID, Publicity, PublicityKind,
        items::{EnumVariant, FnParam, Item, StructField, UnionVariant, UnionVariantKind},
        patterns::Pat,
        types::Ty,
    },
    lexer::token::TokenKind,
    parser::{Parser, Trailing, restrictions::Restrictions},
    source::Span,
};

impl<'a> Parser<'a> {
    pub fn parse_item(&mut self) -> Result<ItemID, ()> {
        let publicity = self.parse_publicity()?;

        match (self.peek(0).kind, publicity) {
            (TokenKind::KwConst, _) => {
                if self.peek(1).kind == TokenKind::KwFn {
                    let const_span = self.consume().span;

                    self.parse_fn_dec_item(publicity, Some(const_span))
                } else {
                    self.parse_const_item(publicity)
                }
            }

            (TokenKind::KwType, _) => self.parse_type_dec_item(publicity),

            (TokenKind::KwStruct, _) => self.parse_struct_dec_item(publicity),

            (TokenKind::KwUnion, _) => self.parse_union_dec_item(publicity),

            (TokenKind::KwEnum, _) => self.parse_enum_dec_item(publicity),

            (TokenKind::KwFn, _) => self.parse_fn_dec_item(publicity, None),

            (TokenKind::KwImpl, None) => self.parse_impl_item(),

            (TokenKind::KwTrait, _) => self.parse_trait_item(publicity),

            (_, Some(_)) => panic!("unexpected public keyword (not allowed here)"),

            _ => panic!("expected item"),
        }
    }

    pub fn parse_const_item(&mut self, publicity: Option<Publicity>) -> Result<ItemID, ()> {
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
            public.span.merge(self.ast.exprs()[expr.index()].span)
        } else {
            const_token.span.merge(self.ast.exprs()[expr.index()].span)
        };

        Ok(self
            .ast
            .items()
            .push(Item::const_dec(ident, publicity, ty, expr, span)))
    }

    pub fn parse_type_dec_item(&mut self, publicity: Option<Publicity>) -> Result<ItemID, ()> {
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

        let span = type_kw.span.merge(self.ast.types()[target.index()].span);

        Ok(self
            .ast
            .items()
            .push(Item::type_dec(ident, publicity, generics, target, span)))
    }

    pub fn parse_struct_dec_item(&mut self, publicity: Option<Publicity>) -> Result<ItemID, ()> {
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
            Some((TokenKind::Comma, Trailing::Allowed)),
            TokenKind::RightCurly,
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
            .items()
            .push(Item::struct_dec(ident, publicity, generics, fields, span)))
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

    pub fn parse_union_dec_item(&mut self, publicity: Option<Publicity>) -> Result<ItemID, ()> {
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
            Some((TokenKind::Comma, Trailing::Allowed)),
            TokenKind::RightCurly,
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
            .items()
            .push(Item::union_dec(ident, publicity, generics, variants, span)))
    }

    fn parse_union_variant(&mut self) -> Result<UnionVariant, ()> {
        let token = self.expect(TokenKind::Ident)?;
        let name = self.make_ident(&token);

        let kind = match self.peek(0).kind {
            TokenKind::LeftParen => {
                self.advance(1);

                let types = self.parse_separated(
                    Some((TokenKind::Comma, Trailing::Allowed)),
                    TokenKind::RightParen,
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

    pub fn parse_enum_dec_item(&mut self, publicity: Option<Publicity>) -> Result<ItemID, ()> {
        let enum_kw = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let _ = self.expect(TokenKind::LeftCurly)?;

        let variants = self.parse_separated(
            Some((TokenKind::Comma, Trailing::Allowed)),
            TokenKind::RightCurly,
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
            .items()
            .push(Item::enum_dec(ident, publicity, variants, span)))
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

    pub fn parse_fn_dec_item(
        &mut self,
        publicity: Option<Publicity>,
        constant: Option<Span>,
    ) -> Result<ItemID, ()> {
        let fn_token = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let generics = if self.peek(0).kind == TokenKind::LeftAngle {
            Some(self.parse_generic_params()?)
        } else {
            None
        };

        let _ = self.expect(TokenKind::LeftParen)?;

        let mut param_idx = 0;

        let params = if self.peek(0).kind != TokenKind::RightParen {
            Some(self.parse_separated(
                Some((TokenKind::Comma, Trailing::Allowed)),
                TokenKind::RightParen,
                |p| p.parse_function_param(&mut param_idx),
            )?)
        } else {
            None
        };

        let close = self.expect(TokenKind::RightParen)?;

        let ty = if self.peek(0).kind == TokenKind::Minus
            && self.peek(0).is_joint()
            && self.peek(1).kind == TokenKind::RightAngle
        {
            self.advance(2);

            Some(self.parse_type()?)
        } else {
            None
        };

        let body = match self.peek(0).kind {
            TokenKind::LeftCurly => Ok(Some(self.parse_block_expr()?)),

            TokenKind::Semicolon => {
                if self.restrictions.contains(Restrictions::IN_TRAIT) {
                    self.advance(1);
                    Ok(None)
                } else {
                    Err(())
                }
            }

            _ => Err(()),
        }?;

        let last_span = if let Some(b) = body {
            self.ast.exprs()[b.index()].span
        } else if let Some(t) = ty {
            self.ast.types()[t.index()].span
        } else {
            close.span
        };

        let span = if let Some(p) = publicity {
            p.span
        } else if let Some(cs) = constant {
            cs
        } else {
            fn_token.span
        }
        .merge(last_span);

        Ok(self.ast.items().push(Item::fn_dec(
            ident, publicity, constant, generics, params, ty, body, span,
        )))
    }

    fn parse_function_param(&mut self, idx: &mut usize) -> Result<FnParam, ()> {
        let arr = [self.peek(0), self.peek(1), self.peek(2)];

        // if *idx == 0 {
        //     match (arr[0].kind, arr[1].kind, arr[2].kind) {
        //         (TokenKind::Amp, TokenKind::KwMut, TokenKind::KwSelf) => {
        //             let ident = self.make_ident(&arr[2]);
        //
        //             let pat = self.ast.patterns().push(Pat::binding(ident, ident.span()));
        //             let inner = self.ast.types().push(Ty::self_(ident.span()));
        //
        //             let ty = self.ast.types().push(Ty::mutable_reference(
        //                 inner,
        //                 arr[0].span.merge(ident.span()),
        //             ));
        //
        //             self.advance(3);
        //
        //             *idx += 1;
        //
        //             return Ok(FnParam::new(pat, ty));
        //         }
        //
        //         (TokenKind::Amp, TokenKind::KwSelf, _) => {
        //             let ident = self.make_ident(&arr[1]);
        //
        //             let pat = self.ast.patterns().add(Pat::binding(ident));
        //             let inner = self.ast.types().add(Ty::self_(ident.span()));
        //
        //             let ty = self
        //                 .ast
        //                 .types()
        //                 .add(Ty::reference(inner, arr[0].span.merge(ident.span())));
        //
        //             self.advance(2);
        //
        //             *idx += 1;
        //
        //             return Ok(FnParam::new(pat, ty));
        //         }
        //
        //         (TokenKind::KwSelf, _, _) => {
        //             let ident = self.make_ident(&arr[0]);
        //
        //             let pat = self.ast.patterns().add(Pat::binding(ident));
        //             let ty = self.ast.types().add(Ty::self_(ident.span()));
        //
        //             self.advance(1);
        //
        //             *idx += 1;
        //
        //             return Ok(FnParam::new(pat, ty));
        //         }
        //
        //         _ => {}
        //     }
        // }

        // normal parameter
        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        // let pat = self.ast.patterns().push(Pat::binding(ident));

        let pat = self.ast.patterns().push(Pat::wildcard(ident.span()));

        self.expect(TokenKind::Colon)?;

        let ty = self.parse_type()?;

        *idx += 1;

        Ok(FnParam::new(pat, ty))
    }

    pub fn parse_impl_item(&mut self) -> Result<ItemID, ()> {
        let impl_token = self.consume();

        let generics = if self.peek(0).kind == TokenKind::LeftAngle {
            Some(self.parse_generic_params()?)
        } else {
            None
        };

        let first_ty = self.parse_type()?;

        let (trait_ty, target_ty) = if self.peek(0).kind == TokenKind::KwFor {
            self.advance(1);

            let target = self.parse_type()?;

            (Some(first_ty), target)
        } else {
            (None, first_ty)
        };

        let _ = self.expect(TokenKind::LeftCurly)?;

        let mut items = Vec::new();

        while self.peek(0).kind != TokenKind::RightCurly {
            items.push(self.parse_assoc_item()?);
        }

        let close = self.expect(TokenKind::RightCurly)?;

        let span = impl_token.span.merge(close.span);

        Ok(self
            .ast
            .items()
            .push(Item::impl_dec(target_ty, generics, trait_ty, items, span)))
    }

    fn parse_trait_item(&mut self, publicity: Option<Publicity>) -> Result<ItemID, ()> {
        let first = self.consume();

        let ident_token = self.expect(TokenKind::Ident)?;
        let ident = self.make_ident(&ident_token);

        let result = if self.peek(0).kind == TokenKind::Eq {
            self.advance(1);

            let items = self.parse_separated(
                Some((TokenKind::Plus, Trailing::Forbidden)),
                TokenKind::Semicolon,
                Self::parse_trait_bound,
            )?;

            if items.is_empty() {
                return Err(());
            }

            let semi = self.expect(TokenKind::Semicolon)?;

            let span = first.span.until(semi.span);

            Item::compound_trait_dec(ident, publicity, items, span)
        } else {
            let _ = self.expect(TokenKind::LeftCurly)?;

            let items = self.with_restrictions(Restrictions::IN_TRAIT, |p| {
                p.parse_separated(None, TokenKind::RightCurly, Self::parse_item)
            })?;

            let close = self.expect(TokenKind::RightCurly)?;

            let span = first.span.merge(close.span);

            Item::trait_dec(ident, publicity, items, span)
        };

        Ok(self.ast.items().push(result))
    }

    fn parse_trait_bound(&mut self) -> Result<Ident, ()> {
        let token = self.expect(TokenKind::Ident)?;
        Ok(self.make_ident(&token))
    }

    fn parse_assoc_item(&mut self) -> Result<ItemID, ()> {
        let publicity = self.parse_publicity()?;

        match (self.peek(0).kind, publicity) {
            (TokenKind::KwFn, _) => self.parse_fn_dec_item(publicity, None),

            (TokenKind::KwConst, _) => {
                if self.peek(1).kind == TokenKind::KwFn {
                    let const_span = self.consume().span;

                    self.parse_fn_dec_item(publicity, Some(const_span))
                } else {
                    self.parse_const_item(publicity)
                }
            }

            (TokenKind::KwType, _) => self.parse_type_dec_item(publicity),

            (_, Some(_)) => panic!("unexpected public keyword (not allowed here)"),

            _ => Err(()),
        }
    }
}
