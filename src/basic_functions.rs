use std::fmt::Display;

use crate::{DerivableFunction, MathExpr, VarId};

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
    fn derive(&self, math_expr: &MathExpr<f32>, variable: VarId) -> MathExpr<f32> {
        match self {
            BasicFunctions::Exp => MathExpr::mul(
                math_expr.derive(variable),
                MathExpr::function(self.clone(), math_expr.clone()),
            ),
            BasicFunctions::Ln => MathExpr::div(math_expr.derive(variable), math_expr.clone()),
            BasicFunctions::Log(b) => MathExpr::div(
                math_expr.derive(variable),
                MathExpr::mul(math_expr, MathExpr::lit(b.ln())),
            ),
            BasicFunctions::Sin => MathExpr::mul(
                math_expr.derive(variable),
                MathExpr::function(BasicFunctions::Cos, math_expr.clone()),
            ),
            BasicFunctions::Cos => MathExpr::mul(
                math_expr.derive(variable),
                -MathExpr::function(BasicFunctions::Sin, math_expr.clone()),
            ),
            BasicFunctions::Sqrt => MathExpr::mul(
                math_expr.derive(variable),
                MathExpr::div(
                    MathExpr::lit(1.0),
                    MathExpr::mul(
                        MathExpr::lit(2.0),
                        MathExpr::function(self.clone(), math_expr.clone()),
                    ),
                ),
            ),
        }
    }
}
