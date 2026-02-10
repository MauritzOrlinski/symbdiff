/// A Symbolic differentiation library
pub(crate) mod ast;
pub mod basic_functions;
pub(crate) mod macros;
use std::fmt::Display;

pub trait DerivableFunction<T: Clone>: Display {
    fn exec(&self, value: T) -> T;
    fn derive(&self, math_expr: &MathExpr<T>) -> MathExpr<T>;
    fn clone_box(&self) -> Box<dyn DerivableFunction<T>>;
}

pub enum MathExpr<T: Clone> {
    Add(Box<MathExpr<T>>, Box<MathExpr<T>>),
    Sub(Box<MathExpr<T>>, Box<MathExpr<T>>),
    Div(Box<MathExpr<T>>, Box<MathExpr<T>>),
    Mul(Box<MathExpr<T>>, Box<MathExpr<T>>),
    Minus(Box<MathExpr<T>>),
    Power(Box<MathExpr<T>>, Box<MathExpr<T>>),
    Function(Box<dyn DerivableFunction<T>>, Box<MathExpr<T>>),
    Value(T),
    Variable(String),
}

impl<T> MathExpr<T>
where
    T: Clone,
{
    pub fn deep_clone(&self) -> Self {
        match self {
            MathExpr::Add(a, b) => {
                MathExpr::Add(Box::new(a.deep_clone()), Box::new(b.deep_clone()))
            }
            MathExpr::Sub(a, b) => {
                MathExpr::Sub(Box::new(a.deep_clone()), Box::new(b.deep_clone()))
            }
            MathExpr::Mul(a, b) => {
                MathExpr::Mul(Box::new(a.deep_clone()), Box::new(b.deep_clone()))
            }
            MathExpr::Div(a, b) => {
                MathExpr::Div(Box::new(a.deep_clone()), Box::new(b.deep_clone()))
            }
            MathExpr::Minus(a) => MathExpr::Minus(Box::new(a.deep_clone())),
            MathExpr::Power(a, b) => {
                MathExpr::Power(Box::new(a.deep_clone()), Box::new(b.deep_clone()))
            }
            MathExpr::Function(f, arg) => {
                MathExpr::Function(f.clone_box(), Box::new(arg.deep_clone()))
            }
            MathExpr::Value(v) => MathExpr::Value(v.clone()),
            MathExpr::Variable(s) => MathExpr::Variable(s.clone()),
        }
    }
}

impl<T> Display for MathExpr<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathExpr::Add(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} + {})", math_expr, math_expr1))
            }
            MathExpr::Sub(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} - {})", math_expr, math_expr1))
            }
            MathExpr::Div(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} / {})", math_expr, math_expr1))
            }
            MathExpr::Mul(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} * {})", math_expr, math_expr1))
            }
            MathExpr::Minus(math_expr) => f.write_fmt(format_args!(" -({})", math_expr)),
            MathExpr::Power(math_expr, math_expr1) => {
                f.write_fmt(format_args!("{} ^ {}", math_expr, math_expr1,))
            }
            MathExpr::Function(function, math_expr) => {
                f.write_fmt(format_args!("{}({})", function, math_expr))
            }
            MathExpr::Value(v) => f.write_str(v.to_string().as_str()),
            MathExpr::Variable(x) => f.write_str(x.as_str()),
        }
    }
}
