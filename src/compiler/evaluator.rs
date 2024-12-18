use std::{
    cmp::Ordering,
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use crate::compiler::ast::{BinOp, Expr, ExprKind};

/// Result of evaluating an expression.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Result {
    Color(Color),
    Number(f64),
    Bool(bool),
}

/// Cast an SSL boolean to a float.
fn bool_to_f64(b: bool) -> f64 {
    match b {
        true => 1.0,
        false => -1.0,
    }
}

/// Create a color result.
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Result::Color(Color {
            r: $r,
            g: $g,
            b: $b,
        })
    };
}

/// Create a number result.
macro_rules! number {
    ($n:expr) => {
        Result::Number($n)
    };
}

/// Create a boolean result.
macro_rules! bool {
    ($n:expr) => {
        Result::Bool($n)
    };
}

impl Result {
    /// Cast the result to a number.
    pub fn as_number(&self) -> f64 {
        match self {
            Result::Color(c) => (c.r + c.g + c.b) / 3.0,
            Result::Number(n) => *n,
            Result::Bool(b) => bool_to_f64(*b),
        }
    }

    /// Cast the result to a color.
    pub fn as_color(&self) -> Color {
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

    /// Cast the result to a boolean.
    pub fn as_bool(&self) -> bool {
        match self {
            Result::Color(_) => self.as_number() >= 0.0,
            Result::Number(n) => *n >= 0.0,
            Result::Bool(b) => *b,
        }
    }

    /// Convert NaN values to zero.
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

    /// Call a function on the result.
    fn call(&mut self, f: impl Fn(f64) -> f64) -> Result {
        match self {
            Result::Color(c) => color!(f(c.r), f(c.g), f(c.b)),
            Result::Number(n) => number!(f(*n)),
            Result::Bool(b) => number!(f(bool_to_f64(*b))),
        }
    }
}

/// An RGB color.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// The sigmoid function. Used to clamp color values.
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-2.0 * x + 0.5))
}

/// Normalize a value to the range [-1, 1].
fn norm(x: f64) -> f64 {
    sigmoid(x % 50.0)
}

impl Color {
    /// Clamp the color values to the range [0, 1].
    fn clamp(&mut self) {
        self.r = norm(self.r);
        self.g = norm(self.g);
        self.b = norm(self.b);
    }
}

/// Implement arithmetic operations for `Result` that have an associated trait.
macro_rules! impl_trait_op {
    ($struct:ident, $trait:ident, $method:ident) => {
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
                    }
                    (Result::Number(n), Result::Bool(b)) => number!(n.$method(bool_to_f64(b))),
                    (Result::Bool(b), Result::Color(c)) => {
                        let b = bool_to_f64(b);
                        color!(b.$method(c.r), b.$method(c.g), b.$method(c.b))
                    }
                    (Result::Bool(b), Result::Number(n)) => number!(bool_to_f64(b).$method(n)),
                    (Result::Bool(b1), Result::Bool(b2)) => {
                        number!(bool_to_f64(b1).$method(bool_to_f64(b2)))
                    }
                }
            }
        }
    };
}
impl_trait_op!(Result, Add, add);
impl_trait_op!(Result, Sub, sub);
impl_trait_op!(Result, Mul, mul);
impl_trait_op!(Result, Div, div);

/// Implement arithmetic operations for `Result` that return a float and have a symbol.
macro_rules! impl_float_op {
    ($struct:ident, $name:ident, $sym:tt) => {
        impl $struct {
            fn $name(self, other: Self) -> Self {
                match (self, other) {
                    (Result::Color(c1), Result::Color(c2)) => {
                        color!(c1.r $sym c2.r, c1.g $sym c2.g, c1.b $sym c2.b)
                    }
                    (Result::Color(c), Result::Number(n)) => {
                        color!(c.r $sym n, c.g $sym n, c.b $sym n)
                    }
                    (Result::Number(n), Result::Color(c)) => {
                        color!(n $sym c.r, n $sym c.g, n $sym c.b)
                    }
                    (Result::Number(n1), Result::Number(n2)) => number!(n1 $sym n2),
                    (Result::Color(c), Result::Bool(b)) => {
                        let b = bool_to_f64(b);
                        color!(c.r $sym b, c.g $sym b, c.b $sym b)
                    },
                    (Result::Number(n), Result::Bool(b)) => number!(n $sym bool_to_f64(b)),
                    (Result::Bool(b), Result::Color(c)) => {
                        let b = bool_to_f64(b);
                        color!(b $sym c.r, b $sym c.g, b $sym c.b)
                    },
                    (Result::Bool(b), Result::Number(n)) => number!(bool_to_f64(b) $sym n),
                    (Result::Bool(b1), Result::Bool(b2)) => {
                        let n = number!(bool_to_f64(b1) $sym bool_to_f64(b2));
                        bool!(n.as_bool())
                    }
                }
            }
        }
    }
}
impl_float_op!(Result, fmod, %);

/// Implement comparison operations for `Result`.
macro_rules! impl_bin_op {
    ($struct:ident, $name:ident, $sym:tt) => {
        impl $struct {
            fn $name(self, other: Self) -> Self {
                match (self, other) {
                    (Result::Color(c1), Result::Color(c2)) => {
                        let r = bool_to_f64(c1.r $sym c2.r);
                        let g = bool_to_f64(c1.g $sym c2.g);
                        let b = bool_to_f64(c1.b $sym c2.b);
                        color!(r, g, b)
                    }
                    (Result::Color(c), Result::Number(n)) => {
                        let r = bool_to_f64(c.r $sym n);
                        let g = bool_to_f64(c.g $sym n);
                        let b = bool_to_f64(c.b $sym n);
                        color!(r, g, b)
                    }
                    (Result::Number(n), Result::Color(c)) => {
                        let r = bool_to_f64(n $sym c.r);
                        let g = bool_to_f64(n $sym c.g);
                        let b = bool_to_f64(n $sym c.b);
                        color!(r, g, b)
                    }
                    (Result::Number(n1), Result::Number(n2)) => bool!(n1 $sym n2),
                    (Result::Color(c), Result::Bool(b)) => {
                        let b = bool_to_f64(b);
                        let r = bool_to_f64(c.r $sym b);
                        let g = bool_to_f64(c.g $sym b);
                        let b = bool_to_f64(c.b $sym b);
                        color!(r, g, b)
                    },
                    (Result::Number(n), Result::Bool(b)) => bool!(n $sym bool_to_f64(b)),
                    (Result::Bool(b), Result::Color(c)) => {
                        let b = bool_to_f64(b);
                        let r = bool_to_f64(b $sym c.r);
                        let g = bool_to_f64(b $sym c.g);
                        let b = bool_to_f64(b $sym c.b);
                        color!(r, g, b)
                    },
                    (Result::Bool(b), Result::Number(n)) => bool!(bool_to_f64(b) $sym n),
                    (Result::Bool(b1), Result::Bool(b2)) => {
                        let b1 = bool_to_f64(b1);
                        let b2 = bool_to_f64(b2);
                        bool!(b1 $sym b2)
                    }
                }
            }
        }
    }
}
impl_bin_op!(Result, less, <);
impl_bin_op!(Result, greater, >);

impl Result {
    /// Raise the result to a power.
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
            }
            (Result::Number(n), Result::Bool(b)) => number!(n.powf(bool_to_f64(b))),
            (Result::Bool(b), Result::Color(c)) => {
                let b = bool_to_f64(*b);
                color!(b.powf(c.r), b.powf(c.g), b.powf(c.b))
            }
            (Result::Bool(b), Result::Number(n)) => number!(bool_to_f64(*b).powf(n)),
            (Result::Bool(b1), Result::Bool(b2)) => bool!(b1 ^ b2),
        }
    }

    /// The absolute value.
    fn abs(&self) -> Self {
        match self {
            Result::Color(c) => {
                color!(c.r.abs(), c.g.abs(), c.b.abs())
            }
            Result::Number(n) => number!(n.abs()),
            Result::Bool(_) => number!(self.as_number().abs()),
        }
    }

    /// The OR operation.
    fn or(self, other: Self) -> Self {
        if let (Self::Bool(b1), Self::Bool(b2)) = (&self, &other) {
            return bool!(*b1 || *b2);
        }

        match self.partial_cmp(&other) {
            Some(Ordering::Less) => other.clone(),
            _ => self.clone(),
        }
    }

    // The AND operation.
    fn and(self, other: Self) -> Self {
        if let (Self::Bool(b1), Self::Bool(b2)) = (&self, &other) {
            return bool!(*b1 && *b2);
        }

        match self.partial_cmp(&other) {
            Some(Ordering::Greater) => other.clone(),
            _ => self.clone(),
        }
    }
}

fn wrap(x: f64) -> f64 {
    match x.is_sign_positive() {
        true => (x + 1.0) % 2.0 - 1.0,
        false => (x - 1.0) % 2.0 + 1.0,
    }
}

/// Evaluate an expression at a point.
pub(crate) fn eval_expr(expr: &Expr, x: f64, y: f64) -> Result {
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
                BinOp::Or => l.or(r),
                BinOp::And => l.and(r),
                BinOp::LessThan => l.less(r),
                BinOp::GreaterThan => l.greater(r),
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
        ExprKind::TransX(e) => {
            let offset = eval_expr(&e.trans, x, y).as_number();
            eval_expr(&e.inner, wrap(x - offset), y)
        }
        ExprKind::TransY(e) => {
            let offset = eval_expr(&e.trans, x, y).as_number();
            eval_expr(&e.inner, x, wrap(y - offset))
        }
        ExprKind::X => number!(x),
        ExprKind::Y => number!(y),
        ExprKind::R => number!(f64::sqrt(x * x + y * y)),
        ExprKind::A => number!(f64::atan(y / x) / PI),
        ExprKind::If(e) => {
            let cond = eval_expr(&e.cond, x, y);
            if cond.as_bool() {
                eval_expr(&e.true_expr, x, y)
            } else {
                eval_expr(&e.false_expr, x, y)
            }
        }
        ExprKind::Sin(e) => eval_expr(&e.inner, x, y).call(f64::sin),
        ExprKind::Cos(e) => eval_expr(&e.inner, x, y).call(f64::cos),
    };
    res.nan_to_zero();
    res
}

/// Evaluate an expression at a given point and return the clamped color.
pub fn eval(expr: &Expr, x: f64, y: f64) -> Color {
    let mut res = eval_expr(expr, x, y).as_color();
    res.clamp();
    res
}
