use std::rc::Rc;

use crate::{
    ast::{AbsExpr, BinExpr, BinOp, ColorExpr, Expr, ExprKind, NegExpr, ParenExpr},
    lexer::{Token, TokenKind},
    text::Span,
};

pub struct Parser {
    source: Rc<String>,
    tokens: Vec<Token>,
    cursor: usize,
    looking_for: Vec<TokenKind>,
}

const DEFAULT_EXPR_KIND: ExprKind = ExprKind::Number(1.0);
const DEFAULT_BIN_OP: BinOp = BinOp::Mul;

/// Choose an expression based on a weighted choice and a seed number.
macro_rules! choice {
    ($n:expr, $($w:expr => $res:expr),*$(,)?) => {{ (|| {
        let total: usize = [$($w),*].into_iter().sum();
        let n = ($n as usize) % total;
        let mut _index: usize = 0;
        $(
            if n >= _index && n < (_index + $w) {
                return $res;
            }
            _index += $w;
        )*
        unreachable!()
    })() }};
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: Rc<String>) -> Self {
        Self {
            tokens,
            source,
            cursor: 0,
            looking_for: vec![],
        }
    }

    fn peak(&self, offset: isize) -> Option<&Token> {
        let cursor = self.cursor as isize + offset;
        if cursor < 0 || cursor >= self.tokens.len() as isize {
            return None;
        }
        self.tokens.get(cursor as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.current()?.clone();
        self.cursor += 1;
        while self.current()?.kind == TokenKind::Whitespace {
            self.cursor += 1;
        }
        // println!("TK: {:?}", &token.kind);
        Some(token)
    }

    fn parse_color(&mut self) -> Expr {
        let start_span = self.current().unwrap().span.clone();

        self.consume(); // Consume left brace

        self.looking_for.push(TokenKind::Comma);
        let r = self.parse_expr();
        self.looking_for.pop();

        self.consume(); // Consume comma

        self.looking_for.push(TokenKind::Comma);
        let g = self.parse_expr();
        self.looking_for.pop();

        self.consume(); // Consume comma

        self.looking_for.push(TokenKind::Rbrace);
        let b = self.parse_expr();
        self.looking_for.pop();

        let end = self.consume(); // Consume right brace

        Expr {
            kind: ExprKind::Color(ColorExpr::new(r, g, b)),
            span: Span {
                end: end.map_or(self.source.len() - 1, |t| t.span.end),
                start: start_span.start,
            },
        }
    }

    fn parse_neg_expr(&mut self) -> Expr {
        let start_span = self.current().unwrap().span.clone();
        self.consume(); // Consume '-'
        let inner = self.parse_expr();
        let end = self.peak(-1); // Consume right parenthesis

        Expr {
            kind: ExprKind::Neg(NegExpr::new(inner)),
            span: Span {
                end: end.map_or(self.source.len() - 1, |t| t.span.end),
                start: start_span.start,
            },
        }
    }

    fn parse_abs_expr(&mut self) -> Expr {
        let start_span = self.current().unwrap().span.clone();

        self.consume(); // Consume left |

        self.looking_for.push(TokenKind::Bar);

        let inner = self.parse_expr();

        self.looking_for.pop();

        let end = self.consume(); // Consume right parenthesis

        Expr {
            kind: ExprKind::Abs(AbsExpr::new(inner)),
            span: Span {
                end: end.map_or(self.source.len() - 1, |t| t.span.end),
                start: start_span.start,
            },
        }
    }

    fn parse_parenthesized_expr(&mut self) -> Expr {
        let start_span = self.current().unwrap().span.clone();

        self.consume(); // Consume left parenthesis

        self.looking_for.push(TokenKind::Rparen);

        let inner = self.parse_expr();

        self.looking_for.pop();

        let end = self.consume(); // Consume right parenthesis

        Expr {
            kind: ExprKind::Paren(ParenExpr::new(inner)),
            span: Span {
                end: end.map_or(self.source.len() - 1, |t| t.span.end),
                start: start_span.start,
            },
        }
    }

    fn is_done(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let Some(token) = self.current() else {
            return Expr {
                kind: DEFAULT_EXPR_KIND,
                span: Span {
                    start: self.source.len(),
                    end: self.source.len(),
                },
            };
        };

        let token_span = token.span.clone();

        let expr = |kind: ExprKind| Expr {
            kind,
            span: token_span,
        };

        match &token.kind {
            TokenKind::Minus => self.parse_neg_expr(),
            TokenKind::Lparen => self.parse_parenthesized_expr(),
            TokenKind::Lbrace => self.parse_color(),
            TokenKind::Bar => self.parse_abs_expr(),
            TokenKind::X => {
                self.consume();
                expr(ExprKind::X)
            }
            TokenKind::Y => {
                self.consume();
                expr(ExprKind::Y)
            }
            TokenKind::Number(n) => {
                let n = n.clone();
                self.consume();
                expr(ExprKind::Number(n))
            },
            tk => {
                let n = tk.as_usize();
                let number = tk.as_f64();
                let expr = choice! { n,
                    3 => self.parse_color(),
                    3 => expr(ExprKind::X),
                    3 => expr(ExprKind::Y),
                    3 => expr(ExprKind::Number(number)),
                    1 => self.parse_neg_expr(),
                    1 => self.parse_parenthesized_expr(),
                };
                self.consume();
                expr
            }
        }
    }

    fn get_bin_op(&self) -> BinOp {
        let Some(token) = self.current() else {
            return DEFAULT_BIN_OP;
        };
        match &token.kind {
            TokenKind::Plus => BinOp::Add,
            TokenKind::Minus => BinOp::Sub,
            TokenKind::Asterisk => BinOp::Mul,
            TokenKind::Slash => BinOp::Div,
            TokenKind::Procent => BinOp::Mod,
            TokenKind::Carrot => BinOp::Pow,
            tk => choice! {tk.as_usize() ,
                1 => BinOp::Add,
                1 => BinOp::Sub,
                3 => BinOp::Mul,
                3 => BinOp::Div,
                3 => BinOp::Mod,
                2 => BinOp::Pow,
            },
        }
    }

    fn is_at_interest(&self) -> bool {
        if let Some(interest) = self.looking_for.last() {
            if let Some(token) = self.current() {

                if &token.kind == interest {
                    return true;
                }

                if matches!(&token.kind, TokenKind::Other(_)) {
                    return choice!{ token.kind.as_usize(),
                        1 => true,
                        3 => false,
                    };
                }
            }
        }
        false
    }

    /// Parse a binary expression.
    /// https://en.wikipedia.org/wiki/Operator-precedence_parser
    fn parse_binary_expr(&mut self, left: Option<Expr>, min_precedence: u8) -> Expr {

        let mut left = match left {
            Some(e) => e,
            None => self.parse_primary_expr(),
        };

        if self.is_at_interest() {
            self.looking_for.pop();
            return left;
        }

        let start_span = left.span.clone();

        while !self.is_done() {

            if self.is_at_interest() {
                return left;
            }

            let op = self.get_bin_op();

            if op.precedence() < min_precedence {
                break;
            }

            self.consume(); // Consume operator

            let mut right = self.parse_primary_expr();

            while !self.is_done() {
                if self.is_at_interest() {
                    let span = Span::from_spans(&start_span, &right.span);
                    return Expr {
                        kind: ExprKind::Bin(BinExpr {
                            op,
                            lhs: Box::new(left),
                            rhs: Box::new(right),
                        }),
                        span,
                    };
                }

                let right_op = self.get_bin_op();
                if right_op.precedence() <= op.precedence() {
                    break;
                }
                right = self.parse_binary_expr(Some(right), right_op.precedence());

                if self.is_at_interest() {
                    break;
                }

            }

            let span = Span::from_spans(&start_span, &right.span);
            left = Expr {
                kind: ExprKind::Bin(BinExpr {
                    op,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                }),
                span,
            };
        }

        left
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_binary_expr(None, 0)
    }
}
