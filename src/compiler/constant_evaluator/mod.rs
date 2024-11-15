mod tests;

use super::{
    ast::{
        AbsExpr, BinExpr, BinOp, ColorExpr, CosExpr, Expr, ExprKind, IfExpr, NegExpr, ParenExpr,
        SinExpr, TransXExpr, TransYExpr,
    },
    evaluator,
};

fn evaluate_constant_expr(expr: &Expr) -> Expr {
    let result = evaluator::eval_expr(expr, 0.0, 0.0);
    let expr = |kind| Expr {
        kind,
        span: expr.span.clone(),
    };
    match result {
        evaluator::Result::Color(c) => expr(ExprKind::Color(ColorExpr::new(
            expr(ExprKind::Number(c.r)),
            expr(ExprKind::Number(c.g)),
            expr(ExprKind::Number(c.b)),
        ))),
        evaluator::Result::Number(n) => expr(ExprKind::Number(n)),
        evaluator::Result::Bool(_) => expr(ExprKind::Number(result.as_number())),
    }
}

pub fn evaluate_constants(expr: Expr) -> Expr {
    match expr.kind {
        ExprKind::Bin(e) => {
            let mut lhs = evaluate_constants(*e.lhs);
            let mut rhs = evaluate_constants(*e.rhs);

            let mut l_const = false;
            let mut r_const = false;

            if lhs.is_constant() {
                l_const = true;
                lhs = evaluate_constant_expr(&lhs);
            }
            if rhs.is_constant() {
                r_const = true;
                rhs = evaluate_constant_expr(&rhs);
            }

            let expr = |kind: ExprKind| Expr {
                kind,
                span: expr.span.clone(),
            };

            if l_const && r_const {
                return evaluate_constant_expr(&expr(ExprKind::Bin(BinExpr::new(e.op, lhs, rhs))));
            }

            // Handle other constant cases
            match (&lhs.kind, &rhs.kind, &e.op) {
                (l, r, BinOp::Mul) if l.is_zero() || r.is_zero() => expr(ExprKind::Number(0.0)),
                (l, _, BinOp::Mul) if l.is_one() => rhs,
                (_, r, BinOp::Mul) if r.is_one() => lhs,

                (l, r, BinOp::Div) if l.is_zero() || r.is_zero() => expr(ExprKind::Number(0.0)),
                (_, r, BinOp::Div) if r.is_one() => lhs,
                (l, r, BinOp::Div) if r == l => expr(ExprKind::Number(1.0)),

                (l, r, BinOp::Mod) if l.is_zero() || r.is_zero() => expr(ExprKind::Number(0.0)),
                (l, r, BinOp::Mod) if r == l => expr(ExprKind::Number(0.0)),

                (l, _, BinOp::Pow) if l.is_zero() => expr(ExprKind::Number(0.0)),
                (l, _, BinOp::Pow) if l.is_one() => expr(ExprKind::Number(1.0)),
                (_, r, BinOp::Pow) if r.is_one() => lhs,
                (_, r, BinOp::Pow) if r.is_zero() => expr(ExprKind::Number(1.0)),

                (l, _, BinOp::Add) if l.is_zero() => rhs,
                (_, r, BinOp::Add) if r.is_zero() => lhs,

                (l, _, BinOp::Sub) if l.is_zero() => expr(ExprKind::Neg(NegExpr::new(rhs))),
                (_, r, BinOp::Sub) if r.is_zero() => lhs,

                (_, _, BinOp::Mul)
                | (_, _, BinOp::Div)
                | (_, _, BinOp::Mod)
                | (_, _, BinOp::Pow)
                | (_, _, BinOp::LessThan)
                | (_, _, BinOp::GreaterThan)
                | (_, _, BinOp::Or)
                | (_, _, BinOp::And)
                | (_, _, BinOp::Add)
                | (_, _, BinOp::Sub) => expr(ExprKind::Bin(BinExpr::new(e.op, lhs, rhs))),
            }
        }
        ExprKind::If(e) => {
            let cond = evaluate_constants(*e.cond);
            let true_expr = evaluate_constants(*e.true_expr);
            let false_expr = evaluate_constants(*e.false_expr);

            if cond.is_constant() {
                match evaluator::eval_expr(&cond, 0.0, 0.0).as_bool() {
                    true => true_expr,
                    false => false_expr,
                }
            } else {
                Expr {
                    kind: ExprKind::If(IfExpr::new(cond, true_expr, false_expr)),
                    span: expr.span,
                }
            }
        }
        ExprKind::TransX(e) => {
            let trans = evaluate_constants(*e.trans);
            let inner = evaluate_constants(*e.inner);

            match (&trans.kind, &inner.kind) {
                (t, _) if t.is_zero() => return inner,
                (_, i) if i.is_constant() => return inner,
                (ExprKind::X, ExprKind::X) => {
                    return Expr {
                        kind: ExprKind::Number(0.5),
                        span: expr.span,
                    }
                }
                _ => {}
            }

            let expr = |trans: Expr, inner: Expr| Expr {
                kind: ExprKind::TransX(TransXExpr::new(trans, inner)),
                span: expr.span,
            };

            match (trans.is_constant(), inner.is_constant()) {
                (true, true) => evaluate_constant_expr(&expr(trans, inner)),
                _ => expr(trans, inner),
            }
        }
        ExprKind::TransY(e) => {
            let trans = evaluate_constants(*e.trans);
            let inner = evaluate_constants(*e.inner);

            match (&trans.kind, &inner.kind) {
                (t, _) if t.is_zero() => return inner,
                (_, i) if i.is_constant() => return inner,
                (ExprKind::Y, ExprKind::A) | (ExprKind::Y, ExprKind::Y) => {
                    return Expr {
                        kind: ExprKind::Number(0.5),
                        span: expr.span,
                    }
                }
                _ => {}
            }

            let expr = |trans: Expr, inner: Expr| Expr {
                kind: ExprKind::TransY(TransYExpr::new(trans, inner)),
                span: expr.span,
            };

            match (trans.is_constant(), inner.is_constant()) {
                (true, true) => evaluate_constant_expr(&expr(trans, inner)),
                _ => expr(trans, inner),
            }
        }
        ExprKind::Neg(NegExpr { inner }) => {
            let inner = evaluate_constants(*inner);
            let is_const = inner.is_constant();
            let expr = Expr {
                kind: ExprKind::Neg(NegExpr::new(inner)),
                span: expr.span,
            };
            match is_const {
                true => evaluate_constant_expr(&expr),
                false => expr,
            }
        }
        ExprKind::Abs(AbsExpr { inner }) => {
            let inner = evaluate_constants(*inner);
            let is_const = inner.is_constant();
            let expr = Expr {
                kind: ExprKind::Abs(AbsExpr::new(inner)),
                span: expr.span,
            };
            match is_const {
                true => evaluate_constant_expr(&expr),
                false => expr,
            }
        }
        ExprKind::Sin(SinExpr { inner }) => {
            let inner = evaluate_constants(*inner);
            let is_const = inner.is_constant();
            let expr = Expr {
                kind: ExprKind::Sin(SinExpr::new(inner)),
                span: expr.span,
            };
            match is_const {
                true => evaluate_constant_expr(&expr),
                false => expr,
            }
        }
        ExprKind::Cos(CosExpr { inner }) => {
            let inner = evaluate_constants(*inner);
            let is_const = inner.is_constant();
            let expr = Expr {
                kind: ExprKind::Cos(CosExpr::new(inner)),
                span: expr.span,
            };
            match is_const {
                true => evaluate_constant_expr(&expr),
                false => expr,
            }
        }
        ExprKind::Paren(ParenExpr { inner }) => {
            let inner = evaluate_constants(*inner);
            let is_const = inner.is_constant();
            let expr = Expr {
                kind: ExprKind::Paren(ParenExpr::new(inner)),
                span: expr.span,
            };
            match is_const {
                true => evaluate_constant_expr(&expr),
                false => expr,
            }
        }
        ExprKind::Color(c) => {
            let mut r = evaluate_constants(*c.r);
            let mut g = evaluate_constants(*c.g);
            let mut b = evaluate_constants(*c.b);
            if r.is_constant() {
                r = evaluate_constant_expr(&r);
            }
            if g.is_constant() {
                g = evaluate_constant_expr(&g);
            }
            if b.is_constant() {
                b = evaluate_constant_expr(&b);
            }
            Expr {
                kind: ExprKind::Color(ColorExpr::new(r, g, b)),
                span: expr.span,
            }
        }
        ExprKind::Number(_) => expr,
        ExprKind::X => expr,
        ExprKind::Y => expr,
        ExprKind::R => expr,
        ExprKind::A => expr,
    }
}
