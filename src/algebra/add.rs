use std::fmt;

use super::{Expr, ExprImpl};

// Add performs element-wise addition.
pub struct Add {
    pub left: Expr,
    pub right: Expr,
}

impl ExprImpl for Add {
    fn gradient(&self, v: &str, i: &ndarray::IxDyn) -> Expr {
        // TODO: matrix-by-scalar
        self.left.gradient(v, i) + self.right.gradient(v, i)
    }

    fn eval(&self) -> ndarray::ArrayD<f32> {
        self.left.eval() + self.right.eval()
    }

    fn shape(&self) -> ndarray::IxDyn {
        self.left.shape()
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + {})", self.left, self.right)
    }
}

impl<T: Into<Expr>> std::ops::Add<T> for Expr {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Expr::new(Add{
            left: self,
            right: rhs.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    use ndarray::Dimension;

    #[test]
    fn test() {
        let x = v("x", Rc::new(VariableValue::new(ndarray::arr0(0.0))));
        let y = v("y", Rc::new(VariableValue::new(ndarray::arr0(0.0))));
        assert_eq!(format!("{}", (x + y).gradient("x", &ndarray::Ix0().into_dyn())), "(1 + 0)");

        let x = v("x", Rc::new(VariableValue::new(ndarray::arr0(0.0))));
        let y = v("y", Rc::new(VariableValue::new(ndarray::arr0(0.0))));
        assert_eq!(format!("{}", (x + y).gradient("y", &ndarray::Ix0().into_dyn())), "(0 + 1)");
    }
}
