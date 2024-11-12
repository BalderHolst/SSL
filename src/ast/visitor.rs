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
            super::ExprKind::Number(e) => self.visit_number_expr(e),
            super::ExprKind::Color(e) => self.visit_color_expr(e),
            super::ExprKind::X => self.visit_x_expr(),
            super::ExprKind::Y => self.visit_y_expr(),
            super::ExprKind::If(e) => self.visit_if_expr(e),
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

    fn visit_if_expr(&mut self, expr: &super::IfExpr) {
        self.do_visit_if_expr(expr);
    }
    fn do_visit_if_expr(&mut self, expr: &super::IfExpr) {
        self.visit_expr(&expr.cond);
        self.visit_expr(&expr.true_expr);
        self.visit_expr(&expr.false_expr);
    }

    fn visit_number_expr(&mut self, expr: &super::NumberExpr);
    fn visit_color_expr(&mut self, expr: &super::ColorExpr);
    fn visit_x_expr(&mut self);
    fn visit_y_expr(&mut self);
}
