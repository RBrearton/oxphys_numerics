use std::collections::HashMap;

use crate::traits::expression::Expression;

use super::{
    binary_node::BinaryNode, leaf_node::LeafNode, scalar_type::ScalarType, unary_node::UnaryNode,
};

/// Expression tree.
pub enum Expr {
    Leaf(LeafNode),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

impl Expression for Expr {
    fn evaluate(&self, variables: &HashMap<String, ScalarType>) -> ScalarType {
        match self {
            Expr::Leaf(leaf) => leaf.evaluate(variables),
            Expr::Unary(unary) => unary.evaluate(variables),
            Expr::Binary(binary) => binary.evaluate(variables),
        }
    }

    fn get_variables(&self) -> Vec<String> {
        match self {
            Expr::Leaf(leaf) => leaf.get_variables(),
            Expr::Unary(unary) => unary.get_variables(),
            Expr::Binary(binary) => binary.get_variables(),
        }
    }
}
