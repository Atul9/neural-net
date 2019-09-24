use std::fmt;

use super::{Expr, ExprImpl};

pub struct Exp {
    pub power: Expr,
}

impl ExprImpl for Exp {
    fn gradient(&self, v: &str, i: &ndarray::IxDyn) -> Expr {
        // TODO: matrix-by-scalar
        self.power.gradient(v, i) * Expr::new(Exp{
            power: self.power.clone(),
        })
    }

    fn eval(&self) -> ndarray::ArrayD<f32> {
        self.power.eval().mapv(|v| v.exp())
    }

    fn shape(&self) -> ndarray::IxDyn {
        self.power.shape()
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(e^{})", self.power)
    }
}
