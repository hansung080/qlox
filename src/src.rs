use std::fmt::{self, Display, Formatter};
use std::iter;
use std::ops::RangeInclusive;
use text_colorizer::Colorize;
use crate::utils::string::Substring;

pub type Index = usize;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub line: Index,
    pub column: Index,
}

#[derive(Debug, PartialEq)]
pub enum Location {
    Created {
        offset: Index,
    },
    Resolved {
        pos: Position,
        line: RangeInclusive<Index>,
    },
}

impl Location {
    pub fn new(offset: Index) -> Self {
        Location::Created { offset }
    }

    pub fn resolve(&mut self, source: &[u8]) {
        if let Location::Created { offset } = self {
            let mut line = 0;
            let mut line_start = 0;
            for i in 0..*offset  {
                if source[i] == b'\n' {
                    line += 1;
                    line_start = i + 1;
                }
            }

            let mut line_end = source.len() - 1;
            for i in *offset..source.len() {
                if source[i] == b'\r' {
                    line_end = i - 1;
                    break;
                } else if source[i] == b'\n' {
                    if source[i - 1] == b'\r' {
                        line_end = i - 2;
                    } else {
                        line_end = i - 1;
                    }
                    break;
                }
            }

            *self = Location::Resolved {
                pos: Position { line, column: *offset - line_start },
                line: line_start..=line_end,
            };
        }
    }

    pub fn snippet(&self, source: &[u8]) -> Option<String> {
        match self {
            Location::Created { .. } => None,
            Location::Resolved { pos, line } => {
                let pos_line = pos.line + 1;
                let code = source.substring_lossy(line.clone());
                let spaces = iter::repeat(' ')
                    .take(pos_line.to_string().len() + 3 + pos.column)
                    .collect::<String>();
                Some(format!("{} | {code}\n{spaces}{}", pos_line, "^".red().bold()))
            },
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Location::Created { offset } => {
                write!(f, "byte {}", offset + 1)
            },
            Location::Resolved { pos, .. } => {
                write!(f, "line {}:{}", pos.line + 1, pos.column + 1)
            },
        }
    }
}

#[derive(Debug)]
pub struct Snippet {
    offset: Index,
    code: Option<String>,
}

impl Snippet {
    pub fn new(offset: Index) -> Self {
        Snippet {
            offset,
            code: None,
        }
    }

    pub fn resolve(&mut self, source: &[u8]) {
        if let None = self.code {
            let mut loc = Location::new(self.offset);
            loc.resolve(source);
            self.code = loc.snippet(source);
        }
    }
}

impl Display for Snippet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(code) = &self.code {
            write!(f, "{code}")
        } else {
            Ok(())
        }
    }
}

pub trait ResolveSnippet {
    fn resolve_snippet(&mut self, source: &[u8]);
}

impl<T: ResolveSnippet> ResolveSnippet for Vec<T> {
    fn resolve_snippet(&mut self, source: &[u8]) {
        for t in self {
            t.resolve_snippet(source);
        }
    }
}

pub struct SnippetResolver<'a> {
    source: &'a [u8],
}

impl<'a> SnippetResolver<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        SnippetResolver { source }
    }

    pub fn resolve<T: ResolveSnippet>(&self, mut t: T) -> T {
        t.resolve_snippet(self.source);
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn src_loc_resolve() {
        let source = String::from("Alice\nBob\nChris\nDaniel\r\nEric").into_bytes();
        let cases = [
            (0, 0, 0, 0, 4),
            (2, 0, 2, 0, 4),
            (4, 0, 4, 0, 4),
            (5, 0, 5, 0, 4),
            (10, 2, 0, 10, 14),
            (12, 2, 2, 10, 14),
            (14, 2, 4, 10, 14),
            (15, 2, 5, 10, 14),
            (16, 3, 0, 16, 21),
            (18, 3, 2, 16, 21),
            (21, 3, 5, 16, 21),
            (22, 3, 6, 16, 21),
            (23, 3, 7, 16, 21),
            (24, 4, 0, 24, 27),
            (26, 4, 2, 24, 27),
            (27, 4, 3, 24, 27),
        ];

        for (offset, line, column, line_start, line_end) in cases {
            let mut loc = Location::new(offset);
            loc.resolve(&source);
            let expected = Location::Resolved {
                pos: Position { line, column },
                line: line_start..=line_end,
            };
            assert_eq!(loc, expected);
        }
    }
}