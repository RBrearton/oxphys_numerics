use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct LengthMismatchError {
    details: String,
}

impl LengthMismatchError {
    /// Create a new LengthMismatchError.
    /// # Arguments
    /// * `map` - A mapping from variable names to their lengths.
    pub(crate) fn new(map: HashMap<&str, usize>) -> LengthMismatchError {
        LengthMismatchError {
            details: format!(
                "The lengths of the variables in the expression do not match: {:?}",
                map
            ),
        }
    }
}

impl fmt::Display for LengthMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LengthMismatchError: {}", self.details)
    }
}

impl std::error::Error for LengthMismatchError {
    fn description(&self) -> &str {
        &self.details
    }
}
