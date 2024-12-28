use super::uninitialized_variable::UninitializedVariable;

/// # Initialized variable
/// A variable is considered to be initialized and ready to use when it's associated with an index.
/// This index is used to look up the value of the variable in the evaluation phase, where variables
/// are passed in as a vector. Similarly, when expressions are jit compiled, the index is associated
/// with the index of the parameter in the function signature.
#[derive(Debug, Clone, PartialEq)]
pub struct InitializedVariable {
    /// The underlying uninitialized variable.
    inner_variable: UninitializedVariable,

    /// The variable's index.
    pub index: usize,
}
