use std::rc::Rc;

use crate::text::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Procent,
    Carrot,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Bar,

    X,
    Y,
    Number(f64),

    Comma,

    Whitespace,

    Other(char),
}

impl TokenKind {
    pub fn as_usize(&self) -> usize {
        match self {
            TokenKind::X => 0,
            TokenKind::Y => 1,
            TokenKind::Plus => 2,
            TokenKind::Minus => 3,
            TokenKind::Asterisk => 4,
            TokenKind::Slash => 5,
            TokenKind::Procent => 6,
            TokenKind::Carrot => 7,
            TokenKind::Lparen => 8,
            TokenKind::Rparen => 9,
            TokenKind::Lbrace => 10,
            TokenKind::Rbrace => 11,
            TokenKind::Bar => 12,
            TokenKind::Comma => 13,
            TokenKind::Whitespace => 14,
            TokenKind::Number(n) => ((n.abs() % 1.0) * (usize::MAX as f64)) as usize,
            TokenKind::Other(c) => (*c) as usize,
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            TokenKind::Number(n) => *n,
            tk => (tk.as_usize() % 10) as f64 / 10.0,
        }
    }
}

pub struct Lexer {
    source: Rc<String>,
    chars: Vec<char>,
    cursor: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            source: Rc::new(source),
            cursor: 0,
        }
    }

    pub fn source(&self) -> Rc<String> {
        Rc::clone(&self.source)
    }

    fn current(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }

    fn span(&self, start: usize) -> Span {
        Span {
            start,
            end: self.cursor,
        }
    }

    fn next(&mut self) {
        self.cursor += 1;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token_start = self.cursor;

        let token = |lexer: &mut Self, tk: TokenKind| {
            lexer.next();
            Some(Token {
                kind: tk,
                span: lexer.span(token_start),
            })
        };

        match self.current()? {
            '+' => token(self, TokenKind::Plus),
            '-' => token(self, TokenKind::Minus),
            '*' => token(self, TokenKind::Asterisk),
            '/' => token(self, TokenKind::Slash),
            '%' => token(self, TokenKind::Procent),
            '^' => token(self, TokenKind::Carrot),
            '(' => token(self, TokenKind::Lparen),
            ')' => token(self, TokenKind::Rparen),
            '{' => token(self, TokenKind::Lbrace),
            '}' => token(self, TokenKind::Rbrace),
            ',' => token(self, TokenKind::Comma),
            '|' => token(self, TokenKind::Bar),
            'x' | 'X' => token(self, TokenKind::X),
            'y' | 'Y' => token(self, TokenKind::Y),
            '0'..='9' => {
                let mut number = String::new();
                while let Some(c) = self.current() {
                    if c.is_ascii_digit() || c == '.' {
                        number.push(c);
                        self.next();
                    } else {
                        break;
                    }
                }
                Some(Token {
                    kind: TokenKind::Number(number.parse().unwrap()),
                    span: self.span(token_start),
                })
            }
            c if c.is_whitespace() => {
                self.next();
                Some(Token {
                    kind: TokenKind::Whitespace,
                    span: self.span(token_start),
                })
            }
            c => {
                self.next();
                Some(Token {
                    kind: TokenKind::Other(c),
                    span: self.span(token_start),
                })
            }
        }
    }
}
