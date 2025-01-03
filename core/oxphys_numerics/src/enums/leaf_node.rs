use super::{initialized_leaf::InitializedLeaf, uninitialized_leaf::UninitializedLeaf};

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
