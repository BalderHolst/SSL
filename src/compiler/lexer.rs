//! Lexer/tokenization implementation for SSL. The lexer converts source code into tokens. This lexer is byte-oriented because SSL input can be any data.

use std::rc::Rc;

use super::text::Span;

/// A language token.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Kinds of token.
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

    Number(f64),
    X,
    Y,
    R,
    A,
    TX,
    TY,

    Comma,

    Sin,
    Cos,

    If,
    Then,
    Else,
    End,

    Whitespace,

    Other(char),
}

impl TokenKind {
    /// Converts the token kind into a [usize].
    #[rustfmt::skip]
    pub fn as_usize(&self) -> usize {
        match self {
            TokenKind::X           => 0,
            TokenKind::Y           => 1,
            TokenKind::A           => 2,
            TokenKind::R           => 3,
            TokenKind::TX          => 4,
            TokenKind::TY          => 5,
            TokenKind::Plus        => 4,
            TokenKind::Minus       => 5,
            TokenKind::Asterisk    => 6,
            TokenKind::Slash       => 7,
            TokenKind::Procent     => 8,
            TokenKind::Less        => 9,
            TokenKind::Greater     => 10,
            TokenKind::Equal       => 11,
            TokenKind::Exclamation => 12,
            TokenKind::Carrot      => 13,
            TokenKind::Lparen      => 14,
            TokenKind::Rparen      => 15,
            TokenKind::Lbrace      => 16,
            TokenKind::Rbrace      => 17,
            TokenKind::Bar         => 18,
            TokenKind::And         => 19,
            TokenKind::Comma       => 20,
            TokenKind::Whitespace  => 21,
            TokenKind::Sin         => 22,
            TokenKind::Cos         => 23,
            TokenKind::If          => 24,
            TokenKind::Then        => 25,
            TokenKind::Else        => 26,
            TokenKind::End         => 27,
            TokenKind::Number(n)   => ((n.abs() % 1.0) * (usize::MAX as f64)) as usize,
            TokenKind::Other(c)    => (*c) as usize,
        }
    }

    /// Converts the token kind into a [f64].
    pub fn as_f64(&self) -> f64 {
        match self {
            TokenKind::Number(n) => *n,
            tk => (tk.as_usize() % 10) as f64 / 10.0,
        }
    }
}

/// The lexer for the SSL language.
pub struct Lexer {
    source: Rc<Vec<u8>>,
    cursor: usize,
}

impl Lexer {
    /// Create a new lexer from a source string.
    pub fn new(source: String) -> Self {
        Self {
            source: Rc::new(source.into_bytes()),
            cursor: 0,
        }
    }

    /// Get the source of the lexer.
    pub fn source(&self) -> Rc<Vec<u8>> {
        Rc::clone(&self.source)
    }

    /// Get the current character. Returns [None] if the cursor is at the end of the source.
    fn current(&self) -> Option<char> {
        self.source.get(self.cursor).copied().map(Into::into)
    }

    /// Peak at a character at an offset from the cursor.
    fn peak(&self, offset: isize) -> Option<char> {
        self.source
            .get((self.cursor as isize + offset) as usize)
            .copied()
            .map(Into::into)
    }

    /// Move the cursor to the next character.
    fn next(&mut self) {
        self.cursor += 1;
    }

    /// Get the span from the start to the current cursor.
    fn span(&self, start: usize) -> Span {
        Span {
            start,
            end: self.cursor,
        }
    }

    /// Checks if the next characters make up a decimal number literal (like 2.1).
    fn is_at_number(&self) -> bool {
        let mut cursor = self.cursor;
        while let Some(c) = self.source.get(cursor).copied() {
            let c = c as char;
            cursor += 1;
            if c == '.' {
                break;
            }
            if !c.is_ascii_digit() {
                return false;
            }
        }
        matches!(self.source.get(cursor).copied().map(|c| c as char), Some(c) if c.is_ascii_digit())
    }
}

impl Iterator for Lexer {
    type Item = Token;

    /// Get the next token from the lexer.
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
            'x' => token(self, TokenKind::X),
            'y' => token(self, TokenKind::Y),
            'a' => token(self, TokenKind::A),
            'r' => token(self, TokenKind::R),
            't' if self.peak(1) == Some('x') => {
                self.next();
                token(self, TokenKind::TX)
            }
            't' if self.peak(1) == Some('y') => {
                self.next();
                token(self, TokenKind::TY)
            }
            '0'..='9' if self.is_at_number() => {
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
                    kind: TokenKind::Number(number.parse().expect("Invalid number parsed.")),
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
                    [Some('s'), Some('i'), Some('n'),         _] => (3, token(self, TokenKind::Sin)),
                    [Some('c'), Some('o'), Some('s'),         _] => (3, token(self, TokenKind::Cos)),
                    _ => return token(self, TokenKind::Other(c)),
                };
                // Subtract one because `token` already increments the cursor.
                self.cursor += size - 1;
                token
            }
        }
    }
}
