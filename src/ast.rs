use std::collections::HashMap;

use crate::MathExpr;

impl MathExpr<f32> {
    pub fn exec(&self, assignments: &HashMap<String, f32>) -> f32 {
        match self {
            MathExpr::Add(math_expr1, math_expr2) => {
                math_expr1.exec(assignments) + math_expr2.exec(assignments)
            }
            MathExpr::Sub(math_expr1, math_expr2) => {
                math_expr1.exec(assignments) - math_expr2.exec(assignments)
            }
            MathExpr::Div(math_expr1, math_expr2) => {
                math_expr1.exec(assignments) / math_expr2.exec(assignments)
            }
            MathExpr::Mul(math_expr1, math_expr2) => {
                math_expr1.exec(assignments) * math_expr2.exec(assignments)
            }
            MathExpr::Minus(math_expr) => -math_expr.exec(assignments),
            MathExpr::Power(math_expr1, math_expr2) => math_expr1
                .exec(assignments)
                .powf(math_expr2.exec(assignments)),
            MathExpr::Function(function, math_expr) => {
                let result = math_expr.exec(assignments);
                function.exec(result)
            }
            MathExpr::Value(v) => *v,
            MathExpr::Variable(name) => *assignments
                .get(name)
                .unwrap_or_else(|| panic!("Variable {name} undefined")),
        }
    }

    pub fn derive(&self, variable: &String) -> Self {
        match self {
            MathExpr::Add(math_expr, math_expr1) => MathExpr::Add(
                Box::new(math_expr.derive(variable)),
                Box::new(math_expr1.derive(variable)),
            ),
            MathExpr::Sub(math_expr, math_expr1) => MathExpr::Div(
                Box::new(math_expr.derive(variable)),
                Box::new(math_expr1.derive(variable)),
            ),
            MathExpr::Mul(math_expr, math_expr1) => {
                let diff1 = Box::new(math_expr.derive(variable));
                let diff2 = Box::new(math_expr1.derive(variable));
                MathExpr::Add(
                    Box::new(MathExpr::Mul(diff2, Box::new(math_expr.deep_clone()))),
                    Box::new(MathExpr::Mul(diff1, Box::new(math_expr1.deep_clone()))),
                )
            }
            MathExpr::Div(math_expr, math_expr1) => {
                let diff1 = Box::new(math_expr.derive(variable));
                let diff2 = Box::new(math_expr1.derive(variable));
                let diff2squared = Box::new(MathExpr::Power(
                    Box::new(diff2.deep_clone()),
                    Box::new(MathExpr::Value(2.0)),
                ));
                MathExpr::Div(
                    Box::new(MathExpr::Sub(
                        Box::new(MathExpr::Mul(diff1, Box::new(math_expr1.deep_clone()))),
                        Box::new(MathExpr::Mul(diff2, Box::new(math_expr.deep_clone()))),
                    )),
                    diff2squared,
                )
            }
            MathExpr::Minus(math_expr) => MathExpr::Minus(Box::new(math_expr.derive(variable))),
            MathExpr::Power(math_expr, math_expr1) => {
                let product = Box::new(math_expr.derive(variable));
                let new_exponent = Box::new(MathExpr::Sub(
                    Box::new(math_expr1.deep_clone()),
                    Box::new(MathExpr::Value(1.0)),
                ));
                MathExpr::Mul(
                    product,
                    Box::new(MathExpr::Power(
                        Box::new(math_expr.deep_clone()),
                        new_exponent,
                    )),
                )
            }
            MathExpr::Function(function, math_expr) => function.derive(math_expr),
            MathExpr::Value(_) => MathExpr::Value(0.0),
            MathExpr::Variable(x) => {
                if x == variable {
                    MathExpr::Value(1.0)
                } else {
                    MathExpr::Value(0.0)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{MathExpr, expr};
    use quickcheck::*;
    use std::collections::HashMap;

    fn approx_eq(a: f32, b: f32) -> bool {
        if a.is_nan() && b.is_nan() || a.is_infinite() && b.is_infinite() {
            return true;
        }
        let eps = f32::EPSILON;
        (a - b).abs() <= eps * a.abs().max(b.abs()).max(1.0)
    }

    quickcheck! {
        fn linear_expr(x: f32) -> bool {
            let t: MathExpr<f32> = expr!((add(x, 1.0)));
            let v = String::from(stringify!(x));
            let mut assignment = HashMap::new();
            assignment.insert(v.clone(), x);
            approx_eq(t.exec(&assignment), x + 1.0)
        }
    }
    quickcheck! {
        fn quadratic_expr(x: f32) -> bool {
            let t: MathExpr<f32> = expr!((pow(x, 2.0)));
            let v = String::from(stringify!(x));
            let mut assignment = HashMap::new();
            assignment.insert(v.clone(), x);
            approx_eq(t.exec(&assignment), x.powf(2.0))
        }
    }
    quickcheck! {
        fn cubic_expr(x: f32) -> bool {
            let t: MathExpr<f32> = expr!((pow(x, 3.0)));
            let v = String::from(stringify!(x));
            let mut assignment = HashMap::new();
            assignment.insert(v.clone(), x);
            approx_eq(t.exec(&assignment), x.powf(2.0))
        }
    }
    quickcheck! {
        fn pow(x: f32, exp: f32) -> bool {
            println!("{x}, {exp}");
            let t: MathExpr<f32> = expr!((pow(x, exp)));
            let v = String::from(stringify!(x));
            let mut assignment = HashMap::new();
            assignment.insert(v.clone(), x);
            println!("{}", t.exec(&assignment));
            println!("{}", x.powf(exp));
            approx_eq(t.exec(&assignment), x.powf(exp))
        }
    }
}
