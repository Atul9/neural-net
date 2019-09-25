use std::fmt;

use super::{Expr, ExprImpl};

pub struct Exp {
    pub power: Expr,
}

impl ExprImpl for Exp {
    fn gradient(&self, v: &str) -> Expr {
        self.power.gradient(v) * Expr::new(Exp{
            power: self.power.clone(),
        })
    }

    fn eval(&self) -> ndarray::ArrayD<f32> {
        self.power.eval().mapv(|v| v.exp())
    }

    fn shape(&self) -> ndarray::IxDyn {
        self.power.shape()
    }

    fn is_constant(&self) -> bool {
        self.power.is_constant()
    }

    fn propagate_constants(&self) -> Expr {
        if self.is_constant() {
            super::expr(self.eval())
        } else {
            Expr::new(Self{
                power: self.power.propagate_constants(),
            })
        }
    }

    fn freeze_dx(&self, v: &str, i: &ndarray::IxDyn) -> Expr {
        Expr::new(Self{
            power: self.power.freeze_dx(v, i),
        })
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exp({})", self.power)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    use ndarray::Dimension;

    #[test]
    fn test() {
        let x = v("x", Rc::new(VariableValue::new(ndarray::arr0(0.0))));
        assert_eq!(format!("{}", (2.0 * x).exp().gradient_by_scalar("x", &ndarray::Ix0().into_dyn())), "(2 * exp((2 * x)))");
    }
}