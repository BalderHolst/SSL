use crate::{
    ast::{AbsExpr, BinExpr, BinOp, ColorExpr, Expr, ExprKind, IfExpr, ParenExpr},
    text::Span,
};

pub fn cornelia_expr(span: Span) -> Expr {
    let expr = |kind: ExprKind| Expr {
        kind,
        span: span.clone(),
    };

    expr(ExprKind::Bin(BinExpr::new(
        BinOp::Sub,
        expr(ExprKind::Bin(BinExpr::new(
            BinOp::Mul,
            expr(ExprKind::Bin(BinExpr::new(
                BinOp::Or,
                expr(ExprKind::Bin(BinExpr::new(
                    BinOp::Mul,
                    expr(ExprKind::Paren(ParenExpr::new(expr(ExprKind::Bin(
                        BinExpr::new(
                            BinOp::LessThan,
                            expr(ExprKind::Abs(AbsExpr::new(expr(ExprKind::Bin(
                                // ((|x| - 0.25)^2 + (y + 0.3 )^2)^0.5
                                BinExpr::new(
                                    BinOp::Pow,
                                    expr(ExprKind::Bin(BinExpr::new(
                                        BinOp::Add,
                                        // (|x| - 0.25)^2
                                        expr(ExprKind::Bin(BinExpr::new(
                                            BinOp::Pow,
                                            expr(ExprKind::Paren(ParenExpr::new(expr(
                                                ExprKind::Bin(BinExpr::new(
                                                    BinOp::Sub,
                                                    expr(ExprKind::Abs(AbsExpr::new(expr(
                                                        ExprKind::X,
                                                    )))),
                                                    expr(ExprKind::Number(0.25)),
                                                )),
                                            )))),
                                            expr(ExprKind::Number(2.0)),
                                        ))),
                                        // (y + 0.3 )^2
                                        expr(ExprKind::Bin(BinExpr::new(
                                            BinOp::Pow,
                                            expr(ExprKind::Paren(ParenExpr::new(expr(
                                                ExprKind::Bin(BinExpr::new(
                                                    BinOp::Add,
                                                    expr(ExprKind::Y),
                                                    expr(ExprKind::Number(0.30)),
                                                )),
                                            )))),
                                            expr(ExprKind::Number(2.0)),
                                        ))),
                                    ))),
                                    expr(ExprKind::Number(0.5)),
                                ),
                            ))))),
                            expr(ExprKind::Number(0.3)),
                        ),
                    ))))),
                    expr(ExprKind::Number(1.0)),
                ))),
                expr(ExprKind::If(IfExpr::new(
                    expr(ExprKind::Bin(BinExpr::new(
                        BinOp::GreaterThan,
                        expr(ExprKind::Bin(BinExpr::new(
                            BinOp::Add,
                            expr(ExprKind::Y),
                            expr(ExprKind::Number(0.133)),
                        ))),
                        expr(ExprKind::Number(0.0)),
                    ))),
                    expr(ExprKind::Bin(BinExpr::new(
                        BinOp::LessThan,
                        expr(ExprKind::Bin(BinExpr::new(
                            BinOp::Add,
                            expr(ExprKind::Abs(AbsExpr::new(expr(ExprKind::X)))),
                            // |(y + 0.133) * 0.6|
                            expr(ExprKind::Abs(AbsExpr::new(expr(ExprKind::Bin(
                                BinExpr::new(
                                    BinOp::Mul,
                                    expr(ExprKind::Bin(BinExpr::new(
                                        BinOp::Add,
                                        expr(ExprKind::Y),
                                        expr(ExprKind::Number(0.133)),
                                    ))),
                                    expr(ExprKind::Number(0.6)),
                                ),
                            ))))),
                        ))),
                        expr(ExprKind::Number(0.5)),
                    ))),
                    expr(ExprKind::Number(-1.0)),
                ))),
            ))),
            expr(ExprKind::Color(ColorExpr::new(
                expr(ExprKind::Number(10.0)),
                expr(ExprKind::Number(0.0)),
                expr(ExprKind::Number(0.0)),
            ))),
        ))),
        expr(ExprKind::Color(ColorExpr::new(
            expr(ExprKind::Number(0.0)),
            expr(ExprKind::Number(1.0)),
            expr(ExprKind::Number(1.0)),
        ))),
    )))
}
