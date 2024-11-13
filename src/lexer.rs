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

    Less,
    Greater,
    Equal,
    Exclamation,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Bar,
    And,

    X,
    Y,
    Number(f64),

    Comma,

    If,
    Then,
    Else,
    End,

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
            TokenKind::Less => 7,
            TokenKind::Greater => 8,
            TokenKind::Equal => 9,
            TokenKind::Exclamation => 10,
            TokenKind::Carrot => 11,
            TokenKind::Lparen => 12,
            TokenKind::Rparen => 13,
            TokenKind::Lbrace => 14,
            TokenKind::Rbrace => 15,
            TokenKind::Bar => 16,
            TokenKind::And => 17,
            TokenKind::Comma => 18,
            TokenKind::Whitespace => 19,
            TokenKind::If => 20,
            TokenKind::Then => 21,
            TokenKind::Else => 22,
            TokenKind::End => 23,
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
    source: Rc<Vec<u8>>,
    cursor: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: Rc::new(source.into_bytes()),
            cursor: 0,
        }
    }

    pub fn source(&self) -> Rc<Vec<u8>> {
        Rc::clone(&self.source)
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.cursor).copied().map(Into::into)
    }

    fn peak(&self, offset: isize) -> Option<char> {
        self.source
            .get((self.cursor as isize + offset) as usize)
            .copied()
            .map(Into::into)
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
            '&' => token(self, TokenKind::And),
            '<' => token(self, TokenKind::Less),
            '>' => token(self, TokenKind::Greater),
            '=' => token(self, TokenKind::Equal),
            '!' => token(self, TokenKind::Exclamation),
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
                #[rustfmt::skip]
                let (size, token) = match [self.peak(0), self.peak(1), self.peak(2), self.peak(3)] {
                    [Some('i'), Some('f'),         _,         _] => (2, token(self, TokenKind::If)),
                    [Some('t'), Some('h'), Some('e'), Some('n')] => (4, token(self, TokenKind::Then)),
                    [Some('e'), Some('l'), Some('s'), Some('e')] => (4, token(self, TokenKind::Else)),
                    [Some('e'), Some('n'), Some('d'),         _] => (3, token(self, TokenKind::End)),
                    _ => return token(self, TokenKind::Other(c)),
                };
                self.cursor += size;
                token
            }
        }
    }
}
