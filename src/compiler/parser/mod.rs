//! Parser implementation for the SSL language

use std::rc::Rc;

mod cornelia;

use crate::text::Span;

use super::{
    ast::{
        self, AbsExpr, BinExpr, BinOp, ColorExpr, CosExpr, Expr, ExprKind, IfExpr, NegExpr,
        ParenExpr, SinExpr,
    },
    constant_evaluator,
    lexer::{self, Token, TokenKind},
};

#[allow(dead_code)] // TODO: Find a better solution
const MAX_TRIES: usize = 100;

#[allow(dead_code)] // TODO: Find a better solution
pub fn parse_tokens(tokens: Vec<Token>, source: Rc<Vec<u8>>, on_retry: impl Fn()) -> ast::Expr {
    let mut parser = Parser::new(tokens, source);
    let mut expr = parser.parse_expr();
    for retry in 1..MAX_TRIES {
        expr = constant_evaluator::evaluate_constants(expr);

        if !expr.is_constant() {
            break;
        }
        on_retry();
        parser.reset();
        parser.seed = retry;
        expr = parser.parse_expr();
    }
    expr
}

/// Parser for SSL
pub struct Parser {
    source: Rc<Vec<u8>>,
    tokens: Vec<Token>,
    cursor: usize,
    looking_for: Vec<TokenKind>,
    seed: usize,
    not_number: usize,
}

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
    fn choose_token(&mut self) -> Expr {
        let mut span = Span {
            start: self.source.len(),
            end: self.source.len(),
        };

        let (n, f) = match self.current() {
            Some(t) => {
                span = t.span.clone();
                (t.kind.as_usize(), t.kind.as_f64())
            }
            None => (self.seed(), (((self.seed() % 100) as f64) / 100.0) % 1.0),
        };

        let expr = |kind: ExprKind| Expr {
            kind,
            span: span.clone(),
        };

        let l = self.looking_for.len() + 1;

        let num = match self.not_number {
            0 => 3,
            _ => 0,
        };

        choice! { n + self.seed(),
            5/l => self.parse_color(),
            2/l => self.parse_parenthesized_expr(),
            1/l => self.parse_sin_expr(),
            1/l => self.parse_cos_expr(),
            4 => expr(ExprKind::X),
            4 => expr(ExprKind::Y),
            2 => expr(ExprKind::R),
            2 => expr(ExprKind::A),
            num => expr(ExprKind::Number(f)),
            l.min(2) => self.parse_if_expr(),
            0 => self.parse_neg_expr(),
        }
    }

    fn choose_binop(&mut self, seed: usize) -> BinOp {
        choice! {seed + self.seed(),
            7 => BinOp::Add,
            7 => BinOp::Sub,
            9 => BinOp::Mul,
            9 => BinOp::Div,
            9 => BinOp::Mod,
            6 => BinOp::Pow,
            1 => BinOp::And,
            1 => BinOp::Or,
            0 => BinOp::LessThan,
            0 => BinOp::GreaterThan,
        }
    }
}

/// Parse source code into an expression
#[allow(dead_code)] // TODO: Find a better solution
pub fn parse_source(source: String) -> ast::Expr {
    let lexer = lexer::Lexer::new(source);
    let source = lexer.source();
    let tokens: Vec<_> = lexer.collect();
    let mut parser = Parser::new(tokens, source);
    parser.parse_expr()
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: Rc<Vec<u8>>) -> Self {
        Self {
            tokens,
            source,
            cursor: 0,
            looking_for: vec![],
            seed: 0,
            not_number: 0,
        }
    }

    fn reset(&mut self) {
        self.cursor = 0;
        self.looking_for.clear();
        self.seed = 0;
        self.not_number = 0;
    }

    fn peak(&self, offset: isize) -> Option<&Token> {
        let cursor = self.cursor as isize + offset;
        if cursor < 0 || cursor >= self.tokens.len() as isize {
            return None;
        }
        self.tokens.get(cursor as usize)
    }

    fn seed(&mut self) -> usize {
        self.seed = self.seed.wrapping_add(1);
        self.seed
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn consume_whitespace(&mut self) {
        while self
            .current()
            .map_or(false, |t| t.kind == TokenKind::Whitespace)
        {
            self.cursor += 1;
        }
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.current()?.clone();
        self.cursor += 1;
        self.consume_whitespace();
        Some(token)
    }

    fn consume_if(&mut self, f: impl FnOnce(&TokenKind) -> bool) -> Option<Token> {
        let token = self.current()?.clone();
        if f(&token.kind) {
            self.consume();
            Some(token)
        } else {
            None
        }
    }

    fn current_span(&self) -> Span {
        match self.current() {
            Some(t) => t.span.clone(),
            None => Span {
                start: self.source.len(),
                end: self.source.len(),
            },
        }
    }

    fn parse_color(&mut self) -> Expr {
        let start_span = self.current_span();

        self.consume_if(|tk| *tk == TokenKind::Lbrace);

        self.looking_for.push(TokenKind::Comma);
        let r = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Comma);

        self.looking_for.push(TokenKind::Comma);
        let g = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Comma);

        self.looking_for.push(TokenKind::Rbrace);
        let b = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Rbrace);

        Expr {
            kind: ExprKind::Color(ColorExpr::new(r, g, b)),
            span: Span {
                start: start_span.start,
                end: self.current_span().start,
            },
        }
    }

    fn parse_neg_expr(&mut self) -> Expr {
        let start_span = self.current_span();
        self.consume_if(|tk| *tk == TokenKind::Minus);
        let inner = self.parse_expr();
        let end = self.peak(-1);

        Expr {
            kind: ExprKind::Neg(NegExpr::new(inner)),
            span: Span {
                end: end.map_or(self.source.len() - 1, |t| t.span.end),
                start: start_span.start,
            },
        }
    }

    fn parse_abs_expr(&mut self) -> Expr {
        let start_span = self.current_span();

        self.consume_if(|tk| *tk == TokenKind::Bar);

        self.looking_for.push(TokenKind::Bar);
        let inner = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Bar);

        Expr {
            kind: ExprKind::Abs(AbsExpr::new(inner)),
            span: Span {
                start: start_span.start,
                end: self.current_span().start,
            },
        }
    }

    fn parse_function(&mut self, kind: impl FnOnce(Expr) -> ExprKind) -> Expr {
        let start_span = self.current_span();
        self.consume(); // Consume function name

        self.consume_if(|tk| *tk == TokenKind::Lparen);

        self.looking_for.push(TokenKind::Rparen);
        let inner = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Rparen); // Consume ')'

        Expr {
            kind: kind(inner),
            span: Span {
                start: start_span.start,
                end: self.current_span().end,
            },
        }
    }

    fn parse_sin_expr(&mut self) -> Expr {
        self.parse_function(|e| ExprKind::Sin(SinExpr::new(e)))
    }

    fn parse_cos_expr(&mut self) -> Expr {
        self.parse_function(|e| ExprKind::Cos(CosExpr::new(e)))
    }

    fn parse_if_expr(&mut self) -> Expr {
        let start_span = self.current_span();

        self.consume_if(|tk| *tk == TokenKind::If);

        self.looking_for.push(TokenKind::Then);
        self.not_number += 1;
        let cond = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Then);

        self.looking_for.push(TokenKind::Else);
        let true_expr = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Else);

        self.looking_for.push(TokenKind::End);
        let false_expr = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::End);

        Expr {
            kind: ExprKind::If(IfExpr::new(cond, true_expr, false_expr)),
            span: Span {
                start: start_span.start,
                end: self.current_span().start,
            },
        }
    }

    fn parse_parenthesized_expr(&mut self) -> Expr {
        let start_span = self.current_span();

        self.consume_if(|tk| *tk == TokenKind::Lparen);

        self.looking_for.push(TokenKind::Rparen);
        let inner = self.parse_expr();
        self.looking_for.pop();

        self.consume_if(|tk| *tk == TokenKind::Rparen);

        Expr {
            kind: ExprKind::Paren(ParenExpr::new(inner)),
            span: Span {
                start: start_span.start,
                end: self.current_span().start,
            },
        }
    }

    fn parse_cornelia(&mut self) -> Expr {
        let start_span = self.current_span();

        let len = "Cornelia".len();

        for _ in 0..len {
            self.consume();
        }

        let span = Span {
            start: start_span.start,
            end: start_span.start + len,
        };

        cornelia::cornelia_expr(span)
    }

    fn is_done(&mut self) -> bool {
        self.consume_whitespace();
        self.cursor >= self.tokens.len()
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let Some(token) = self.current() else {
            return self.choose_token();
        };

        let token_span = token.span.clone();

        let expr = |kind: ExprKind| Expr {
            kind,
            span: token_span,
        };

        let expr = match &token.kind {
            TokenKind::Minus => self.parse_neg_expr(),
            TokenKind::Lparen => self.parse_parenthesized_expr(),
            TokenKind::Lbrace => self.parse_color(),
            TokenKind::Bar => self.parse_abs_expr(),
            TokenKind::If => self.parse_if_expr(),
            TokenKind::X => {
                self.consume();
                expr(ExprKind::X)
            }
            TokenKind::Y => {
                self.consume();
                expr(ExprKind::Y)
            }
            TokenKind::R => {
                self.consume();
                expr(ExprKind::R)
            }
            TokenKind::A => {
                self.consume();
                expr(ExprKind::A)
            }
            TokenKind::Number(n) => {
                let n = *n;
                self.consume();
                expr(ExprKind::Number(n))
            }
            TokenKind::Sin => self.parse_sin_expr(),
            TokenKind::Cos => self.parse_cos_expr(),
            TokenKind::Other('C') | TokenKind::Other('c')
                if (
                    self.peak(1).map(|t| &t.kind),
                    self.peak(2).map(|t| &t.kind),
                    self.peak(3).map(|t| &t.kind),
                    self.peak(4).map(|t| &t.kind),
                    self.peak(5).map(|t| &t.kind),
                    self.peak(6).map(|t| &t.kind),
                    self.peak(7).map(|t| &t.kind),
                ) == (
                    Some(&TokenKind::Other('o')),
                    Some(&TokenKind::R),
                    Some(&TokenKind::Other('n')),
                    Some(&TokenKind::Other('e')),
                    Some(&TokenKind::Other('l')),
                    Some(&TokenKind::Other('i')),
                    Some(&TokenKind::A),
                ) =>
            {
                self.parse_cornelia()
            }
            _ => self.choose_token(),
        };
        if self.not_number > 0 {
            self.not_number -= 1;
        }
        expr
    }

    fn get_bin_op(&mut self) -> BinOp {
        let Some(token) = self.current() else {
            return self.choose_binop(0);
        };
        match &token.kind {
            TokenKind::Plus => BinOp::Add,
            TokenKind::Minus => BinOp::Sub,
            TokenKind::Asterisk => BinOp::Mul,
            TokenKind::Slash => BinOp::Div,
            TokenKind::Procent => BinOp::Mod,
            TokenKind::Carrot => BinOp::Pow,
            TokenKind::Bar => BinOp::Or,
            TokenKind::And => BinOp::And,
            TokenKind::Less => BinOp::LessThan,
            TokenKind::Greater => BinOp::GreaterThan,
            tk => self.choose_binop(tk.as_usize()),
        }
    }

    fn is_at_interest(&mut self) -> bool {
        let seed = self.seed();
        if let Some(interest) = self.looking_for.last() {
            if let Some(token) = self.current() {
                if &token.kind == interest {
                    return true;
                }

                if matches!(&token.kind, TokenKind::Other(_)) {
                    return choice! { token.kind.as_usize() + seed,
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
