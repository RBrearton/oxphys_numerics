use crate::structs::{
    initialized_variable::InitializedVariable, uninitialized_variable::UninitializedVariable,
};

/// # Variable
/// This enum represents a variable in an expression.
/// Variables are associated with a specific index, which is used to look up the value of the
/// variable in the evaluation phase, where variables are passed in as a vector. This index is an
/// implementation detail, but instances of this enum are not yet associated with a specific
#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    /// A variable that has been initialized with a specific value.
    Initialized(InitializedVariable),

    /// A variable that has not been initialized with a specific value.
    Uninitialized(UninitializedVariable),
}
