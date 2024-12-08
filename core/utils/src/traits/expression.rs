/// # Expression
/// This defines everything that we expect from our data structures that represent mathematical
/// expressions.
pub trait Expression {
    /// # Evaluate
    /// Run the expression with the given variables and return the result.
    fn evaluate(&self, variables: &Vec<f64>) -> f64;
}
