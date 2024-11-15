use super::Expr;

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) {
        self.do_visit_expr(expr);
    }
    fn do_visit_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            super::ExprKind::Bin(e) => self.visit_bin_expr(e),
            super::ExprKind::Paren(e) => self.visit_paren_expr(e),
            super::ExprKind::Neg(e) => self.visit_neg_expr(e),
            super::ExprKind::Abs(e) => self.visit_abs_expr(e),
            super::ExprKind::Sin(e) => self.visit_sin_expr(e),
            super::ExprKind::Cos(e) => self.visit_cos_expr(e),
            super::ExprKind::Number(e) => self.visit_number_expr(e),
            super::ExprKind::Color(e) => self.visit_color_expr(e),
            super::ExprKind::If(e) => self.visit_if_expr(e),
            super::ExprKind::TransX(e) => self.visit_trans_x_expr(e),
            super::ExprKind::TransY(e) => self.visit_trans_y_expr(e),
            super::ExprKind::X => self.visit_x_expr(),
            super::ExprKind::Y => self.visit_y_expr(),
            super::ExprKind::R => self.visit_r_expr(),
            super::ExprKind::A => self.visit_a_expr(),
        }
    }

    fn visit_bin_expr(&mut self, expr: &super::BinExpr) {
        self.do_visit_bin_expr(expr);
    }
    fn do_visit_bin_expr(&mut self, expr: &super::BinExpr) {
        self.visit_expr(&expr.lhs);
        self.visit_expr(&expr.rhs);
    }

    fn visit_paren_expr(&mut self, expr: &super::ParenExpr) {
        self.do_visit_paren_expr(expr);
    }
    fn do_visit_paren_expr(&mut self, expr: &super::ParenExpr) {
        self.visit_expr(&expr.inner)
    }

    fn visit_neg_expr(&mut self, expr: &super::NegExpr) {
        self.do_visit_neg_expr(expr);
    }
    fn do_visit_neg_expr(&mut self, expr: &super::NegExpr) {
        self.visit_expr(&expr.inner)
    }

    fn visit_abs_expr(&mut self, expr: &super::AbsExpr) {
        self.do_visit_abs_expr(expr);
    }
    fn do_visit_abs_expr(&mut self, expr: &super::AbsExpr) {
        self.visit_expr(&expr.inner)
    }

    fn visit_sin_expr(&mut self, expr: &super::SinExpr) {
        self.do_visit_sin_expr(expr);
    }
    fn do_visit_sin_expr(&mut self, expr: &super::SinExpr) {
        self.visit_expr(&expr.inner)
    }

    fn visit_cos_expr(&mut self, expr: &super::CosExpr) {
        self.do_visit_cos_expr(expr);
    }
    fn do_visit_cos_expr(&mut self, expr: &super::CosExpr) {
        self.visit_expr(&expr.inner)
    }

    fn visit_if_expr(&mut self, expr: &super::IfExpr) {
        self.do_visit_if_expr(expr);
    }
    fn do_visit_if_expr(&mut self, expr: &super::IfExpr) {
        self.visit_expr(&expr.cond);
        self.visit_expr(&expr.true_expr);
        self.visit_expr(&expr.false_expr);
    }

    fn visit_trans_x_expr(&mut self, expr: &super::TransXExpr) {
        self.do_visit_trans_x_expr(expr);
    }
    fn do_visit_trans_x_expr(&mut self, expr: &super::TransXExpr) {
        self.visit_expr(&expr.trans);
        self.visit_expr(&expr.inner);
    }

    fn visit_trans_y_expr(&mut self, expr: &super::TransYExpr) {
        self.do_visit_trans_y_expr(expr);
    }
    fn do_visit_trans_y_expr(&mut self, expr: &super::TransYExpr) {
        self.visit_expr(&expr.trans);
        self.visit_expr(&expr.inner);
    }

    fn visit_number_expr(&mut self, expr: &super::NumberExpr);
    fn visit_color_expr(&mut self, expr: &super::ColorExpr);
    fn visit_x_expr(&mut self);
    fn visit_y_expr(&mut self);
    fn visit_r_expr(&mut self);
    fn visit_a_expr(&mut self);
}
