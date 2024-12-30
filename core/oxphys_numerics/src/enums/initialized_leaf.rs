use crate::structs::initialized_variable::InitializedVariable;

/// # Initialized leaf
/// A leaf node that has been initialized, so it's associated with an index and can be used to
/// evaluate expressions.
#[derive(Debug, Clone)]
pub enum InitializedLeaf {
    Constant(f64),                 // Leaf node: a constant value.
    Variable(InitializedVariable), // The usize is the index of the variable in the input vector.
}
