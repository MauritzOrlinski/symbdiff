//! # SymbDiff - A Symbolic differentiation library
//!
//! A small library for deriving mathematical functions of any amount of variables,
//! using symbolic differentiation.
//!
//! Expressions are shared through `Arc`, and variables are identified by compact IDs.
pub(crate) mod ast;
pub mod basic_functions;
pub(crate) mod macros;
pub mod symbols;

use std::{fmt::Display, ops, sync::Arc};
pub use symbols::{Env, SymbolTable, VarId};

/// An interface for defining mathematical functions yourself.
pub trait DerivableFunction<T: Clone>: Display {
    fn eval(&self, value: T) -> T;
    fn derive(&self, math_expr: &MathExpr<T>, variable: VarId) -> MathExpr<T>;
}

/// A shared handle to a symbolic expression node.
pub struct MathExpr<T: Clone + 'static> {
    pub(crate) node: Arc<ast::ExprNode<T>>,
}

impl<T: Clone + 'static> Clone for MathExpr<T> {
    fn clone(&self) -> Self {
        Self {
            node: Arc::clone(&self.node),
        }
    }
}

impl<T: Clone + 'static> MathExpr<T> {
    fn new(node: ast::ExprNode<T>) -> Self {
        Self {
            node: Arc::new(node),
        }
    }

    pub(crate) fn node(&self) -> &ast::ExprNode<T> {
        self.node.as_ref()
    }

    pub fn add(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Self::new(ast::ExprNode::Add(lhs.into(), rhs.into()))
    }

    pub fn sub(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Self::new(ast::ExprNode::Sub(lhs.into(), rhs.into()))
    }

    pub fn mul(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Self::new(ast::ExprNode::Mul(lhs.into(), rhs.into()))
    }

    pub fn div(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
        Self::new(ast::ExprNode::Div(lhs.into(), rhs.into()))
    }

    pub fn minus(expr: impl Into<Self>) -> Self {
        Self::new(ast::ExprNode::Minus(expr.into()))
    }

    pub fn pow(base: impl Into<Self>, exponent: impl Into<Self>) -> Self {
        Self::new(ast::ExprNode::Power(base.into(), exponent.into()))
    }

    pub fn function<F>(function: F, expr: impl Into<Self>) -> Self
    where
        F: DerivableFunction<T> + 'static,
    {
        Self::new(ast::ExprNode::Function(Arc::new(function), expr.into()))
    }

    pub fn lit(value: T) -> Self {
        Self::new(ast::ExprNode::Value(value))
    }

    pub fn var(value: VarId) -> Self {
        Self::new(ast::ExprNode::Variable(value))
    }

    pub fn symbol(symbols: &mut SymbolTable, name: impl AsRef<str>) -> Self {
        Self::var(symbols.intern(name))
    }

    pub fn display_with<'a>(&'a self, symbols: &'a SymbolTable) -> DisplayExpr<'a, T> {
        DisplayExpr {
            expr: self,
            symbols,
        }
    }
}

pub struct DisplayExpr<'a, T: Clone + 'static> {
    expr: &'a MathExpr<T>,
    symbols: &'a SymbolTable,
}

macro_rules! impl_binary_ops {
    ($trait:ident, $method:ident, $helper:ident) => {
        impl<T, Rhs> ops::$trait<Rhs> for MathExpr<T>
        where
            T: Clone + 'static,
            Rhs: Into<MathExpr<T>>,
        {
            type Output = MathExpr<T>;

            fn $method(self, rhs: Rhs) -> Self::Output {
                MathExpr::$helper(self, rhs)
            }
        }

        impl<'a, T, Rhs> ops::$trait<Rhs> for &'a MathExpr<T>
        where
            T: Clone + 'static,
            Rhs: Into<MathExpr<T>>,
        {
            type Output = MathExpr<T>;

            fn $method(self, rhs: Rhs) -> Self::Output {
                MathExpr::$helper(self.clone(), rhs)
            }
        }
    };
}

impl_binary_ops!(Add, add, add);
impl_binary_ops!(Sub, sub, sub);
impl_binary_ops!(Mul, mul, mul);
impl_binary_ops!(Div, div, div);

impl<T> ops::Neg for MathExpr<T>
where
    T: Clone + 'static,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        MathExpr::minus(self)
    }
}

impl<T> ops::Neg for &MathExpr<T>
where
    T: Clone + 'static,
{
    type Output = MathExpr<T>;

    fn neg(self) -> Self::Output {
        MathExpr::minus(self.clone())
    }
}

impl<'a, T> From<&'a MathExpr<T>> for MathExpr<T>
where
    T: Clone + 'static,
{
    fn from(value: &'a MathExpr<T>) -> Self {
        value.clone()
    }
}

impl From<f32> for MathExpr<f32> {
    fn from(value: f32) -> Self {
        Self::lit(value)
    }
}

impl From<VarId> for MathExpr<f32> {
    fn from(value: VarId) -> Self {
        Self::var(value)
    }
}

macro_rules! impl_var_binary_ops {
    ($trait:ident, $method:ident, $helper:ident) => {
        impl<Rhs> ops::$trait<Rhs> for VarId
        where
            Rhs: Into<MathExpr<f32>>,
        {
            type Output = MathExpr<f32>;

            fn $method(self, rhs: Rhs) -> Self::Output {
                MathExpr::$helper(self, rhs)
            }
        }
    };
}

impl_var_binary_ops!(Add, add, add);
impl_var_binary_ops!(Sub, sub, sub);
impl_var_binary_ops!(Mul, mul, mul);
impl_var_binary_ops!(Div, div, div);

impl ops::Neg for VarId {
    type Output = MathExpr<f32>;

    fn neg(self) -> Self::Output {
        MathExpr::minus(self)
    }
}

impl<T> Display for MathExpr<T>
where
    T: Display + Clone + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ast::ExprNode;

        match self.node() {
            ExprNode::Add(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} + {})", math_expr, math_expr1))
            }
            ExprNode::Sub(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} - {})", math_expr, math_expr1))
            }
            ExprNode::Div(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} / {})", math_expr, math_expr1))
            }
            ExprNode::Mul(math_expr, math_expr1) => {
                f.write_fmt(format_args!("({} * {})", math_expr, math_expr1))
            }
            ExprNode::Minus(math_expr) => f.write_fmt(format_args!("-({})", math_expr)),
            ExprNode::Power(math_expr, math_expr1) => {
                f.write_fmt(format_args!("{} ^ {}", math_expr, math_expr1))
            }
            ExprNode::Function(function, math_expr) => {
                f.write_fmt(format_args!("{}({})", function, math_expr))
            }
            ExprNode::Value(v) => f.write_str(v.to_string().as_str()),
            ExprNode::Variable(id) => f.write_fmt(format_args!("v{}", id.index())),
        }
    }
}

impl<'a, T> Display for DisplayExpr<'a, T>
where
    T: Display + Clone + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ast::ExprNode;

        match self.expr.node() {
            ExprNode::Add(math_expr, math_expr1) => f.write_fmt(format_args!(
                "({} + {})",
                math_expr.display_with(self.symbols),
                math_expr1.display_with(self.symbols)
            )),
            ExprNode::Sub(math_expr, math_expr1) => f.write_fmt(format_args!(
                "({} - {})",
                math_expr.display_with(self.symbols),
                math_expr1.display_with(self.symbols)
            )),
            ExprNode::Div(math_expr, math_expr1) => f.write_fmt(format_args!(
                "({} / {})",
                math_expr.display_with(self.symbols),
                math_expr1.display_with(self.symbols)
            )),
            ExprNode::Mul(math_expr, math_expr1) => f.write_fmt(format_args!(
                "({} * {})",
                math_expr.display_with(self.symbols),
                math_expr1.display_with(self.symbols)
            )),
            ExprNode::Minus(math_expr) => {
                f.write_fmt(format_args!("-({})", math_expr.display_with(self.symbols)))
            }
            ExprNode::Power(math_expr, math_expr1) => f.write_fmt(format_args!(
                "{} ^ {}",
                math_expr.display_with(self.symbols),
                math_expr1.display_with(self.symbols)
            )),
            ExprNode::Function(function, math_expr) => f.write_fmt(format_args!(
                "{}({})",
                function,
                math_expr.display_with(self.symbols)
            )),
            ExprNode::Value(v) => f.write_str(v.to_string().as_str()),
            ExprNode::Variable(id) => {
                if let Some(name) = self.symbols.resolve(*id) {
                    f.write_str(name)
                } else {
                    f.write_fmt(format_args!("v{}", id.index()))
                }
            }
        }
    }
}
