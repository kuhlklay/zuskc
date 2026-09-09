use crate::{
    ast::{
        ExprID, TyID,
        types::{GenericArgs, GenericParam, Ty},
    },
    lexer::token::TokenKind,
    parser::{Parser, Trailing},
};

impl<'a> Parser<'a> {
    pub fn parse_type(&mut self) -> Result<TyID, ()> {
        let mut span = self.peek(0).span;

        match self.peek(0).kind {
            TokenKind::LeftBracket => {
                self.advance(1);

                let inner = self.parse_type()?;
                let mut size_expr: Option<ExprID> = None;

                if self.peek(0).kind == TokenKind::Semicolon {
                    self.advance(1);

                    size_expr = Some(self.parse_expr()?);
                }

                let close = self.expect(TokenKind::RightBracket)?;

                span = span.merge(close.span);

                if let Some(size) = size_expr {
                    return Ok(self.ast.types().push(Ty::array(inner, size, span)));
                }

                Ok(self.ast.types().push(Ty::slice(inner, span)))
            }

            TokenKind::LeftParen => {
                self.advance(1);

                let inner = if self.peek(0).kind == TokenKind::RightParen {
                    Vec::new()
                } else {
                    self.parse_separated(
                        Some((TokenKind::Comma, Trailing::Allowed)),
                        TokenKind::RightParen,
                        Self::parse_type,
                    )?
                };

                let close = self.expect(TokenKind::RightParen)?;

                span = span.merge(close.span);

                Ok(self.ast.types().push(if inner.is_empty() {
                    Ty::unit(span)
                } else {
                    Ty::tuple(inner, span)
                }))
            }

            TokenKind::Amp => {
                let amp = self.consume();

                let inner = self.parse_type()?;

                let span = amp.span.merge(self.ast.types()[inner.index()].span);

                Ok(self.ast.types().push(Ty::reference(inner, span)))
            }

            TokenKind::Asterisk => {
                let asterisk = self.consume();

                let inner = self.parse_type()?;

                let span = asterisk.span.merge(self.ast.types()[inner.index()].span);

                Ok(self.ast.types().push(Ty::pointer(inner, span)))
            }

            TokenKind::KwSelf => {
                let span = self.consume().span;

                Ok(self.ast.types().push(Ty::self_(span)))
            }

            TokenKind::Ident => {
                let token = self.expect(TokenKind::Ident)?;
                let ident = self.make_ident(&token);

                let args = if self.peek(0).kind == TokenKind::LeftAngle {
                    Some(self.parse_generic_args()?)
                } else {
                    None
                };

                let span = match &args {
                    Some(args) => token.span.merge(args.span),
                    None => token.span,
                };

                Ok(self.ast.types().push(Ty::named(ident, args, span)))
            }

            _ => Err(()),
        }
    }

    // ---- Generic Arguments ----

    pub fn parse_generic_args(&mut self) -> Result<GenericArgs, ()> {
        let open = self.expect(TokenKind::LeftAngle)?;

        let args = if self.peek(0).kind == TokenKind::RightAngle {
            Vec::with_capacity(0)
        } else {
            self.parse_separated(
                Some((TokenKind::Comma, Trailing::Allowed)),
                TokenKind::RightAngle,
                Self::parse_type,
            )?
        };

        let close = self.expect(TokenKind::RightAngle)?;

        let span = open.span.merge(close.span);

        Ok(GenericArgs { args, span })
    }

    // ---- Generic Parameters ----

    pub fn parse_generic_params(&mut self) -> Result<Vec<GenericParam>, ()> {
        let _ = self.expect(TokenKind::LeftAngle)?;

        let params = if self.peek(0).kind == TokenKind::RightAngle {
            Vec::with_capacity(0)
        } else {
            self.parse_separated(
                Some((TokenKind::Comma, Trailing::Allowed)),
                TokenKind::RightAngle,
                Self::single_generic_param,
            )?
        };

        let _ = self.expect(TokenKind::RightAngle)?;

        Ok(params)
    }

    fn single_generic_param(&mut self) -> Result<GenericParam, ()> {
        let token = self.expect(TokenKind::Ident)?;
        let name = self.make_ident(&token);

        let mut bounds = Vec::new();

        if self.peek(0).kind == TokenKind::Colon {
            self.advance(1);

            bounds.push(self.parse_type()?);

            while self.peek(0).kind == TokenKind::Plus {
                self.advance(1);
                bounds.push(self.parse_type()?);
            }
        }

        let default = if self.peek(0).kind == TokenKind::Eq {
            self.advance(1);
            Some(self.parse_type()?)
        } else {
            None
        };

        let end_span = default
            .map(|t| self.ast.types()[t.index()].span)
            .or_else(|| bounds.last().map(|t| self.ast.types()[t.index()].span))
            .unwrap_or(token.span);

        Ok(GenericParam {
            name,
            bounds,
            default,
            span: token.span.merge(end_span),
        })
    }
}
