use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::errors::expression_error::ExpressionError;

use super::{
    binary_node::BinaryNode, expr::Expr, unary_node::UnaryNode,
    uninitialized_leaf::UninitializedLeaf,
};

/// # UninitializedExpr
/// An uninitialized expression. This represents a node in an `oxphys_numerics` expression tree, but
/// it can't yet be evaluated or compiled.
#[derive(Debug, Clone)]
pub enum UninitializedExpr {
    Leaf(UninitializedLeaf),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

impl UninitializedExpr {
    /// # Initialize
    /// Returns an initialized Expr struct.
    pub fn initialize(self, index: usize) -> Result<Expr, ExpressionError> {
        unimplemented!()
    }
}

impl Add for UninitializedExpr {
    type Output = UninitializedExpr;

    fn add(self, other: UninitializedExpr) -> UninitializedExpr {
        UninitializedExpr::Binary(BinaryNode::Add(Box::new(self), Box::new(other)))
    }
}

impl Sub for UninitializedExpr {
    type Output = UninitializedExpr;

    fn sub(self, other: UninitializedExpr) -> UninitializedExpr {
        UninitializedExpr::Binary(BinaryNode::Subtract(Box::new(self), Box::new(other)))
    }
}

impl Mul for UninitializedExpr {
    type Output = UninitializedExpr;

    fn mul(self, other: UninitializedExpr) -> UninitializedExpr {
        UninitializedExpr::Binary(BinaryNode::Multiply(Box::new(self), Box::new(other)))
    }
}

impl Div for UninitializedExpr {
    type Output = UninitializedExpr;

    fn div(self, other: UninitializedExpr) -> UninitializedExpr {
        UninitializedExpr::Binary(BinaryNode::Frac(Box::new(self), Box::new(other)))
    }
}

impl Neg for UninitializedExpr {
    type Output = UninitializedExpr;

    fn neg(self) -> UninitializedExpr {
        UninitializedExpr::Unary(UnaryNode::Negate(Box::new(self)))
    }
}
