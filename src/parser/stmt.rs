use crate::{
    ast::{Mutability, StmtID, stmt::Stmt},
    lexer::token::TokenKind,
    parser::{Parser, restrictions::Restrictions},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_stmt(&mut self) -> Result<StmtID, ()> {
        match self.peek(0).kind {
            TokenKind::KwDec => self.parse_dec_stmt(),

            TokenKind::KwIf => {
                let expr = self.parse_if_expr()?;

                let span = self.ast.exprs()[expr.index()].span;

                Ok(self.ast.stmts().push(Stmt::expr(expr, span)))
            }

            _ => self.with_restrictions(Restrictions::STMT_EXPR, Self::parse_expr_stmt),
        }
    }

    pub fn parse_dec_stmt(&mut self) -> Result<StmtID, ()> {
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

        let span = dec_token
            .span
            .merge(self.ast.exprs().get(init).unwrap().span);

        Ok(self
            .ast
            .stmts()
            .push(Stmt::var_dec(ident, mutable, ty, init, span)))
    }

    pub fn parse_expr_stmt(&mut self) -> Result<StmtID, ()> {
        let expr = self.parse_expr()?;
        let span = self.ast.exprs()[expr.index()].span;

        Ok(self.ast.stmts().push(Stmt::expr(expr, span)))
    }
}
