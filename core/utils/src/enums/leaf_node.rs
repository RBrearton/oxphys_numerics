use crate::traits::expression::Expression;

/// # LeafExpr
/// An enum that represents the different types of leaf expressions that can be used in the
/// expression tree.
///
/// Being a leaf node, this node has no children.
pub enum LeafNode {
    Constant(f64), // Leaf node: a constant value.

    // The variable's value indicates its index in the variables vector.
    Variable(usize),
}

impl Expression for LeafNode {
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            LeafNode::Constant(value) => *value,
            LeafNode::Variable(idx) => variables[*idx],
        }
    }
}
