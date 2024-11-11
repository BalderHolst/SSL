use std::{cmp::Ordering, ops::{Add, Div, Mul, Sub}};

use crate::ast::{BinOp, Expr, ExprKind};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Result {
    Color(Color),
    Number(f64),
    Bool(bool),
}

fn bool_to_f64(b: bool) -> f64 {
    match b {
        true => 1.0,
        false => -1.0,
    }
}

impl Result {
    fn as_number(&self) -> f64 {
        match self {
            Result::Color(c) => (c.r + c.g + c.b) / 3.0,
            Result::Number(n) => *n,
            Result::Bool(b) => bool_to_f64(*b),
        }
    }

    fn as_color(&self) -> Color {
        match self {
            Result::Color(c) => c.clone(),
            Result::Number(n) => Color {
                r: *n,
                g: *n,
                b: *n,
            },
            Result::Bool(b) => Color {
                r: bool_to_f64(*b),
                g: bool_to_f64(*b),
                b: bool_to_f64(*b),
            },
        }
    }

    fn as_bool(&self) -> bool {
        match self {
            Result::Color(_) => self.as_number() >= 0.0,
            Result::Number(n) => *n >= 0.0,
            Result::Bool(b) => *b,
        }
    }

    fn nan_to_zero(&mut self) {
        fn zero_if_nan(n: &mut f64) {
            if n.is_nan() {
                *n = 0.0;
            }
        }

        match self {
            Result::Color(c) => {
                zero_if_nan(&mut c.r);
                zero_if_nan(&mut c.g);
                zero_if_nan(&mut c.b);
            }
            Result::Number(n) => zero_if_nan(n),
            Result::Bool(_) => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-2.0 * x + 0.5))
}

impl Color {
    fn clamp(&mut self) {
        self.r = sigmoid(self.r);
        self.g = sigmoid(self.g);
        self.b = sigmoid(self.b);
    }
}

macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Result::Color(Color {
            r: $r,
            g: $g,
            b: $b,
        })
    };
}

macro_rules! number {
    ($n:expr) => {
        Result::Number($n)
    };
}

macro_rules! bool {
    ($n:expr) => {
        Result::Bool($n)
    };
}


macro_rules! impl_trait_op {
    ($trait:ident, $method:ident, $struct:ident) => {
        impl $trait for $struct {
            type Output = Self;
            fn $method(self, other: Self) -> Self::Output {
                match (self, other) {
                    (Result::Color(c1), Result::Color(c2)) => {
                        color!(c1.r.$method(c2.r), c1.g.$method(c2.g), c1.b.$method(c2.b))
                    }
                    (Result::Color(c), Result::Number(n)) => {
                        color!(c.r.$method(n), c.g.$method(n), c.b.$method(n))
                    }
                    (Result::Number(n), Result::Color(c)) => {
                        color!(n.$method(c.r), n.$method(c.g), n.$method(c.b))
                    }
                    (Result::Number(n1), Result::Number(n2)) => number!(n1.$method(n2)),
                    (Result::Color(c), Result::Bool(b)) => {
                        let b = bool_to_f64(b);
                        color!(c.r.$method(b), c.g.$method(b), c.b.$method(b))
                    },
                    (Result::Number(n), Result::Bool(b)) => number!(n.$method(bool_to_f64(b))),
                    (Result::Bool(b), Result::Color(c)) => {
                        let b = bool_to_f64(b);
                        color!(b.$method(c.r), b.$method(c.g), b.$method(c.b))
                    },
                    (Result::Bool(b), Result::Number(n)) => number!(bool_to_f64(b).$method(n)),
                    (Result::Bool(b1), Result::Bool(b2)) => {
                        number!(bool_to_f64(b1).$method(bool_to_f64(b2)))
                    }
                }
            }
        }
    };
}
impl_trait_op!(Add, add, Result);
impl_trait_op!(Sub, sub, Result);
impl_trait_op!(Mul, mul, Result);
impl_trait_op!(Div, div, Result);

impl Result {
    fn fmod(&self, other: Self) -> Self {
        match (self, other) {
            (Result::Color(c1), Result::Color(c2)) => {
                color!(c1.r % c2.r, c1.g % c2.g, c1.b % c2.b)
            }
            (Result::Color(c), Result::Number(n)) => {
                color!(c.r % n, c.g % n, c.b % n)
            }
            (Result::Number(n), Result::Color(c)) => {
                color!(n % c.r, n % c.g, n % c.b)
            }
            (Result::Number(n1), Result::Number(n2)) => number!(n1 % n2),
            (Result::Color(c), Result::Bool(b)) => {
                let b = bool_to_f64(b);
                color!(c.r % b, c.g % b, c.b % b)
            },
            (Result::Number(n), Result::Bool(b)) => number!(n % bool_to_f64(b)),
            (Result::Bool(b), Result::Color(c)) => {
                let b = bool_to_f64(*b);
                color!(b % c.r, b % c.g, b % c.b)
            },
            (Result::Bool(b), Result::Number(n)) => number!(bool_to_f64(*b) % n),
            (Result::Bool(b1), Result::Bool(b2)) => bool!(b1 ^ b2),
        }
    }

    fn pow(&self, other: Self) -> Self {
        match (self, other) {
            (Result::Color(c1), Result::Color(c2)) => {
                color!(c1.r.powf(c2.r), c1.g.powf(c2.g), c1.b.powf(c2.b))
            }
            (Result::Color(c), Result::Number(n)) => {
                color!(c.r.powf(n), c.g.powf(n), c.b.powf(n))
            }
            (Result::Number(n), Result::Color(c)) => {
                color!(n.powf(c.r), n.powf(c.g), n.powf(c.b))
            }
            (Result::Number(n1), Result::Number(n2)) => number!(n1.powf(n2)),
            (Result::Color(c), Result::Bool(b)) => {
                let b = bool_to_f64(b);
                color!(c.r.powf(b), c.g.powf(b), c.b.powf(b))
            },
            (Result::Number(n), Result::Bool(b)) => number!(n.powf(bool_to_f64(b))),
            (Result::Bool(b), Result::Color(c)) => {
                let b = bool_to_f64(*b);
                color!(b.powf(c.r), b.powf(c.g), b.powf(c.b))
            },
            (Result::Bool(b), Result::Number(n)) => number!(bool_to_f64(*b).powf(n)),
            (Result::Bool(b1), Result::Bool(b2)) => bool!(b1 ^ b2),
        }
    }

    fn abs(&self) -> Self {
        match self {
            Result::Color(c) => {
                color!(c.r.abs(), c.g.abs(), c.b.abs())
            }
            Result::Number(n) => number!(n.abs()),
            Result::Bool(_) => number!(self.as_number().abs()),
        }
    }

    fn max(&self, other: &Self) -> Self {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => other.clone(),
            _ => self.clone(),
        }
    }

    fn min(&self, other: &Self) -> Self {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => other.clone(),
            _ => self.clone(),
        }
    }
}

fn eval_expr(expr: &Expr, x: f64, y: f64) -> Result {
    let mut res = match &expr.kind {
        ExprKind::Bin(e) => {
            let l = eval_expr(&e.lhs, x, y);
            let r = eval_expr(&e.rhs, x, y);
            match e.op {
                BinOp::Add => l + r,
                BinOp::Sub => l - r,
                BinOp::Mul => l * r,
                BinOp::Div => l / r,
                BinOp::Mod => l.fmod(r),
                BinOp::Pow => l.pow(r),
                BinOp::Max => l.max(&r),
                BinOp::Min => l.min(&r),
            }
        }
        ExprKind::Color(c) => {
            let r = eval_expr(&c.r, x, y);
            let b = eval_expr(&c.g, x, y);
            let g = eval_expr(&c.b, x, y);
            color!(r.as_number(), g.as_number(), b.as_number())
        }
        ExprKind::Paren(e) => eval_expr(&e.inner, x, y),
        ExprKind::Neg(e) => eval_expr(&e.inner, x, y) * number!(-1.0),
        ExprKind::Abs(e) => eval_expr(&e.inner, x, y).abs(),
        ExprKind::Number(n) => number!(*n),
        ExprKind::X => number!(x),
        ExprKind::Y => number!(y),
        ExprKind::If(e) => {
            let cond = eval_expr(&e.cond, x, y);
            if cond.as_bool() {
                eval_expr(&e.true_expr, x, y)
            } else {
                eval_expr(&e.false_expr, x, y)
            }
        }
    };
    res.nan_to_zero();
    res
}

pub fn eval(expr: &Expr, x: f64, y: f64) -> Color {
    let mut res = eval_expr(expr, x, y).as_color();
    res.clamp();
    res
}
