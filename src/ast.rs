use std::sync::Arc;

use crate::{DerivableFunction, MathExpr, VarId, basic_functions::BasicFunctions};

pub(crate) enum ExprNode<T: Clone + 'static> {
    Add(MathExpr<T>, MathExpr<T>),
    Sub(MathExpr<T>, MathExpr<T>),
    Div(MathExpr<T>, MathExpr<T>),
    Mul(MathExpr<T>, MathExpr<T>),
    Minus(MathExpr<T>),
    Power(MathExpr<T>, MathExpr<T>),
    Function(Arc<dyn DerivableFunction<T>>, MathExpr<T>),
    Value(T),
    Variable(VarId),
}

impl MathExpr<f32> {
    pub fn eval(&self, env: &crate::Env<f32>) -> f32 {
        use ExprNode::*;

        match self.node() {
            Add(math_expr1, math_expr2) => math_expr1.eval(env) + math_expr2.eval(env),
            Sub(math_expr1, math_expr2) => math_expr1.eval(env) - math_expr2.eval(env),
            Div(math_expr1, math_expr2) => math_expr1.eval(env) / math_expr2.eval(env),
            Mul(math_expr1, math_expr2) => math_expr1.eval(env) * math_expr2.eval(env),
            Minus(math_expr) => -math_expr.eval(env),
            Power(math_expr1, math_expr2) => math_expr1.eval(env).powf(math_expr2.eval(env)),
            Function(function, math_expr) => function.eval(math_expr.eval(env)),
            Value(v) => *v,
            Variable(id) => *env
                .get(*id)
                .unwrap_or_else(|| panic!("Variable {:?} undefined", id)),
        }
    }

    pub fn exec(&self, env: &crate::Env<f32>) -> f32 {
        self.eval(env)
    }

    pub fn derive(&self, variable: VarId) -> Self {
        use ExprNode::*;

        match self.node() {
            Add(math_expr, math_expr1) => {
                MathExpr::add(math_expr.derive(variable), math_expr1.derive(variable))
            }
            Sub(math_expr, math_expr1) => {
                MathExpr::sub(math_expr.derive(variable), math_expr1.derive(variable))
            }
            Mul(math_expr, math_expr1) => MathExpr::add(
                MathExpr::mul(math_expr.derive(variable), math_expr1.clone()),
                MathExpr::mul(math_expr.clone(), math_expr1.derive(variable)),
            ),
            Div(math_expr, math_expr1) => {
                let numerator = MathExpr::sub(
                    MathExpr::mul(math_expr.derive(variable), math_expr1.clone()),
                    MathExpr::mul(math_expr1.derive(variable), math_expr.clone()),
                );
                let denominator = MathExpr::pow(math_expr1.clone(), MathExpr::lit(2.0));
                MathExpr::div(numerator, denominator)
            }
            Minus(math_expr) => MathExpr::minus(math_expr.derive(variable)),
            Power(base, exponent) => {
                let d_base = base.derive(variable);
                let d_exponent = exponent.derive(variable);
                if let Value(0.0) = d_exponent.node() {
                    let factor = MathExpr::mul(exponent.clone(), d_base);
                    let new_exponent = MathExpr::sub(exponent.clone(), MathExpr::lit(1.0));
                    return MathExpr::mul(factor, MathExpr::pow(base.clone(), new_exponent));
                }
                let ln_base = MathExpr::function(BasicFunctions::Ln, base.clone());
                let inner = MathExpr::add(
                    MathExpr::mul(d_exponent, ln_base),
                    MathExpr::div(MathExpr::mul(exponent.clone(), d_base), base.clone()),
                );
                MathExpr::mul(self.clone(), inner)
            }
            Function(function, math_expr) => function.derive(math_expr, variable),
            Value(_) => MathExpr::lit(0.0),
            Variable(x) => {
                if *x == variable {
                    MathExpr::lit(1.0)
                } else {
                    MathExpr::lit(0.0)
                }
            }
        }
    }

    pub fn differentiate(&self, variable: VarId) -> Self {
        self.derive(variable)
    }

    pub fn grad(&self) -> Vec<Self> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{Env, MathExpr, SymbolTable, expr};
    use quickcheck::*;

    // Due to testing for floats we use an approximate equality
    fn approx_eq(a: f32, b: f32) -> bool {
        approx_eq_rel(a, b, f32::EPSILON)
    }

    // Same as `approx_eq` but with a caller-chosen relative tolerance. Derivative
    // expressions are built from several operations (e.g. `3 * x^(3 - 1)`), so they
    // accumulate a few ulps of rounding compared to the hand-written closed form.
    fn approx_eq_rel(a: f32, b: f32, rel_tol: f32) -> bool {
        if a.is_nan() && b.is_nan() || a.is_infinite() && b.is_infinite() {
            return true;
        }
        (a - b).abs() <= rel_tol * a.abs().max(b.abs()).max(1.0)
    }

    fn derivative_at(expr: &MathExpr<f32>, x: crate::VarId, at: f32) -> f32 {
        let mut env = Env::new();
        env.set(x, at);
        expr.derive(x).eval(&env)
    }

    quickcheck! {
        fn linear_expr(x: f32) -> bool {
            let mut symbols = SymbolTable::new();
            let t: MathExpr<f32> = expr!(symbols, (add(x, 1.0)));
            let x_id = symbols.intern("x");
            let mut env = Env::new();
            env.set(x_id, x);
            approx_eq(t.eval(&env), x + 1.0)
        }
    }

    quickcheck! {
        fn quadratic_expr(x: f32) -> bool {
            let mut symbols = SymbolTable::new();
            let t: MathExpr<f32> = expr!(symbols, (pow(x, 2.0)));
            let x_id = symbols.intern("x");
            let mut env = Env::new();
            env.set(x_id, x);
            approx_eq(t.eval(&env), x.powf(2.0))
        }
    }

    quickcheck! {
        fn cubic_expr(x: f32) -> bool {
            let mut symbols = SymbolTable::new();
            let t: MathExpr<f32> = expr!(symbols, (pow(x, 3.0)));
            let x_id = symbols.intern("x");
            let mut env = Env::new();
            env.set(x_id, x);
            approx_eq(t.eval(&env), x.powf(3.0))
        }
    }

    quickcheck! {
        fn pow(x: f32, exp: f32) -> bool {
            let mut symbols = SymbolTable::new();
            let t: MathExpr<f32> = expr!(symbols, (pow(x, e)));
            let x_id = symbols.intern("x");
            let e_id = symbols.intern("e");
            let mut env = Env::new();
            env.set(x_id, x);
            env.set(e_id, exp);
            approx_eq(t.eval(&env), x.powf(exp))
        }
    }

    #[test]
    fn operator_overloads_share_nodes() {
        let mut symbols = SymbolTable::new();
        let x_id = symbols.intern("x");
        let y_id = symbols.intern("y");
        let expr = (y_id + x_id) * x_id + 2.0;

        let mut env = Env::new();
        env.set(x_id, 3.0);
        env.set(y_id, 4.0);

        assert!(approx_eq(expr.eval(&env), (4.0 + 3.0) * 3.0 + 2.0));
    }

    // ---- derivative tests -------------------------------------------------
    //
    // Each quickcheck below compares the symbolic derivative, evaluated at a random
    // point, against the closed-form derivative we know from calculus.
    //
    // Non-finite inputs are discarded: the unsimplified derivative contains terms
    // like `0 * x`, and `0 * inf` is NaN even though the closed form is not.

    const DERIV_TOL: f32 = 8.0 * f32::EPSILON;

    fn check(inputs: &[f32], property: impl FnOnce() -> bool) -> TestResult {
        if inputs.iter().any(|v| !v.is_finite()) {
            return TestResult::discard();
        }
        TestResult::from_bool(property())
    }

    #[test]
    fn derive_constant_is_zero() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let c: MathExpr<f32> = expr!(symbols, 5.0);
        assert_eq!(derivative_at(&c, x, 3.0), 0.0);
    }

    #[test]
    fn derive_variable_is_one() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let v = MathExpr::var(x);
        assert_eq!(derivative_at(&v, x, 3.0), 1.0);
    }

    #[test]
    fn derive_other_variable_is_zero() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");
        let v = MathExpr::var(y);
        // `y` is not set in the env; the derivative must be the literal 0 and not
        // need to look `y` up.
        assert_eq!(derivative_at(&v, x, 3.0), 0.0);
    }

    quickcheck! {
        // (x + 1)' = 1
        fn derive_sum_rule(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (add(x, 1.0)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), 1.0, DERIV_TOL)})
        }

        // (x * x)' = 2x
        fn derive_product_rule(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (mul(x, x)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), 2.0 * x, DERIV_TOL)})
        }

        // (1 / x)' = -1 / x^2
        fn derive_quotient_rule(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (div(1.0, x)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), -1.0 / (x * x), DERIV_TOL)})
        }

        // (x^3)' = 3x^2
        fn derive_power_rule(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (pow(x, 3.0)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), 3.0 * x * x, DERIV_TOL)})
        }

        // (-x)' = -1
        fn derive_negation(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (neg(x)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), -1.0, DERIV_TOL)})
        }

        // exp(2x)' = 2 exp(2x)   (chain rule through a function node)
        fn derive_chain_rule_exp(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (exp((mul(2.0, x)))));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), 2.0 * (2.0 * x).exp(), DERIV_TOL)})
        }

        // sin(x)' = cos(x), cos(x)' = -sin(x)
        fn derive_sin_cos(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let s: MathExpr<f32> = expr!(symbols, (sin(x)));
                let c: MathExpr<f32> = expr!(symbols, (cos(x)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&s, x_id, x), x.cos(), DERIV_TOL)
                    && approx_eq_rel(derivative_at(&c, x_id, x), -x.sin(), DERIV_TOL)})
        }

        // ln(x)' = 1 / x
        fn derive_ln(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (ln(x)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), 1.0 / x, DERIV_TOL)})
        }

        // sqrt(x)' = 1 / (2 sqrt(x))
        fn derive_sqrt(x: f32) -> TestResult {
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (sqrt(x)));
                let x_id = symbols.intern("x");
                approx_eq_rel(derivative_at(&t, x_id, x), 1.0 / (2.0 * x.sqrt()), DERIV_TOL)})
        }

        // Partial derivatives of f(x, y) = x*y + y:  df/dx = y,  df/dy = x + 1
        fn derive_partial(x: f32, y: f32) -> TestResult {
            check(&[x, y], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (add((mul(x, y)), y)));
                let x_id = symbols.intern("x");
                let y_id = symbols.intern("y");
                let mut env = Env::new();
                env.set(x_id, x);
                env.set(y_id, y);
                approx_eq_rel(t.derive(x_id).eval(&env), y, DERIV_TOL)
                    && approx_eq_rel(t.derive(y_id).eval(&env), x + 1.0, DERIV_TOL)})
        }
    }

    // A derivative that we do not want to write out by hand: compare against a
    // central finite difference instead. This is only a sanity check, so the
    // tolerance is loose and the points are chosen where the function is smooth.
    #[test]
    fn derive_matches_finite_difference() {
        let mut symbols = SymbolTable::new();
        // f(x) = sin(x^2) / (x + 3) + exp(cos(x))
        let f: MathExpr<f32> = expr!(
            symbols,
            (add((div((sin((pow(x, 2.0)))), (add(x, 3.0)))), (exp((cos(x))))))
        );
        let x = symbols.intern("x");
        let df = f.derive(x);

        let h = 1e-2_f32;
        for at in [-2.0_f32, -0.5, 0.3, 1.0, 2.5] {
            let mut env = Env::new();
            env.set(x, at + h);
            let f_plus = f.eval(&env);
            env.set(x, at - h);
            let f_minus = f.eval(&env);
            let numeric = (f_plus - f_minus) / (2.0 * h);

            env.set(x, at);
            let symbolic = df.eval(&env);

            assert!(
                approx_eq_rel(symbolic, numeric, 1e-2),
                "at x = {at}: symbolic {symbolic} vs numeric {numeric}"
            );
        }
    }

    // (2^x)' = ln(2) * 2^x   (exponent depends on the variable, base does not)
    #[test]
    fn derive_power_with_variable_exponent() {
        let mut symbols = SymbolTable::new();
        let t: MathExpr<f32> = expr!(symbols, (pow(2.0, x)));
        let x = symbols.intern("x");
        let at = 1.5_f32;
        let expected = std::f32::consts::LN_2 * 2.0_f32.powf(at);
        assert!(approx_eq_rel(derivative_at(&t, x, at), expected, DERIV_TOL));
    }

    quickcheck! {
        // (x^x)' = x^x * (ln x + 1), defined for x > 0
        fn derive_power_variable_base_and_exponent(x: f32) -> TestResult {
            if x.is_nan() || x <= 0.0 {
                return TestResult::discard();
            }
            check(&[x], || {
                let mut symbols = SymbolTable::new();
                let t: MathExpr<f32> = expr!(symbols, (pow(x, x)));
                let x_id = symbols.intern("x");
                let expected = x.powf(x) * (x.ln() + 1.0);
                approx_eq_rel(derivative_at(&t, x_id, x), expected, DERIV_TOL)
            })
        }
    }
}
