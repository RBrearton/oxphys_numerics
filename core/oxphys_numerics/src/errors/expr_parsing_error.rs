use std::fmt;

#[derive(Debug)]
pub struct ExprParsingError {
    details: String,
}

impl ExprParsingError {
    /// Create a new ExprParsingError.
    /// # Arguments
    /// * `map` - A mapping from variable names to their lengths.
    pub(crate) fn new(lengths: Vec<usize>) -> ExprParsingError {
        ExprParsingError {
            details: format!(
                "The lengths of the variables in the expression do not match: {:?}",
                lengths
            ),
        }
    }
}

impl fmt::Display for ExprParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExprParsingError: {}", self.details)
    }
}

impl std::error::Error for ExprParsingError {
    fn description(&self) -> &str {
        &self.details
    }
}
