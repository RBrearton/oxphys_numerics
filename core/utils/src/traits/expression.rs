use std::collections::HashMap;

use crate::enums::scalar_type::ScalarType;

/// # Expression
/// This defines everything that we expect from our data structures that represent mathematical
/// expressions.
pub trait Expression {
    /// # Evaluate
    /// Run the expression with the given variables and return the result.
    fn evaluate(&self, variables: &HashMap<String, ScalarType>) -> ScalarType;

    /// # Get variables
    /// Returns a vector of the names of all the variables in the expression.
    fn get_variables(&self) -> Vec<String>;
}
