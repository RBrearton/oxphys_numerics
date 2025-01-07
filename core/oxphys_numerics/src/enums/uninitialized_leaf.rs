use crate::structs::uninitialized_variable::UninitializedVariable;

use super::{expr::Expr, initialized_leaf::InitializedLeaf, uninitialized_expr::UninitializedExpr};

/// # Uninitialized leaf
/// A leaf node associated with metadata that the user has input, but it hasn't yet been initialized
/// and therefore can't yet be used to evaluate expressions.
#[derive(Debug, Clone)]
pub enum UninitializedLeaf {
    Constant(f64),                   // Leaf node: a constant value.
    Variable(UninitializedVariable), // The usize is the index of the variable in the input vector.
}

impl UninitializedLeaf {
    /// # Initialize
    /// Returns an initialized leaf node.
    pub fn initialize(self, index: usize) -> InitializedLeaf {
        unimplemented!()
    }

    /// # New variable
    /// Create a new variable leaf node.
    pub fn new_variable(name: String) -> Self {
        UninitializedLeaf::Variable(UninitializedVariable::new(name))
    }

    /// # New constant
    /// Create a new constant leaf node.
    pub fn new_constant(value: f64) -> Self {
        UninitializedLeaf::Constant(value)
    }

    /// # To expr
    /// Convert the leaf node to an expression.
    pub fn to_expr(self) -> Expr {
        Expr::Uninitialized(UninitializedExpr::Leaf(self))
    }
}
