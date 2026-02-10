#[macro_export]
macro_rules! expr {
    ((add($a:tt , $b:tt))) => {
        $crate::MathExpr::Add(Box::new(expr!($a)), Box::new(expr!($b)))
    };
    ((sub($a:tt , $b:tt))) => {
        $crate::MathExpr::Sub(Box::new(expr!($a)), Box::new(expr!($b)))
    };
    ((mul($a:tt , $b:tt))) => {
        $crate::MathExpr::Mul(Box::new(expr!($a)), Box::new(expr!($b)))
    };
    ((div($a:tt , $b:tt))) => {
        $crate::MathExpr::Div(Box::new(expr!($a)), Box::new(expr!($b)))
    };
    ((pow($a:tt , $b:tt))) => {
        $crate::MathExpr::Power(Box::new(expr!($a)), Box::new(expr!($b)))
    };
    ((neg($a:tt))) => {
        $crate::MathExpr::Minus(Box::new(expr!($a)))
    };
    ((exp($a:tt))) => {
        $crate::MathExpr::Function(Function::Exp, Box::new(expr!($a)))
    };
    ((ln($a:tt))) => {
        $crate::MathExpr::Function(Function::Ln, Box::new(expr!($a)))
    };
    ((sin($a:tt))) => {
        $crate::MathExpr::Function(Function::Sin, Box::new(expr!($a)))
    };
    ((cos($a:tt))) => {
        $crate::MathExpr::Function(Function::Cos, Box::new(expr!($a)))
    };
    ((sqrt($a:tt))) => {
        $crate::MathExpr::Function(Function::Sqrt, Box::new(expr!($a)))
    };
    ($v:literal) => {
        $crate::MathExpr::Value($v)
    };
    ($v:ident) => {
        $crate::MathExpr::Variable(stringify!($v).into())
    };
}
