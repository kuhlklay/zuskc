use crate::{
    ast::{ExprID, StmtID, expr::Expr},
    lexer::token::TokenKind,
    parser::{Parser, Trailing},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self) -> Result<ExprID, ()> {
        self.parse_binary_expr(0)
    }

    // ---- Primitive Expressions ----

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<ExprID, ()> {
        let mut left = self.parse_cast_expr()?;

        loop {
            let Some(op) = self.parse_binary_operator() else {
                break;
            };

            let cur_prec = op.kind.precedence();

            if cur_prec < min_prec {
                break;
            }

            self.advance(op.span.len());

            let right = self.parse_binary_expr(cur_prec + 1)?;

            let left_span = self.ast.exprs()[left.index()].span;
            let right_span = self.ast.exprs()[right.index()].span;

            let span = left_span.merge(right_span);

            left = self.ast.exprs().push(Expr::binary(left, right, op, span));
        }

        Ok(left)
    }

    fn parse_cast_expr(&mut self) -> Result<ExprID, ()> {
        let mut expr = self.parse_prefix_expr()?;

        while self.peek(0).kind == TokenKind::KwAs {
            self.consume(); // 'as'

            let ty = self.parse_type()?;
            let span = self.ast.exprs()[expr.index()]
                .span
                .merge(self.ast.types()[ty.index()].span);

            expr = self.ast.exprs().push(Expr::cast(expr, ty, span));
        }

        Ok(expr)
    }

    fn parse_postfix_expr(&mut self) -> Result<ExprID, ()> {
        let mut expr = self.parse_primary_expr()?;

        loop {
            let arr = [*self.peek(0), *self.peek(1)];

            let mut span = arr[0].span;

            match arr[0].kind {
                TokenKind::LeftParen => {
                    self.advance(1);

                    let args = self.parse_separated(
                        Some((TokenKind::Comma, Trailing::Allowed)),
                        TokenKind::RightParen,
                        Self::parse_expr,
                    )?;

                    let close = self.expect(TokenKind::RightParen)?;

                    span = self.ast.exprs()[expr.index()].span.merge(close.span);

                    expr = self.ast.exprs().push(Expr::call(expr, args, None, span))
                }

                TokenKind::LeftBracket => {
                    self.advance(1);

                    let index = self.parse_expr()?;

                    let close = self.expect(TokenKind::RightBracket)?;

                    span = span.merge(close.span);

                    expr = self.ast.exprs().push(Expr::index(index, span));
                }

                TokenKind::Dot => {
                    self.advance(1);

                    let field = self.parse_expr()?;

                    span = span.merge(self.ast.exprs()[field.index()].span);

                    expr = self.ast.exprs().push(Expr::field(expr, field, span));
                }

                TokenKind::Colon if arr[0].is_joint() && arr[1].kind == TokenKind::Colon => {
                    self.advance(2);

                    let open = self.expect(TokenKind::LeftAngle)?;

                    let generics = Some(self.parse_generic_args()?);

                    let _ = self.expect(TokenKind::RightAngle)?;

                    let _ = self.expect(TokenKind::LeftParen)?;

                    let args = self.parse_separated(
                        Some((TokenKind::Comma, Trailing::Allowed)),
                        TokenKind::RightParen,
                        Self::parse_expr,
                    )?;

                    let close = self.expect(TokenKind::RightParen)?;

                    let span = open.span.merge(close.span);

                    expr = self
                        .ast
                        .exprs()
                        .push(Expr::call(expr, args, generics, span));
                }

                _ => {
                    let Some(op) = self.parse_postfix_operator() else {
                        break;
                    };

                    self.advance(op.span.len());

                    let expr_span = self.ast.exprs()[expr.index()].span;
                    let op_span = op.span;

                    expr = self
                        .ast
                        .exprs()
                        .push(Expr::postfix(expr, op, expr_span.merge(op_span)));
                }
            }
        }

        Ok(expr)
    }

    pub(super) fn parse_prefix_expr(&mut self) -> Result<ExprID, ()> {
        if let Some(op) = self.parse_prefix_operator() {
            self.advance(op.span.len());

            let expr = self.parse_prefix_expr()?;

            let span = op.span.merge(self.ast.exprs()[expr.index()].span);

            return Ok(self.ast.exprs().push(Expr::prefix(expr, op, span)));
        }

        self.parse_postfix_expr()
    }

    fn parse_primary_expr(&mut self) -> Result<ExprID, ()> {
        let current = *self.peek(0);

        match current.kind {
            TokenKind::Ident | TokenKind::KwSelf => {
                self.advance(1);
                let ident = self.make_ident(&current);

                Ok(self.ast.exprs().push(Expr::variable(ident)))
            }

            TokenKind::Int => {
                self.advance(1);

                Ok(self.ast.exprs().push(Expr::int(current.span)))
            }

            TokenKind::Float => {
                self.advance(1);

                Ok(self.ast.exprs().push(Expr::float(current.span)))
            }

            TokenKind::Char => {
                self.advance(1);

                Ok(self.ast.exprs().push(Expr::char(current.span)))
            }

            TokenKind::Str => {
                self.advance(1);

                Ok(self.ast.exprs().push(Expr::str(current.span)))
            }

            TokenKind::KwTrue => {
                self.advance(1);

                Ok(self.ast.exprs().push(Expr::bool(true, current.span)))
            }

            TokenKind::KwFalse => {
                self.advance(1);

                Ok(self.ast.exprs().push(Expr::bool(false, current.span)))
            }

            TokenKind::LeftCurly => self.parse_block_expr(),

            _ => Err(()),
        }
    }

    // ---- Advanced Expressions ----

    pub(super) fn parse_block_expr(&mut self) -> Result<ExprID, ()> {
        let open_curly = self.expect(TokenKind::LeftCurly)?;

        let mut stmts: Vec<StmtID> = Vec::new();

        while self.peek(0).kind != TokenKind::RightCurly {
            stmts.push(self.parse_stmt()?);
        }

        let close_curly = self.expect(TokenKind::RightCurly)?;

        let span = open_curly.span.merge(close_curly.span);

        Ok(self.ast.exprs().push(Expr::block(stmts, span)))
    }

    pub(super) fn parse_if_expr(&mut self) -> Result<ExprID, ()> {
        let if_token = self.consume();

        let condition = self.parse_expr()?;

        let body = self.parse_block_expr()?;

        let branch = if self.peek(0).kind == TokenKind::KwElse {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let span = if_token.span.merge(
            self.ast.exprs()[if let Some(branch) = branch {
                branch
            } else {
                body
            }
            .index()]
            .span,
        );

        Ok(self
            .ast
            .exprs()
            .push(Expr::if_(condition, body, branch, span)))
    }
}
