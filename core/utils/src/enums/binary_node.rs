use crate::traits::expression::Expression;

/// # BinaryNode
/// A node that has exactly two child nodes.
pub enum BinaryNode<E1: Expression, E2: Expression> {
    Add(E1, E2),      // Binary operation: addition.
    Multiply(E1, E2), // Binary operation: multiplication.
    Pow(E1, E2),      // Binary operation: exponentiation.
    Log(E1, E2),      // Binary operation: logarithm.
}

impl<E1: Expression, E2: Expression> Expression for BinaryNode<E1, E2> {
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
