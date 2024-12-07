use crate::traits::expression::Expression;

use super::expr::Expr;

/// # BinaryNode
/// A node that has exactly two child nodes.
pub enum BinaryNode {
    Add(Box<Expr>, Box<Expr>),      // Binary operation: addition.
    Multiply(Box<Expr>, Box<Expr>), // Binary operation: multiplication.
    Pow(Box<Expr>, Box<Expr>),      // Binary operation: exponentiation.
    Log(Box<Expr>, Box<Expr>),      // Binary operation: logarithm.
}

impl Expression for BinaryNode {
    fn evaluate(
        &self,
        variables: &std::collections::HashMap<String, super::scalar_type::ScalarType>,
    ) -> super::scalar_type::ScalarType {
        match self {
            BinaryNode::Add(left, right) => left.evaluate(variables) + right.evaluate(variables),
            BinaryNode::Multiply(left, right) => {
                left.evaluate(variables) * right.evaluate(variables)
            }
            BinaryNode::Pow(base, exponent) => {
                base.evaluate(variables).powf(exponent.evaluate(variables))
            }
            BinaryNode::Log(base, inner) => inner.evaluate(variables).log(base.evaluate(variables)),
        }
    }

    fn get_variables(&self) -> Vec<String> {
        match self {
            BinaryNode::Add(left, right)
            | BinaryNode::Multiply(left, right)
            | BinaryNode::Pow(left, right)
            | BinaryNode::Log(left, right) => {
                let mut variables = left.get_variables();
                variables.extend(right.get_variables());
                variables
            }
        }
    }
}
