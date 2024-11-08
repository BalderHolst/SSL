use std::rc::Rc;

use visitor::Visitor;

use crate::text::Span;

mod visitor;
mod printer;

pub type NumberExpr = f64;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn print(&self, source: Rc<String>) {
        let mut printer = printer::Printer::new(source);
        printer.visit_expr(self);
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Bin(BinExpr),
    Number(NumberExpr),
    Color(ColorExpr),
    Paren(ParenExpr),
    Neg(NegExpr),
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
}

impl BinOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinOp::Add => 1,
            BinOp::Sub => 1,
            BinOp::Mul => 2,
            BinOp::Div => 2,
            BinOp::Mod => 2,
            BinOp::Pow => 3,
        }
    }
}

#[derive(Debug)]
pub struct BinExpr {
    pub op: BinOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
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
