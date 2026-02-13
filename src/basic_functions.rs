use std::fmt::Display;

use crate::{DerivableFunction, MathExpr};

#[derive(Debug, Clone)]
pub enum BasicFunctions {
    Exp,
    Ln,
    Log(f32),
    Sin,
    Cos,
    Sqrt,
}

impl Display for BasicFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
impl DerivableFunction<f32> for BasicFunctions {
    fn eval(&self, value: f32) -> f32 {
        match self {
            BasicFunctions::Exp => f32::exp(value),
            BasicFunctions::Ln => f32::ln(value),
            BasicFunctions::Log(n) => f32::log(value, *n),
            BasicFunctions::Sin => f32::sin(value),
            BasicFunctions::Cos => f32::cos(value),
            BasicFunctions::Sqrt => f32::sqrt(value),
        }
    }
    #[allow(unused)]
    fn derive(&self, math_expr: &MathExpr<f32>, variable: &str) -> MathExpr<f32> {
        match self {
            BasicFunctions::Exp => MathExpr::Mul(
                Box::new(math_expr.derive(variable)),
                Box::new(MathExpr::Function(
                    self.clone_box(),
                    Box::new(math_expr.clone()),
                )),
            ),
            BasicFunctions::Ln => MathExpr::Mul(
                Box::new(math_expr.derive(variable)),
                Box::new(MathExpr::Div(
                    Box::new(MathExpr::Value(1.0)),
                    Box::new(math_expr.clone()),
                )),
            ),
            BasicFunctions::Log(_) => todo!(),
            BasicFunctions::Sin => MathExpr::Mul(
                Box::new(math_expr.derive(variable)),
                Box::new(MathExpr::Function(
                    Box::new(BasicFunctions::Cos),
                    Box::new(math_expr.clone()),
                )),
            ),
            BasicFunctions::Cos => MathExpr::Mul(
                Box::new(math_expr.derive(variable)),
                Box::new(MathExpr::Minus(Box::new(MathExpr::Function(
                    Box::new(BasicFunctions::Cos),
                    Box::new(math_expr.clone()),
                )))),
            ),

            BasicFunctions::Sqrt => MathExpr::Div(
                Box::new(MathExpr::Value(1.0)),
                Box::new(MathExpr::Mul(
                    Box::new(MathExpr::Value(2.0)),
                    Box::new(MathExpr::Function(
                        self.clone_box(),
                        Box::new(math_expr.clone()),
                    )),
                )),
            ),
        }
    }

    fn clone_box(&self) -> Box<dyn DerivableFunction<f32>> {
        Box::new(self.clone())
    }
}
