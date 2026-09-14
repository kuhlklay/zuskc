use crate::{
    lexer::token::{KEYWORDS, Spacing, Token, TokenKind},
    source::{SourceFile, Span},
    utils::sync::Lrc,
};

pub mod token;

pub struct Lexer {
    pub file: Lrc<SourceFile>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(file: Lrc<SourceFile>) -> Self {
        Self { file, pos: 0 }
    }

    pub fn collect(&mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(self.file.content.len() / 6);

        loop {
            let token = self.next();
            let is_eof = token.kind == TokenKind::EOF;

            tokens.push(token);

            if is_eof {
                break;
            }
        }

        tokens
    }

    fn input(&self) -> &[u8] {
        self.file.content.as_bytes()
    }

    fn peek(&self, offset: usize) -> &u8 {
        self.input().get(self.pos + offset).unwrap_or(&b'\0')
    }

    fn advance(&mut self, offset: usize) {
        self.pos += offset;
    }

    fn tokenize(&mut self, kind: TokenKind, start: usize, len: usize, spacing: Spacing) -> Token {
        Token::new(kind, Span::new(start, len, self.file.id), spacing)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input().get(self.pos) {
            if c.is_ascii_whitespace() {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    fn advance_until(&mut self, pattern: &[u8]) {
        let pat_len = pattern.len();

        while self.pos + pat_len <= self.input().len() {
            if &self.input()[self.pos..self.pos + pat_len] == pattern {
                self.advance(pat_len);
                break;
            }
            self.advance(1);
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.pos;

        let (kind, len, spacing) = match self.peek(0) {
            0 => (TokenKind::EOF, 1, Spacing::Alone),

            b'+' => {
                let spacing = match self.peek(1) {
                    b'+' | b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Plus, 1, spacing)
            }

            b'-' => {
                let spacing = match self.peek(1) {
                    b'-' | b'=' | b'>' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Minus, 1, spacing)
            }

            b'*' => {
                let spacing = match self.peek(1) {
                    b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Asterisk, 1, spacing)
            }

            b'/' => {
                match self.peek(1) {
                    // Line comment
                    b'/' => {
                        self.advance(2);
                        self.advance_until(b"\n");
                        return self.next();
                    }

                    // Block comment
                    b'*' => {
                        self.advance(2);
                        self.advance_until(b"*/");
                        return self.next();
                    }

                    b'=' => (TokenKind::Slash, 1, Spacing::Joint),

                    _ => (TokenKind::Slash, 1, Spacing::Alone),
                }
            }

            b'%' => {
                let spacing = match self.peek(1) {
                    b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Percent, 1, spacing)
            }

            b'=' => {
                let spacing = match self.peek(1) {
                    b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Eq, 1, spacing)
            }

            b'&' => {
                let spacing = match self.peek(1) {
                    b'&' | b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Amp, 1, spacing)
            }

            b'|' => {
                let spacing = match self.peek(1) {
                    b'|' | b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Pipe, 1, spacing)
            }

            b'^' => {
                let spacing = match self.peek(1) {
                    b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Caret, 1, spacing)
            }

            b'~' => {
                let spacing = match self.peek(1) {
                    b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Tilde, 1, spacing)
            }

            b'(' => (TokenKind::LeftParen, 1, Spacing::Alone),
            b')' => (TokenKind::RightParen, 1, Spacing::Alone),

            b'[' => (TokenKind::LeftBracket, 1, Spacing::Alone),
            b']' => (TokenKind::RightBracket, 1, Spacing::Alone),

            b'{' => (TokenKind::LeftCurly, 1, Spacing::Alone),
            b'}' => (TokenKind::RightCurly, 1, Spacing::Alone),

            b'<' => {
                let spacing = match self.peek(1) {
                    b'<' | b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::LeftAngle, 1, spacing)
            }

            b'>' => {
                let spacing = match self.peek(1) {
                    b'>' | b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::RightAngle, 1, spacing)
            }

            b'!' => {
                let spacing = match self.peek(1) {
                    b'=' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Bang, 1, spacing)
            }

            b'?' => (TokenKind::Question, 1, Spacing::Alone),

            b'.' => {
                let spacing = match self.peek(1) {
                    b'.' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Dot, 1, spacing)
            }

            b',' => (TokenKind::Comma, 1, Spacing::Alone),

            b':' => {
                let spacing = match self.peek(1) {
                    b':' => Spacing::Joint,
                    _ => Spacing::Alone,
                };

                (TokenKind::Colon, 1, spacing)
            }

            b';' => (TokenKind::Semicolon, 1, Spacing::Alone),

            // b'"' => return self.lex_string(),
            // b'\'' => return self.lex_char(),

            // b'$' => (TokenKind::Dollar, 1),
            c if (*c as char).is_ascii_alphabetic() || *c == b'_' => return self.lex_ident(),

            c if (*c as char).is_numeric() => return self.lex_number(),

            _ => {
                panic!("unknown char");
            }
        };

        self.advance(len);
        self.tokenize(kind, start, len, spacing)
    }

    fn lex_ident(&mut self) -> Token {
        let start = self.pos;
        let mut len = 1;

        loop {
            let c = *self.peek(len);

            if !c.is_ascii_alphanumeric() && c != b'_' {
                break;
            }

            len += 1;
        }

        let text = &self.file.content[start..start + len];

        if let Some(&kind) = KEYWORDS.get(text) {
            self.advance(len);
            return self.tokenize(kind, start, len, Spacing::Alone);
        }

        self.advance(len);
        self.tokenize(TokenKind::Ident, start, len, Spacing::Alone)
    }

    fn lex_number(&mut self) -> Token {
        if *self.peek(0) == b'0' {
            match self.peek(1) {
                b'x' | b'X' => return self.read_radix_integer(16, 2),
                b'o' | b'O' => return self.read_radix_integer(8, 2),
                b'b' | b'B' => return self.read_radix_integer(2, 2),
                _ => {}
            }
        }

        self.read_radix_integer(10, 0)
    }

    #[inline]
    fn digit_value(c: u8) -> Option<u32> {
        match c {
            b'0'..=b'9' => Some((c - b'0') as u32),
            b'a'..=b'f' => Some((c - b'a' + 10) as u32),
            b'A'..=b'F' => Some((c - b'A' + 10) as u32),
            _ => None,
        }
    }

    fn read_radix_integer(&mut self, radix: u32, prefix_len: usize) -> Token {
        let start = self.pos;

        self.advance(prefix_len);

        let mut len = prefix_len;

        let mut value = 0u128;
        let mut saw_digit = false;
        let mut invalid_digit = false;
        let mut overflow = false;

        loop {
            let &c = self.peek(0);
            if c == b'_' {
                self.advance(1);
                len += 1;
                continue;
            }

            let Some(digit) = Self::digit_value(c) else {
                break;
            };

            if digit >= radix {
                invalid_digit = true;
            } else {
                saw_digit = true;

                match value
                    .checked_mul(radix as u128)
                    .and_then(|v| v.checked_add(digit as u128))
                {
                    Some(v) => value = v,
                    None => overflow = true,
                }
            }

            len += 1;
            self.advance(1);
        }

        let kind = if !saw_digit || invalid_digit || overflow {
            TokenKind::Err
        } else {
            TokenKind::Int
        };

        self.tokenize(kind, start, len, Spacing::Alone)
    }

    // fn lex_string(&mut self) -> Token {}
}
