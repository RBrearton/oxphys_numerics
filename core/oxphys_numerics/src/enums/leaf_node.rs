use super::{
    expr::Expr, initialized_expr::InitializedExpr, initialized_leaf::InitializedLeaf,
    uninitialized_expr::UninitializedExpr, uninitialized_leaf::UninitializedLeaf,
};

/// # LeafExpr
/// An enum that represents the different types of leaf expressions that can be used in the
/// expression tree.
///
/// Being a leaf node, this node has no children.
#[derive(Debug, Clone)]
pub enum LeafNode {
    Uninitialized(UninitializedLeaf), // An uninitialized leaf node.
    Initialized(InitializedLeaf),     // An initialized leaf node.
}

impl LeafNode {
    pub fn to_expr(self) -> Expr {
        match self {
            LeafNode::Uninitialized(uninitialized) => {
                Expr::Uninitialized(UninitializedExpr::Leaf(uninitialized))
            }
            LeafNode::Initialized(initialized) => {
                Expr::Initialized(InitializedExpr::Leaf(initialized))
            }
        }
    }
}
