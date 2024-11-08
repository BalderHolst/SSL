use std::rc::Rc;

use super::{visitor::Visitor, Expr};

pub struct Printer {
    source: Rc<String>,
    level: usize,
}

impl Printer {
    const INDENTATION: usize = 2;

    pub fn new(source: Rc<String>) -> Self {
        Self { source, level: 0 }
    }

    /// Print a string at the current indentation.
    fn print(&self, text: &str) {
        println!("{}{}", " ".repeat(self.level * Self::INDENTATION), text);
    }

    /// Add one indentation level.
    fn indent(&mut self) {
        self.level += 1;
    }

    /// Remove one indentation level.
    fn unindent(&mut self) {
        self.level -= 1;
    }
}


macro_rules! vprintln {
    ($self:ident, $($arg:tt)*) => {
        $self.print(&format!($($arg)*))
    }
}

impl Visitor for Printer {
    fn visit_expr(&mut self, expr: &Expr) {
        vprintln!(self, "Expr \"{}\":", expr.span.slice(&self.source));
        self.indent();
        self.do_visit_expr(expr);
        self.unindent()
    }

    fn visit_bin_expr(&mut self, expr: &super::BinExpr) {
        vprintln!(self, "BinExpr:");
        self.indent();

        vprintln!(self, "operation: {:?}", expr.op);

        vprintln!(self, "left:");
        self.indent();
        self.visit_expr(&expr.lhs);
        self.unindent();

        vprintln!(self, "right:");
        self.indent();
        self.visit_expr(&expr.rhs);
        self.unindent();

        self.unindent();
    }

    fn visit_paren_expr(&mut self, expr: &super::ParenExpr) {
        vprintln!(self, "ParenExpr:");
        self.indent();
        self.do_visit_paren_expr(expr);
        self.unindent();
    }

    fn visit_neg_expr(&mut self, expr: &super::NegExpr) {
        vprintln!(self, "NegExpr:");
        self.indent();
        self.do_visit_neg_expr(expr);
        self.unindent();
    }

    fn visit_number_expr(&mut self, expr: &super::NumberExpr) {
        vprintln!(self, "NumberExpr: {:?}", expr)
    }

    fn visit_color_expr(&mut self, expr: &super::ColorExpr) {
        vprintln!(self, "ColorExpr:");
        self.indent();

        vprintln!(self, "R:");
        self.indent();
        self.visit_expr(&expr.r);
        self.unindent();

        vprintln!(self, "G:");
        self.indent();
        self.visit_expr(&expr.g);
        self.unindent();

        vprintln!(self, "B:");
        self.indent();
        self.visit_expr(&expr.b);
        self.unindent();

        self.unindent();
    }

    fn visit_x_expr(&mut self) {
        vprintln!(self, "X")
    }

    fn visit_y_expr(&mut self) {
        vprintln!(self, "Y")
    }
}
