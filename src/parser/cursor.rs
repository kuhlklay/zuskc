use crate::lexer::token::{Token, TokenKind};

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        debug_assert!(matches!(
            tokens.last().map(|t| t.kind),
            Some(TokenKind::EOF)
        ));
        Self { tokens }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self::new(tokens)
    }

    fn get(&self, i: usize) -> Option<&Token> {
        self.tokens.get(i)
    }

    fn last(&self) -> &Token {
        self.tokens.last().expect("TokenStream needs EOF sentinel")
    }
}

pub struct TokenCursor {
    stream: TokenStream,
    pos: usize,
}

impl TokenCursor {
    pub fn new(stream: TokenStream) -> Self {
        Self { stream, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn peek(&self, n: usize) -> &Token {
        self.stream
            .get(self.pos + n)
            .unwrap_or_else(|| self.stream.last())
    }

    pub fn bump(&mut self) -> Token {
        let token = *self.peek(0);
        if token.kind != TokenKind::EOF {
            self.pos += 1;
        }
        token
    }

    pub fn at_eof(&self) -> bool {
        self.peek(0).kind == TokenKind::EOF
    }

    pub fn checkpoint(&self) -> usize {
        self.pos
    }

    pub fn rollback(&mut self, cp: usize) {
        debug_assert!(cp <= self.pos);
        self.pos = cp;
    }
}
