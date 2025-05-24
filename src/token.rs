use std::fmt::{self, Display, Formatter};
use crate::types::Number;

#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    Number(Number),
    String(String),

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Others
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            LeftParen => f.write_str("("),
            RightParen => f.write_str(")"),
            LeftBrace => f.write_str("{"),
            RightBrace => f.write_str("}"),
            Comma => f.write_str(","),
            Dot => f.write_str("."),
            Minus => f.write_str("-"),
            Plus => f.write_str("+"),
            Semicolon => f.write_str(";"),
            Slash => f.write_str("/"),
            Star => f.write_str("*"),
            Bang => f.write_str("!"),
            BangEqual => f.write_str("!="),
            Equal => f.write_str("="),
            EqualEqual => f.write_str("=="),
            Greater => f.write_str(">"),
            GreaterEqual => f.write_str(">="),
            Less => f.write_str("<"),
            LessEqual => f.write_str("<="),
            Identifier(s) => f.write_str(s),
            Number(n) => n.fmt(f),
            String(s) => s.fmt(f),
            And => f.write_str("and"),
            Class => f.write_str("class"),
            Else => f.write_str("else"),
            False => f.write_str("false"),
            For => f.write_str("for"),
            Fun => f.write_str("fun"),
            If => f.write_str("if"),
            Nil => f.write_str("nil"),
            Or => f.write_str("or"),
            Print => f.write_str("print"),
            Return => f.write_str("return"),
            Super => f.write_str("super"),
            This => f.write_str("this"),
            True => f.write_str("true"),
            Var => f.write_str("var"),
            While => f.write_str("while"),
            Eof => f.write_str("\\d"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Token {
            kind,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}