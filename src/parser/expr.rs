use crate::{
    ast::{ExprID, StmtID, expr::Expr},
    lexer::token::TokenKind,
    parser::{Parser, Trailing},
};

impl<'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> Result<ExprID, ()> {
        self.parse_binary_expr(0)
    }

    // ---- Primitive Expressions ----

    pub fn parse_binary_expr(&mut self, min_prec: u8) -> Result<ExprID, ()> {
        let mut left = self.parse_cast_expr()?;

        loop {
            let Some(op) = self.parse_binary_operator() else {
                break;
            };

            let cur_prec = op.kind.precedence();

            if cur_prec < min_prec {
                break;
            }

            self.advance(op.span.len);

            let right = self.parse_binary_expr(cur_prec + 1)?;

            let left_ref = self.ast.expr(left);
            let right_ref = self.ast.expr(right);

            let span = left_ref.span.merge(right_ref.span);

            left = self.ast.add_expr(Expr::binary(left, right, op, span));
        }

        Ok(left)
    }

    pub fn parse_cast_expr(&mut self) -> Result<ExprID, ()> {
        let mut expr = self.parse_prefix_expr()?;

        while self.peek(0).kind == TokenKind::KwAs {
            self.consume(); // 'as'

            let ty = self.parse_type()?;
            let span = self.ast.expr(expr).span.merge(self.ast.ty(ty).span);

            expr = self.ast.add_expr(Expr::cast(expr, ty, span));
        }

        Ok(expr)
    }

    pub fn parse_postfix_expr(&mut self) -> Result<ExprID, ()> {
        let mut expr = self.parse_primary_expr()?;

        loop {
            let arr = [self.peek(0), self.peek(1)];

            let mut span = arr[0].span;

            match arr[0].kind {
                TokenKind::LeftParen => {
                    self.advance(1);

                    let args = self.parse_separated(
                        TokenKind::Comma,
                        TokenKind::RightParen,
                        Trailing::Allowed,
                        Self::parse_expr,
                    )?;

                    let close = self.expect(TokenKind::RightParen)?;

                    span = self.ast.expr(expr).span.merge(close.span);

                    expr = self.ast.add_expr(Expr::call(expr, args, Vec::new(), span))
                }

                TokenKind::LeftBracket => {
                    self.advance(1);

                    let index = self.parse_expr()?;

                    let close = self.expect(TokenKind::RightBracket)?;

                    span = span.merge(close.span);

                    expr = self.ast.add_expr(Expr::index(index, span));
                }

                TokenKind::Dot => {
                    self.advance(1);

                    let field = self.parse_expr()?;

                    span = span.merge(self.ast.expr(field).span);

                    expr = self.ast.add_expr(Expr::field(expr, field, span));
                }

                TokenKind::Colon if arr[0].is_joint() && arr[1].kind == TokenKind::Colon => {
                    self.advance(2);

                    let open = self.expect(TokenKind::LeftAngle)?;

                    let generics = self.parse_separated(
                        TokenKind::Comma,
                        TokenKind::RightAngle,
                        Trailing::Allowed,
                        Self::parse_type,
                    )?;

                    let _ = self.expect(TokenKind::RightAngle)?;

                    let _ = self.expect(TokenKind::LeftParen)?;

                    let args = self.parse_separated(
                        TokenKind::Comma,
                        TokenKind::RightParen,
                        Trailing::Allowed,
                        Self::parse_expr,
                    )?;

                    let close = self.expect(TokenKind::RightParen)?;

                    let span = open.span.merge(close.span);

                    expr = self.ast.add_expr(Expr::call(expr, args, generics, span));
                }

                _ => {
                    let Some(op) = self.parse_postfix_operator() else {
                        break;
                    };

                    self.advance(op.span.len);

                    let expr_ref = self.ast.expr(expr);

                    expr = self
                        .ast
                        .add_expr(Expr::postfix(expr, op, expr_ref.span.merge(op.span)));
                }
            }
        }

        Ok(expr)
    }

    pub fn parse_prefix_expr(&mut self) -> Result<ExprID, ()> {
        if let Some(op) = self.parse_prefix_operator() {
            self.advance(op.span.len);

            let expr = self.parse_prefix_expr()?;

            let span = op.span.merge(self.ast.expr(expr).span);

            return Ok(self.ast.add_expr(Expr::prefix(expr, op, span)));
        }

        self.parse_postfix_expr()
    }

    pub fn parse_primary_expr(&mut self) -> Result<ExprID, ()> {
        let current = self.peek(0);

        match current.kind {
            TokenKind::Ident => {
                self.advance(1);
                let ident = self.make_ident(&current);

                Ok(self.ast.add_expr(Expr::variable(ident)))
            }

            TokenKind::Int => {
                self.advance(1);

                Ok(self.ast.add_expr(Expr::int(current.span)))
            }

            TokenKind::Float => {
                self.advance(1);

                Ok(self.ast.add_expr(Expr::float(current.span)))
            }

            TokenKind::Char => {
                self.advance(1);

                Ok(self.ast.add_expr(Expr::char(current.span)))
            }

            TokenKind::String => {
                self.advance(1);

                Ok(self.ast.add_expr(Expr::str(current.span)))
            }

            TokenKind::LeftCurly => self.parse_block_expr(),

            _ => Err(()),
        }
    }

    // ---- Advanced Expressions ----

    pub fn parse_block_expr(&mut self) -> Result<ExprID, ()> {
        let open_curly = self.expect(TokenKind::LeftCurly)?;

        let mut stmts: Vec<StmtID> = Vec::new();

        while self.peek(0).kind != TokenKind::RightCurly {
            stmts.push(self.parse_stmt()?);
        }

        let close_curly = self.expect(TokenKind::RightCurly)?;

        let span = open_curly.span.merge(close_curly.span);

        Ok(self.ast.add_expr(Expr::block(stmts, span)))
    }

    pub fn parse_if_expr(&mut self) -> Result<ExprID, ()> {
        let if_token = self.consume();

        let condition = self.parse_expr()?;

        let body = self.parse_block_expr()?;

        let branch = if self.peek(0).kind == TokenKind::KwElse {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let span = if_token.span.merge(
            self.ast
                .expr(if let Some(branch) = branch {
                    branch
                } else {
                    body
                })
                .span,
        );

        Ok(self.ast.add_expr(Expr::if_(condition, body, branch, span)))
    }
}
