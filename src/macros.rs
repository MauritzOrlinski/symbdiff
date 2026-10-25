#[macro_export]
macro_rules! expr {
    ($symbols:ident, (add($a:tt , $b:tt))) => {
        $crate::MathExpr::add(expr!($symbols, $a), expr!($symbols, $b))
    };
    ($symbols:ident, (sub($a:tt , $b:tt))) => {
        $crate::MathExpr::sub(expr!($symbols, $a), expr!($symbols, $b))
    };
    ($symbols:ident, (mul($a:tt , $b:tt))) => {
        $crate::MathExpr::mul(expr!($symbols, $a), expr!($symbols, $b))
    };
    ($symbols:ident, (div($a:tt , $b:tt))) => {
        $crate::MathExpr::div(expr!($symbols, $a), expr!($symbols, $b))
    };
    ($symbols:ident, (pow($a:tt , $b:tt))) => {
        $crate::MathExpr::pow(expr!($symbols, $a), expr!($symbols, $b))
    };
    ($symbols:ident, (neg($a:tt))) => {
        $crate::MathExpr::minus(expr!($symbols, $a))
    };
    ($symbols:ident, (exp($a:tt))) => {
        $crate::MathExpr::function(
            $crate::basic_functions::BasicFunctions::Exp,
            expr!($symbols, $a),
        )
    };
    ($symbols:ident, (ln($a:tt))) => {
        $crate::MathExpr::function(
            $crate::basic_functions::BasicFunctions::Ln,
            expr!($symbols, $a),
        )
    };
    ($symbols:ident, (sin($a:tt))) => {
        $crate::MathExpr::function(
            $crate::basic_functions::BasicFunctions::Sin,
            expr!($symbols, $a),
        )
    };
    ($symbols:ident, (cos($a:tt))) => {
        $crate::MathExpr::function(
            $crate::basic_functions::BasicFunctions::Cos,
            expr!($symbols, $a),
        )
    };
    ($symbols:ident, (sqrt($a:tt))) => {
        $crate::MathExpr::function(
            $crate::basic_functions::BasicFunctions::Sqrt,
            expr!($symbols, $a),
        )
    };
    ($symbols:ident, $v:literal) => {
        $crate::MathExpr::lit($v)
    };
    ($symbols:ident, $v:ident) => {
        $symbols.intern(stringify!($v))
    };
}
