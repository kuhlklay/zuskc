use crate::{
    ast::{PatID, patterns::Pat},
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_pattern(&mut self) -> Result<PatID, ()> {
        let token = self.peek(0);

        match token.kind {
            // ...
            _ => match self.parse_prefix_expr() {
                Ok(e) => {
                    let span = self.ast.exprs()[e.index()].span;
                    Ok(self.ast.patterns().push(Pat::expr(e, span)))
                }
                Err(_) => Err(()),
            },
        }
    }

    // fn parse_pattern_field(&mut self) -> Result<PatField, ()> {}
}
