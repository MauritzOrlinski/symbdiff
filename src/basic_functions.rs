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
    fn exec(&self, value: f32) -> f32 {
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
    fn derive(&self, math_expr: &MathExpr<f32>) -> MathExpr<f32> {
        match self {
            BasicFunctions::Exp => todo!(),
            BasicFunctions::Ln => todo!(),
            BasicFunctions::Log(_) => todo!(),
            BasicFunctions::Sin => todo!(),
            BasicFunctions::Cos => todo!(),
            BasicFunctions::Sqrt => todo!(),
        }
    }

    fn clone_box(&self) -> Box<dyn DerivableFunction<f32>> {
        Box::new(self.clone())
    }
}
