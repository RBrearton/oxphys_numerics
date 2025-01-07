//! # Functions Module
//!
//! This module provides utility functions to create various types of expressions
//! for the `oxphys_numerics` expression tree. These functions help in constructing
//! expressions involving constants, variables, and various unary and binary operations.

use crate::{
    enums::{
        binary_node::BinaryNode, expr::Expr, unary_node::UnaryNode,
        uninitialized_expr::UninitializedExpr, uninitialized_leaf::UninitializedLeaf,
    },
    traits::expression_node::ExpressionNode,
};

/// # Constant
/// Returns an expression that is a constant leaf node in the expression tree.
/// You must specify whether you want to put the constant in an initialized or uninitialized
/// expression.
pub fn constant(value: f64) -> UninitializedExpr {
    UninitializedExpr::Leaf(UninitializedLeaf::Constant(value))
}

/// # Variable
/// Returns an expression that is a variable leaf node in the expression tree. This will always give
/// you an uninitialized expression.
pub fn variable(name: String) -> UninitializedExpr {
    UninitializedExpr::Leaf(UninitializedLeaf::new_variable(name))
}

/// # Sqrt
/// Returns an expression that is the square root of the input expression.
pub fn sqrt(expr: Expr) -> Expr {
    // Check to see whether we've been given an initialized or uninitialized expression.
    match expr {
        Expr::Initialized(_) => UnaryNode::Sqrt(Box::new(expr)).to_expr(true),
        Expr::Uninitialized(_) => UnaryNode::Sqrt(Box::new(expr)).to_expr(false),
    }
}

/// # Sin
/// Returns an expression that is the sine of the input expression.
pub fn sin(expr: Expr) -> Expr {
    match expr {
        Expr::Initialized(_) => UnaryNode::Sin(Box::new(expr)).to_expr(true),
        Expr::Uninitialized(_) => UnaryNode::Sin(Box::new(expr)).to_expr(false),
    }
}

/// # Cos
/// Returns an expression that is the cosine of the input expression.
pub fn cos(expr: Expr) -> Expr {
    match expr {
        Expr::Initialized(_) => UnaryNode::Cos(Box::new(expr)).to_expr(true),
        Expr::Uninitialized(_) => UnaryNode::Cos(Box::new(expr)).to_expr(false),
    }
}

/// # Exp
/// Returns an expression that is the e^(input expression).
pub fn exp(expr: Expr) -> Expr {
    match expr {
        Expr::Initialized(_) => UnaryNode::Exp(Box::new(expr)).to_expr(true),
        Expr::Uninitialized(_) => UnaryNode::Exp(Box::new(expr)).to_expr(false),
    }
}

/// # Ln
/// Returns an expression that is the natural logarithm of the input expression.
pub fn ln(expr: Expr) -> Expr {
    match expr {
        Expr::Initialized(_) => UnaryNode::Ln(Box::new(expr)).to_expr(true),
        Expr::Uninitialized(_) => UnaryNode::Ln(Box::new(expr)).to_expr(false),
    }
}

/// # Pow
/// Returns an expression that is the input expression raised to the power of the exponent
/// expression.
pub fn pow(base: Expr, exponent: Expr) -> Expr {
    match (base.clone(), exponent.clone()) {
        (Expr::Initialized(_), Expr::Initialized(_)) => {
            BinaryNode::Pow(Box::new(base), Box::new(exponent)).to_expr(true)
        }
        (Expr::Uninitialized(_), Expr::Initialized(_)) => {
            BinaryNode::Pow(Box::new(base), Box::new(exponent)).to_expr(false)
        }
        (Expr::Initialized(_), Expr::Uninitialized(_)) => {
            BinaryNode::Pow(Box::new(base), Box::new(exponent)).to_expr(false)
        }
        (Expr::Uninitialized(_), Expr::Uninitialized(_)) => {
            BinaryNode::Pow(Box::new(base), Box::new(exponent)).to_expr(false)
        }
    }
}

/// # Log
/// Returns an expression that is the logarithm of the input expression with a given base.
pub fn log(argument: Expr, base: Expr) -> Expr {
    match (argument.clone(), base.clone()) {
        (Expr::Initialized(_), Expr::Initialized(_)) => {
            BinaryNode::Log(Box::new(argument), Box::new(base)).to_expr(true)
        }
        (Expr::Uninitialized(_), Expr::Initialized(_)) => {
            BinaryNode::Log(Box::new(argument), Box::new(base)).to_expr(false)
        }
        (Expr::Initialized(_), Expr::Uninitialized(_)) => {
            BinaryNode::Log(Box::new(argument), Box::new(base)).to_expr(false)
        }
        (Expr::Uninitialized(_), Expr::Uninitialized(_)) => {
            BinaryNode::Log(Box::new(argument), Box::new(base)).to_expr(false)
        }
    }
}
