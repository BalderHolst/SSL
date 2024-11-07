#[derive(Debug, PartialEq)]
pub enum Token {

    PLUS,
    MINUS,
    MULT,
    DIV,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    X,
    Y,
    R,
    NUMBER(f64),

    SPACE,
    NEWLINE,

    OTHER(char),
}

pub struct Lexer {
    chars: Vec<char>,
    cursor: usize,
}

impl Lexer {

    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            cursor: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }

    fn next(&mut self) {
        self.cursor += 1;
    }

}

impl Iterator for Lexer {

    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.current() {
            match c {
                '+' => {
                    self.next();
                    return Some(Token::PLUS);
                }
                '-' => {
                    self.next();
                    return Some(Token::MINUS);
                }
                '*' => {
                    self.next();
                    return Some(Token::MULT);
                }
                '/' => {
                    self.next();
                    return Some(Token::DIV);
                }
                '(' => {
                    self.next();
                    return Some(Token::LPAREN);
                }
                ')' => {
                    self.next();
                    return Some(Token::RPAREN);
                }
                '{' => {
                    self.next();
                    return Some(Token::LBRACE);
                }
                '}' => {
                    self.next();
                    return Some(Token::RBRACE);
                }
                'x' | 'X' => {
                    self.next();
                    return Some(Token::X);
                }
                'y' | 'Y' => {
                    self.next();
                    return Some(Token::Y);
                }
                'r' | 'R' => {
                    self.next();
                    return Some(Token::R);
                }
                '0'..='9' => {
                    let mut number = String::new();
                    while let Some(c) = self.current() {
                        if c.is_digit(10) || c == '.' {
                            number.push(c);
                            self.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::NUMBER(number.parse().unwrap()));
                }
                '\n' => {
                    self.next();
                    return Some(Token::NEWLINE);
                }
                c if c.is_whitespace() => {
                    self.next();
                    return Some(Token::SPACE);
                }
                _ => {
                    self.next();
                    return Some(Token::OTHER(c));
                }
            }
        }
        None
    }

}
