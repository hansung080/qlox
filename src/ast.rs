use std::fmt::{self, Display, Formatter};
use paste::paste;
use crate::token::Token;
use crate::types::Number;

pub trait Accept<V, C, R> {
    fn accept(&self, visitor: V, context: C) -> R;
}

macro_rules! ast {
    ($(pub enum $base:ident { $($name:ident: $typ:ident $body:tt),* $(,)? })*) => { paste! {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub enum $base {
                $($name($name)),*
            }

            $(
                #[derive(Debug, Clone, PartialEq)]
                pub $typ $name $body
            )*

            pub trait [<$base Visitor>]<C, R> {
                $(
                    fn [<visit_ $name:snake>](self, [<$base:snake>]: &$name, context: C) -> R;
                )*
            }

            impl<C, R, V: [<$base Visitor>]<C, R>> Accept<V, C, R> for $base {
                fn accept(&self, visitor: V, context: C) -> R {
                    match self {
                        $(
                            $base::$name(x) => visitor.[<visit_ $name:snake>](x, context)
                        ),*
                    }
                }
            }
        )*
    } };
}

ast! {
    pub enum Expr {
        Binary: struct {
            pub left: Box<Expr>,
            pub operator: Token,
            pub right: Box<Expr>,
        },
        Grouping: struct {
            pub expr: Box<Expr>,
        },
        Literal: enum {
            Number(Number),
            String(String),
            True,
            False,
            Nil,
        },
        Unary: struct {
            pub operator: Token,
            pub right: Box<Expr>,
        },
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(Binary { left, operator, right }) => {
                f.write_fmt(format_args!("({} {} {})", operator, left, right))
            },
            Expr::Grouping(Grouping { expr }) => {
                f.write_fmt(format_args!("(group {})", expr))
            },
            Expr::Literal(Literal::Number(n)) => n.fmt(f),
            Expr::Literal(Literal::String(s)) => s.fmt(f),
            Expr::Literal(Literal::True) => f.write_str("true"),
            Expr::Literal(Literal::False) => f.write_str("false"),
            Expr::Literal(Literal::Nil) => f.write_str("nil"),
            Expr::Unary(Unary { operator, right }) => {
                f.write_fmt(format_args!("({} {})", operator, right))
            },
        }
    }
}
