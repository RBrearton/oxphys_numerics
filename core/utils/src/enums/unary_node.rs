use crate::traits::expression::Expression;

/// # UnaryNode
/// A node that has exactly one child node.
pub enum UnaryNode<E: Expression> {
    Negate(E), // Unary operation: negation.
    Sqrt(E),   // Unary operation: square root.
    Sin(E),    // Unary operation: sine.
    Cos(E),    // Unary operation: cosine.
    Exp(E),    // Unary operation: exponentiate.
    Ln(E),     // Unary operation: natural logarithm.
}

impl<E: Expression> Expression for UnaryNode<E> {
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
