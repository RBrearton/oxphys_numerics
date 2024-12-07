use super::{length_mismatch_error::LengthMismatchError, no_variable_error::NoVariableError};

/// # ExpressionError
/// All the errors that can be thrown when evaluating an expression.
pub enum ExpressionError {
    NoVariable(NoVariableError),
    LengthMismatch(LengthMismatchError),
}
