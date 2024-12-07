use crate::traits::expression::Expression;

use super::scalar_type::ScalarType;

/// # LeafExpr
/// An enum that represents the different types of leaf expressions that can be used in the
/// expression tree.
///
/// Being a leaf node, this node has no children.
pub enum LeafNode {
    Constant(ScalarType), // Leaf node: a constant value.
    Variable(String),     // Leaf node: a variable (e.g., "x").
}

impl Expression for LeafNode {
    fn evaluate(&self, variables: &std::collections::HashMap<String, ScalarType>) -> ScalarType {
        match self {
            LeafNode::Constant(value) => *value,
            LeafNode::Variable(name) => variables[name],
        }
    }

    fn get_variables(&self) -> Vec<String> {
        match self {
            LeafNode::Constant(_) => Vec::new(),
            LeafNode::Variable(name) => vec![name.clone()],
        }
    }
}
