//! Abstract syntax tree for the SSL language.

use core::fmt;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::text::Span;
use visitor::Visitor;

mod printer;
mod visitor;

/// A number expression
pub type NumberExpr = f64;

/// An SSL expression
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Expr {
    /// Print the AST of the expression. This is a lot of output.
    #[allow(dead_code)]
    pub fn print_ast(&self, source: Rc<Vec<u8>>) {
        let mut printer = printer::Printer::new(source);
        printer.visit_expr(self);
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Bin(e) => write!(f, "{:?}({}, {})", e.op, e.lhs, e.rhs),
            ExprKind::Paren(e) => write!(f, "({})", e.inner),
            ExprKind::Neg(e) => write!(f, "Neg({})", e.inner),
            ExprKind::Abs(e) => write!(f, "Abs({})", e.inner),
            ExprKind::Sin(e) => write!(f, "Sin({})", e.inner),
            ExprKind::Cos(e) => write!(f, "Cos({})", e.inner),
            ExprKind::Color(e) => write!(f, "{{{}, {}, {}}}", e.r, e.g, e.b),
            ExprKind::If(e) => write!(f, "If({}, {}, {})", e.cond, e.true_expr, e.false_expr),
            ExprKind::Number(n) => write!(f, "{n}"),
            ExprKind::X => write!(f, "X"),
            ExprKind::Y => write!(f, "Y"),
            ExprKind::R => write!(f, "R"),
            ExprKind::A => write!(f, "A"),
        }
    }
}

/// The kind of an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Bin(BinExpr),
    If(IfExpr),
    Number(NumberExpr),
    Color(ColorExpr),
    Paren(ParenExpr),
    Neg(NegExpr),
    Abs(AbsExpr),
    Sin(SinExpr),
    Cos(CosExpr),
    X,
    Y,
    R,
    A,
}

impl ExprKind {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Number(n) => *n == 0.0,
            Self::Color(ColorExpr { r, g, b }) => {
                r.kind.is_zero() && g.kind.is_zero() && b.kind.is_zero()
            }
            _ => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Self::Number(n) => *n == 1.0,
            _ => false,
        }
    }
}

/// Color expression. Syntax: `{r, g, b}`.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorExpr {
    pub r: Box<Expr>,
    pub g: Box<Expr>,
    pub b: Box<Expr>,
}

impl ColorExpr {
    pub fn new(r: Expr, g: Expr, b: Expr) -> Self {
        Self {
            r: Box::new(r),
            g: Box::new(g),
            b: Box::new(b),
        }
    }
}

/// Operator for binary expressions. The precedence is used to determine the order of operations.
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    LessThan,
    GreaterThan,
    Or,
    And,
}

impl BinOp {
    /// Get the precedence of the operator.
    pub fn precedence(&self) -> u8 {
        match self {
            Self::LessThan => 0,
            Self::GreaterThan => 0,
            Self::Or => 0,
            Self::And => 0,
            Self::Add => 1,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Div => 2,
            Self::Mod => 2,
            Self::Pow => 3,
        }
    }
}

/// A binary expression.
#[derive(Debug, Clone, PartialEq)]
pub struct BinExpr {
    pub op: BinOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl BinExpr {
    pub fn new(op: BinOp, lhs: Expr, rhs: Expr) -> Self {
        Self {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

/// An if expression. Syntax: `if <cond> then <true_expr> else <false_expr>`.
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub cond: Box<Expr>,
    pub true_expr: Box<Expr>,
    pub false_expr: Box<Expr>,
}

impl IfExpr {
    pub fn new(cond: Expr, true_expr: Expr, false_expr: Expr) -> Self {
        Self {
            cond: Box::new(cond),
            true_expr: Box::new(true_expr),
            false_expr: Box::new(false_expr),
        }
    }
}

/// Define an expression kind that simply wraps an expression.
macro_rules! wrapper_expr {
    ($name:ident: $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            pub inner: Box<Expr>,
        }

        impl $name {
            pub fn new(inner: Expr) -> Self {
                Self {
                    inner: Box::new(inner),
                }
            }
        }
    };
}

wrapper_expr!(ParenExpr: "Parenthesised expression. Syntax: `(<expr>)`.");
wrapper_expr!(NegExpr: "Negated expression. Syntax: `-<expr>`.");
wrapper_expr!(AbsExpr: "Absolute value expression. Syntax: `abs(<expr>)`.");
wrapper_expr!(SinExpr: "Sine expression. Syntax: `sin(<expr>)`.");
wrapper_expr!(CosExpr: "Cosine expression. Syntax: `cos(<expr>)`.");
