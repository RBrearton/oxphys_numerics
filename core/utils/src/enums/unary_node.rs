use crate::traits::expression::Expression;

use super::expr::Expr;

/// # UnaryNode
/// A node that has exactly one child node.
pub enum UnaryNode {
    Negate(Box<Expr>), // Unary operation: negation.
    Sqrt(Box<Expr>),   // Unary operation: square root.
    Sin(Box<Expr>),    // Unary operation: sine.
    Cos(Box<Expr>),    // Unary operation: cosine.
    Exp(Box<Expr>),    // Unary operation: exponentiate.
    Ln(Box<Expr>),     // Unary operation: natural logarithm.
}

impl Expression for UnaryNode {
    fn evaluate(
        &self,
        variables: &std::collections::HashMap<String, super::scalar_type::ScalarType>,
    ) -> super::scalar_type::ScalarType {
        match self {
            UnaryNode::Negate(inner) => -inner.evaluate(variables),
            UnaryNode::Sqrt(inner) => inner.evaluate(variables).sqrt(),
            UnaryNode::Sin(inner) => inner.evaluate(variables).sin(),
            UnaryNode::Cos(inner) => inner.evaluate(variables).cos(),
            UnaryNode::Exp(inner) => inner.evaluate(variables).exp(),
            UnaryNode::Ln(inner) => inner.evaluate(variables).ln(),
        }
    }

    fn get_variables(&self) -> Vec<String> {
        match self {
            UnaryNode::Negate(inner)
            | UnaryNode::Sqrt(inner)
            | UnaryNode::Sin(inner)
            | UnaryNode::Cos(inner)
            | UnaryNode::Exp(inner)
            | UnaryNode::Ln(inner) => inner.get_variables(),
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

        // f(x) = -x
        let expr = Expr::Unary(UnaryNode::Negate(Box::new(Expr::Leaf(LeafNode::Variable(
            "x".to_string(),
        )))));
        assert_eq!(expr.evaluate(&variables), ScalarType::F64(-2.0));
    }
}
