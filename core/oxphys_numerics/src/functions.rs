//! # Functions Module
//!
//! This module provides utility functions to create various types of expressions
//! for the `oxphys_numerics` expression tree. These functions help in constructing
//! expressions involving constants, variables, and various unary and binary operations.

use crate::enums::{
    binary_node::BinaryNode, expr::Expr, leaf_node::LeafNode, unary_node::UnaryNode,
};

/// # Constant
/// Returns an expression that is a constant leaf node in the expression tree.
pub fn constant(value: f64) -> Expr {
    Expr::Leaf(LeafNode::Constant(value))
}

/// # Variable
/// Returns an expression that is a variable leaf node in the expression tree.
/// You must provide the index of the variable; this is zero-indexed.
pub fn variable(index: usize) -> Expr {
    Expr::Leaf(LeafNode::Variable(index))
}

/// # Sqrt
/// Returns an expression that is the square root of the input expression.
pub fn sqrt(expr: Expr) -> Expr {
    Expr::Unary(UnaryNode::Sqrt(Box::new(expr)))
}

/// # Sin
/// Returns an expression that is the sine of the input expression.
pub fn sin(expr: Expr) -> Expr {
    Expr::Unary(UnaryNode::Sin(Box::new(expr)))
}

/// # Cos
/// Returns an expression that is the cosine of the input expression.
pub fn cos(expr: Expr) -> Expr {
    Expr::Unary(UnaryNode::Cos(Box::new(expr)))
}

/// # Exp
/// Returns an expression that is the e^(input expression).
pub fn exp(expr: Expr) -> Expr {
    Expr::Unary(UnaryNode::Exp(Box::new(expr)))
}

/// # Ln
/// Returns an expression that is the natural logarithm of the input expression.
pub fn ln(expr: Expr) -> Expr {
    Expr::Unary(UnaryNode::Ln(Box::new(expr)))
}

/// # Pow
/// Returns an expression that is the input expression raised to the power of the exponent
/// expression.
pub fn pow(base: Expr, exponent: Expr) -> Expr {
    Expr::Binary(BinaryNode::Pow(Box::new(base), Box::new(exponent)))
}

/// # Log
/// Returns an expression that is the logarithm of the input expression with a given base.
pub fn log(argument: Expr, base: Expr) -> Expr {
    Expr::Binary(BinaryNode::Log(Box::new(base), Box::new(argument)))
}
