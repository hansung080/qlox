use thiserror::Error;
use qlox_macros::ResolveSnippet;
use crate::consts::tag::ERROR;
use crate::src::{Index, Snippet};
use crate::token;
use crate::token::{Token, TokenKind};
use crate::types::Number;
use crate::utils::string::{Substring, SubstringError};

#[derive(Error, Debug, ResolveSnippet, PartialEq)]
pub enum Error {
    #[error("{ERROR}: unexpected char `{c}`\n\n{snippet}\n")]
    UnexpectedChar {
        snippet: Snippet,
        c: char,
    },

    #[error("{ERROR}: invalid utf-8 char\n\n{snippet}\n")]
    InvalidUtf8Char {
        snippet: Snippet,
    },

    #[error("{ERROR}: unterminated multi-line comment\n\n{snippet}\n")]
    UnterminatedMultiLineComment {
        snippet: Snippet,
    },

    #[error("{ERROR}: unterminated single quote string\n\n{snippet}\n")]
    UnterminatedSingleQuoteString {
        snippet: Snippet,
    },

    #[error("{ERROR}: unterminated double quote string\n\n{snippet}\n")]
    UnterminatedDoubleQuoteString {
        snippet: Snippet,
    },
}

impl From<SubstringError> for Error {
    fn from(error: SubstringError) -> Self {
        Error::InvalidUtf8Char {
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

        if errors.len() > 0 {
            return Err(errors);
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            offset: self.next - 1,
        });
        Ok(tokens)
    }

    fn scan_token(&mut self, c: u8) -> Result<Option<TokenKind>, Error> {
        match c {
            b' ' | b'\t' | b'\r' | b'\n' => Ok(None),
            b'(' => Ok(Some(TokenKind::LeftParen)),
            b')' => Ok(Some(TokenKind::RightParen)),
            b'{' => Ok(Some(TokenKind::LeftBrace)),
            b'}' => Ok(Some(TokenKind::RightBrace)),
            b',' => Ok(Some(TokenKind::Comma)),
            b'.' => Ok(Some(TokenKind::Dot)),
            b'-' => Ok(Some(TokenKind::Minus)),
            b'+' => Ok(Some(TokenKind::Plus)),
            b';' => Ok(Some(TokenKind::Semicolon)),
            b'/' => {
                if self.matches(b'/') {
                    self.scan_single_line_comment();
                    Ok(None)
                } else if self.matches(b'*') {
                    self.scan_multi_line_comment().map(|_| None)
                } else {
                    Ok(Some(TokenKind::Slash))
                }
            },
            b'*' => Ok(Some(TokenKind::Star)),
            b'!' => {
                Ok(Some(if self.matches(b'=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }))
            },
            b'=' => {
                Ok(Some(if self.matches(b'=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }))
            },
            b'>' => {
                Ok(Some(if self.matches(b'=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }))
            },
            b'<' => {
                Ok(Some(if self.matches(b'=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }))
            },
            b'\'' | b'"' => {
                self.scan_string(c).map(Some)
            },
            _ if c.is_ascii_digit() => {
                self.scan_number().map(Some)
            },
            _ if c.is_ascii_alphabetic() || c == b'_' => {
                self.scan_identifier().map(Some)
            },
            _ => {
                Err(Error::UnexpectedChar {
                    snippet: Snippet::new(self.token_start),
                    c: c as char,
                })
            },
        }
    }

    fn scan_single_line_comment(&mut self) {
        while let Some(&c) = self.peek() {
            if c == b'\n' { break; }
            self.go_next();
        }
    }

    fn scan_multi_line_comment(&mut self) -> Result<(), Error> {
        while let (Some(&c), Some(&d)) = (self.peek(), self.peek_next()) {
            if c == b'*' && d == b'/' { break; }
            self.go_next();
        }

        if let None = self.peek_next() {
            Err(Error::UnterminatedMultiLineComment {
                snippet: Snippet::new(self.token_start),
            })
        } else {
            self.go_step(2);
            Ok(())
        }
    }

    fn scan_string(&mut self, quote: u8) -> Result<TokenKind, Error> {
        while let Some(&c) = self.peek() {
            if c == quote { break; }
            self.go_next();
        }

        if let None = self.peek() {
            if quote == b'\'' {
                Err(Error::UnterminatedSingleQuoteString {
                    snippet: Snippet::new(self.token_start),
                })
            } else {
                Err(Error::UnterminatedDoubleQuoteString {
                    snippet: Snippet::new(self.token_start),
                })
            }
        } else {
            self.go_next();
            Ok(TokenKind::String(
                self.source.substring(self.token_start + 1..self.next - 1)?
            ))
        }
    }

    fn scan_number(&mut self) -> Result<TokenKind, Error> {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() { break; }
            self.go_next();
        }

        if let (Some(&c), Some(d)) = (self.peek(), self.peek_next()) {
            if c == b'.' && d.is_ascii_digit() {
                self.go_step(2);
                while let Some(c) = self.peek() {
                    if !c.is_ascii_digit() { break; }
                    self.go_next();
                }
            }
        }

        Ok(TokenKind::Number(
            self.source.substring(self.token_start..self.next)?
                .parse::<Number>()
                .unwrap()
        ))
    }

    fn scan_identifier(&mut self) -> Result<TokenKind, Error> {
        while let Some(&c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == b'_') { break; }
            self.go_next();
        }

        let s = self.source.substring(self.token_start..self.next)?;
        match token::KEYWORDS.get(s.as_str()) {
            Some(kind) => Ok(kind.clone()),
            None => Ok(TokenKind::Identifier(s)),
        }
    }

    #[inline(always)]
    fn next(&mut self) -> Option<&u8> {
        let c = self.source.get(self.next);
        self.next += 1;
        c
    }

    #[inline(always)]
    fn matches(&mut self, expected: u8) -> bool {
        if let Some(&c) = self.source.get(self.next) {
            if c == expected {
                self.next += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    #[inline(always)]
    fn peek(&self) -> Option<&u8> {
        self.source.get(self.next)
    }

    #[inline(always)]
    fn peek_next(&self) -> Option<&u8> {
        self.source.get(self.next + 1)
    }

    #[inline(always)]
    fn go_next(&mut self) {
        self.next += 1;
    }

    #[inline(always)]
    fn go_step(&mut self, step: Index) {
        self.next += step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_token(kind: TokenKind, lexeme: &str, offset: Index) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            offset,
        }
    }

    #[test]
    fn scanner_scan_tokens() {
        use crate::token::TokenKind::*;

        let source = b" \t\r\n(){},.-+;//xxx\n/*xxx*// *!!== ==>>=<<='abc'\"def\"1230 456.789id and class else false for fun if nil or print return super this true var while";
        let tokens = Scanner::new(source).scan_tokens().unwrap();
        let expected = vec![
            new_token(LeftParen,                    "(",       4),
            new_token(RightParen,                   ")",       5),
            new_token(LeftBrace,                    "{",       6),
            new_token(RightBrace,                   "}",       7),
            new_token(Comma,                        ",",       8),
            new_token(Dot,                          ".",       9),
            new_token(Minus,                        "-",       10),
            new_token(Plus,                         "+",       11),
            new_token(Semicolon,                    ";",       12),
            new_token(Slash,                        "/",       26),
            new_token(Star,                         "*",       28),
            new_token(Bang,                         "!",       29),
            new_token(BangEqual,                    "!=",      30),
            new_token(Equal,                        "=",       32),
            new_token(EqualEqual,                   "==",      34),
            new_token(Greater,                      ">",       36),
            new_token(GreaterEqual,                 ">=",      37),
            new_token(Less,                         "<",       39),
            new_token(LessEqual,                    "<=",      40),
            new_token(String("abc".to_string()),    "'abc'",   42),
            new_token(String("def".to_string()),    "\"def\"", 47),
            new_token(Number(1230_f64),             "1230",    52),
            new_token(Number(456.789),              "456.789", 57),
            new_token(Identifier("id".to_string()), "id",      64),
            new_token(And,                          "and",     67),
            new_token(Class,                        "class",   71),
            new_token(Else,                         "else",    77),
            new_token(False,                        "false",   82),
            new_token(For,                          "for",     88),
            new_token(Fun,                          "fun",     92),
            new_token(If,                           "if",      96),
            new_token(Nil,                          "nil",     99),
            new_token(Or,                           "or",      103),
            new_token(Print,                        "print",   106),
            new_token(Return,                       "return",  112),
            new_token(Super,                        "super",   119),
            new_token(This,                         "this",    125),
            new_token(True,                         "true",    130),
            new_token(Var,                          "var",     135),
            new_token(While,                        "while",   139),
            new_token(Eof,                          "",        144),
        ];

        assert_eq!(tokens.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(tokens[i], expected[i]);
        }
    }

    #[test]
    fn scanner_scan_tokens_error() {
        let source = b"  &";
        let errors = Scanner::new(source).scan_tokens().err().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], Error::UnexpectedChar {
            snippet: Snippet::new(2),
            c: '&',
        });

        let mut source = "  'ab가cd'".to_string().into_bytes();
        source[7] += 100;
        let errors = Scanner::new(&source).scan_tokens().err().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], Error::InvalidUtf8Char {
            snippet: Snippet::new(5),
        });

        let source = b"  /*a";
        let errors = Scanner::new(source).scan_tokens().err().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], Error::UnterminatedMultiLineComment {
            snippet: Snippet::new(2),
        });

        let source = b"  'a";
        let errors = Scanner::new(source).scan_tokens().err().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], Error::UnterminatedSingleQuoteString {
            snippet: Snippet::new(2),
        });

        let source = b"  \"a";
        let errors = Scanner::new(source).scan_tokens().err().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], Error::UnterminatedDoubleQuoteString {
            snippet: Snippet::new(2),
        });
    }
}