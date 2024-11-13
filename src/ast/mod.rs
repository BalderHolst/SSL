use std::rc::Rc;

use visitor::Visitor;

use crate::text::Span;

mod printer;
mod visitor;

pub type NumberExpr = f64;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    #[allow(dead_code)]
    pub fn print(&self, source: Rc<Vec<u8>>) {
        let mut printer = printer::Printer::new(source);
        printer.visit_expr(self);
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Bin(BinExpr),
    If(IfExpr),
    Number(NumberExpr),
    Color(ColorExpr),
    Paren(ParenExpr),
    Neg(NegExpr),
    Abs(AbsExpr),
    X,
    Y,
}

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(u8)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    LessThan,
    GreaterThan,
    Equal,
    Or,
    And,
}

impl BinOp {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::LessThan => 0,
            Self::GreaterThan => 0,
            Self::Equal => 0,
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ParenExpr {
    pub inner: Box<Expr>,
}

impl ParenExpr {
    pub fn new(inner: Expr) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

#[derive(Debug)]
pub struct NegExpr {
    pub inner: Box<Expr>,
}

impl NegExpr {
    pub fn new(inner: Expr) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

#[derive(Debug)]
pub struct AbsExpr {
    pub inner: Box<Expr>,
}

impl AbsExpr {
    pub fn new(inner: Expr) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}
