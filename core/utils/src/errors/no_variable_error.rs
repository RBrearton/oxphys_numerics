use std::fmt;

#[derive(Debug)]
pub struct NoVariableError {
    details: String,
}

impl NoVariableError {
    /// Create a new LengthMismatchError.
    /// # Arguments
    /// * `map` - A mapping from variable names to their lengths.
    pub(crate) fn new() -> NoVariableError {
        let err_msg = concat!(
            "No variables were provided to the expression. Expressions can't be ",
            "made without variables."
        )
        .to_string();

        NoVariableError { details: err_msg }
    }
}

impl fmt::Display for NoVariableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LengthMismatchError: {}", self.details)
    }
}

impl std::error::Error for NoVariableError {
    fn description(&self) -> &str {
        &self.details
    }
}
