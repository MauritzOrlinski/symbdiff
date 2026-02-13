//! # SymbDiff - A Symbolic differentiation library
//!
//! A small library for deriving mathematical functions of any amount of variables, using a symbolic approach.
//! The idea is to walk through the AST of the expression and apply differentiation rules.
//! Every mathematical function can be executed for a specific variable assignment.
pub(crate) mod ast;
pub mod basic_functions;
pub(crate) mod macros;
use std::fmt::Display;

/// An interface for defining mathematical functions yourself
/// We only allow maps of the form $f: T \to T$ for a generic set T
pub trait DerivableFunction<T: Clone>: Display {
    /// Evaluates the underlying function
    fn eval(&self, value: T) -> T;
    fn derive(&self, math_expr: &MathExpr<T>, variable: &str) -> MathExpr<T>;
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

impl<T> Clone for MathExpr<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Add(arg0, arg1) => Self::Add(arg0.clone(), arg1.clone()),
            Self::Sub(arg0, arg1) => Self::Sub(arg0.clone(), arg1.clone()),
            Self::Div(arg0, arg1) => Self::Div(arg0.clone(), arg1.clone()),
            Self::Mul(arg0, arg1) => Self::Mul(arg0.clone(), arg1.clone()),
            Self::Minus(arg0) => Self::Minus(arg0.clone()),
            Self::Power(arg0, arg1) => Self::Power(arg0.clone(), arg1.clone()),
            Self::Function(arg0, arg1) => Self::Function(arg0.clone_box(), arg1.clone()),
            Self::Value(arg0) => Self::Value(arg0.clone()),
            Self::Variable(arg0) => Self::Variable(arg0.clone()),
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
