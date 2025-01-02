/// # Uninitialized variable
/// A variable associated with metadata that the user has input, but it hasn't yet been initialized
/// and therefore can't yet be used to evaluate expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct UninitializedVariable {
    /// The name of the variable.
    name: String,
}

impl UninitializedVariable {
    /// Create a new uninitialized variable.
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Get the name of the variable.
    pub fn name(&self) -> &str {
        &self.name
    }
}
