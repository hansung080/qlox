use thiserror::Error;
use qlox_macros::ResolveError;
use crate::token::{Token, TokenKind};
use crate::loc::SrcLoc;

#[derive(Error, Debug, ResolveError)]
pub enum Error {
    #[error("unexpected char `{c}` at {loc:?}")]
    UnexpectedChar {
        loc: SrcLoc,
        c: char,
    },
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_ended() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenKind::Eof, String::new(), self.line));
        &self.tokens
    }

    fn scan_token(&self) {
        todo!()
    }

    fn is_ended(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&self) -> char {
        todo!()
    }
}