/// # Variable
/// A variable always has a name, and, once it has been initialized, it is also associated with an
/// index that is used to look up the value of the variable at the evaluation phase.
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    /// The name of the variable.
    name: String,

    /// The index of the variable in the jit compiled function signature's parameter list.
    index: Option<usize>,
}

impl Variable {
    /// Create a new uninitialized variable.
    pub fn new(name: String) -> Self {
        Self { name, index: None }
    }

    /// Get the name of the variable.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// # Initialize
    /// Returns a new variable that has been initialized with an index.
    ///
    /// This method takes a mutable reference to a vector of existing variable names. If a variable
    /// with the same name has already been initialized, then the new variable will have the same
    /// index as the existing variable. Otherwise, the new variable will have the next available
    /// index, and the name of the new variable will be added to the list of existing variable
    /// names.
    pub fn initialize(self, existing_variables: &mut Vec<String>) -> Self {
        // Try to find the variable in the list of existing variables.
        let index = match existing_variables.iter().position(|v| v == &self.name) {
            // If the variable is found, get the index.
            Some(index) => index,

            // If the variable is not found, add it to the list of existing variables and get the
            // new index.
            None => {
                existing_variables.push(self.name.clone());
                existing_variables.len() - 1
            }
        };

        // Return the initialized variable.
        Self {
            name: self.name,
            index: Some(index),
        }
    }

    /// # Is initialized
    /// Returns true if the variable has been initialized.
    pub fn is_initialized(&self) -> bool {
        self.index.is_some()
    }
}
