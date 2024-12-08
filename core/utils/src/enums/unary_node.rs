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
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            UnaryNode::Negate(inner) => -inner.evaluate(variables),
            UnaryNode::Sqrt(inner) => inner.evaluate(variables).sqrt(),
            UnaryNode::Sin(inner) => inner.evaluate(variables).sin(),
            UnaryNode::Cos(inner) => inner.evaluate(variables).cos(),
            UnaryNode::Exp(inner) => inner.evaluate(variables).exp(),
            UnaryNode::Ln(inner) => inner.evaluate(variables).ln(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::leaf_node::LeafNode;

    #[test]
    fn test_evaluate() {
        // Set up the variables hashmap.
        let variables = vec![1.0, 2.0, 3.0];

        // f(x) = -x
        let expr = UnaryNode::Negate(Box::new(Expr::Leaf(LeafNode::Variable(1))));
        assert_eq!(expr.evaluate(&variables), -2.0);
    }
}
