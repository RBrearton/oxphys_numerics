use crate::traits::expression::Expression;

use super::{expr::Expr, scalar_type::ScalarType};

/// # BinaryNode
/// A node that has exactly two child nodes.
pub enum BinaryNode {
    Add(Box<Expr>, Box<Expr>),      // Binary operation: addition.
    Multiply(Box<Expr>, Box<Expr>), // Binary operation: multiplication.
    Pow(Box<Expr>, Box<Expr>),      // Binary operation: exponentiation.
    Log(Box<Expr>, Box<Expr>),      // Binary operation: logarithm.
}

impl Expression for BinaryNode {
    fn evaluate(&self, variables: &std::collections::HashMap<String, ScalarType>) -> ScalarType {
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

#[cfg(test)]
mod tests {

    use crate::enums::{leaf_node::LeafNode, scalar_type::ScalarType};

    use super::*;

    #[test]
    fn test_evaluate() {
        // Set up the variables hashmap.
        let mut variables = std::collections::HashMap::new();
        variables.insert("x".to_string(), ScalarType::F64(2.0));

        // f(x) = x + 1
        let expr = Expr::Binary(BinaryNode::Add(
            Box::new(Expr::Leaf(LeafNode::Variable("x".to_string()))),
            Box::new(Expr::Leaf(LeafNode::Constant(ScalarType::F64(1.0)))),
        ));
        assert_eq!(expr.evaluate(&variables), ScalarType::F64(3.0));

        // f(x) = x^2
        let expr = Expr::Binary(BinaryNode::Pow(
            Box::new(Expr::Leaf(LeafNode::Variable("x".to_string()))),
            Box::new(Expr::Leaf(LeafNode::Constant(ScalarType::F64(2.0)))),
        ));
        assert_eq!(expr.evaluate(&variables), ScalarType::F64(4.0));

        // f(x) = log(x, 2)
        let expr = Expr::Binary(BinaryNode::Log(
            Box::new(Expr::Leaf(LeafNode::Variable("x".to_string()))),
            Box::new(Expr::Leaf(LeafNode::Constant(ScalarType::F64(2.0)))),
        ));
        assert_eq!(expr.evaluate(&variables), ScalarType::F64(1.0));
    }
}
