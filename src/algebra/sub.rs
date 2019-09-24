use std::fmt;

use super::{Expr, ExprImpl};

pub struct Sub {
    pub left: Expr,
    pub right: Expr,
}

impl ExprImpl for Sub {
    fn gradient(&self, v: &str) -> Expr {
        self.left.gradient(v) - self.right.gradient(v)
    }

    fn eval(&self) -> f32 {
        self.left.eval() - self.right.eval()
    }
}

impl fmt::Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} - {})", self.left, self.right)
    }
}

impl<T: Into<Expr>> std::ops::Sub<T> for Expr {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Expr::new(Sub{
            left: self,
            right: rhs.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    use std::cell::Cell;

    #[test]
    fn test() {
        let x = v("x", Rc::new(Cell::new(0.0)));
        let y = v("y", Rc::new(Cell::new(0.0)));
        assert_eq!(format!("{}", (x - y).gradient("x")), "(1 - 0)");

        let x = v("x", Rc::new(Cell::new(0.0)));
        let y = v("y", Rc::new(Cell::new(0.0)));
        assert_eq!(format!("{}", (x - y).gradient("y")), "(0 - 1)");
    }
}
