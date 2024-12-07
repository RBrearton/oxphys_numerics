use std::{any::type_name, fmt};

#[derive(Debug)]
pub struct UnsupportedTypeError {
    details: String,
}

impl UnsupportedTypeError {
    /// Create a new UnsupportedTypeError.
    pub(crate) fn new<T>() -> UnsupportedTypeError {
        UnsupportedTypeError {
            details: format!("The type '{}' is not supported.", type_name::<T>()),
        }
    }
}

impl fmt::Display for UnsupportedTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnsupportedTypeError: {}", self.details)
    }
}

impl std::error::Error for UnsupportedTypeError {
    fn description(&self) -> &str {
        &self.details
    }
}
