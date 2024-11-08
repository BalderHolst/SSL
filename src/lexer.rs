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
            TokenKind::Comma => 12,
            TokenKind::Whitespace => 13,
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
        match self.current()? {
            '+' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Plus,
                    span: self.span(token_start),
                })
            }
            '-' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Minus,
                    span: self.span(token_start),
                })
            }
            '*' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Asterisk,
                    span: self.span(token_start),
                })
            }
            '/' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Slash,
                    span: self.span(token_start),
                })
            }
            '%' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Procent,
                    span: self.span(token_start),
                })
            }
            '^' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Carrot,
                    span: self.span(token_start),
                })
            }
            '(' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Lparen,
                    span: self.span(token_start),
                })
            }
            ')' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Rparen,
                    span: self.span(token_start),
                })
            }
            '{' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Lbrace,
                    span: self.span(token_start),
                })
            }
            '}' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Rbrace,
                    span: self.span(token_start),
                })
            }
            'x' | 'X' => {
                self.next();
                Some(Token {
                    kind: TokenKind::X,
                    span: self.span(token_start),
                })
            }
            'y' | 'Y' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Y,
                    span: self.span(token_start),
                })
            }
            ',' => {
                self.next();
                Some(Token {
                    kind: TokenKind::Comma,
                    span: self.span(token_start),
                })
            }
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
