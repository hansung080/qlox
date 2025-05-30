use thiserror::Error;
use qlox_macros::ResolveSnippet;
use crate::consts::tag::ERROR;
use crate::src::{Index, Snippet};
use crate::token::{Token, TokenKind};
use crate::utils::string::{Substring, SubstringError};

#[derive(Error, Debug, ResolveSnippet)]
pub enum Error {
    #[error("{ERROR}: unexpected char `{c}`\n\n{snippet}\n")]
    UnexpectedChar {
        snippet: Snippet,
        c: char,
    },

    #[error("{ERROR}: invalid utf-8\n\n{snippet}\n")]
    InvalidUtf8 {
        snippet: Snippet,
    },
}

impl From<SubstringError> for Error {
    fn from(error: SubstringError) -> Self {
        Error::InvalidUtf8 {
            snippet: Snippet::new(error.range.start + error.source.utf8_error().valid_up_to()),
        }
    }
}

pub struct Scanner<'a> {
    source: &'a [u8],
    next: Index,
    token_start: Index,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Scanner {
            source,
            next: 0,
            token_start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        while let Some(&c) = self.next() {
            self.token_start = self.next - 1;

            let token = self.scan_token(c)
                .and_then(|ok| ok.map(|k| {
                    Ok::<Token, Error>(Token {
                        kind: k,
                        lexeme: self.source.substring(self.token_start..self.next)?,
                        offset: self.token_start,
                    })
                }).transpose());

            match token {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => (),
                Err(e) => errors.push(e),
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            offset: self.next - 1,
        });

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn scan_token(&mut self, c: u8) -> Result<Option<TokenKind>, Error> {
        match c {
            b'(' => Ok(Some(TokenKind::LeftParen)),
            _ => Err(Error::UnexpectedChar {
                snippet: Snippet::new(self.token_start),
                c: c as char,
            }),
        }
    }

    fn next(&mut self) -> Option<&u8> {
        let c = self.source.get(self.next);
        self.next += 1;
        c
    }
}