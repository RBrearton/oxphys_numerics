use crate::structs::uninitialized_variable::UninitializedVariable;

/// # Uninitialized leaf
/// A leaf node associated with metadata that the user has input, but it hasn't yet been initialized
/// and therefore can't yet be used to evaluate expressions.
#[derive(Debug, Clone)]
pub enum UninitializedLeaf {
    Constant(f64),                   // Leaf node: a constant value.
    Variable(UninitializedVariable), // The usize is the index of the variable in the input vector.
}
