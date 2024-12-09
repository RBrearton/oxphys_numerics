use crate::{
    errors::{
        expression_error::ExpressionError, length_mismatch_error::LengthMismatchError,
        no_variable_error::NoVariableError,
    },
    traits::expression::Expression,
};

use super::{binary_node::BinaryNode, leaf_node::LeafNode, unary_node::UnaryNode};

/// Expression tree.
pub enum Expr {
    Leaf(LeafNode),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

impl Expression for Expr {
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            Expr::Leaf(leaf) => leaf.evaluate(variables),
            Expr::Unary(unary) => unary.evaluate(variables),
            Expr::Binary(binary) => binary.evaluate(variables),
        }
    }
}

impl Expr {
    /// # Evaluate vec
    /// Evaluate the expression tree on vectors of input variables.
    pub fn evaluate_vec(&self, variables: &Vec<Vec<f64>>) -> Result<Vec<f64>, ExpressionError> {
        // Get the lengths of the vectors we were given as a new hashmap mapping variable names to
        // their lengths.
        let lengths: Vec<usize> = variables.iter().map(|values| (values.len())).collect();

        // Get the first length. If there are no lengths, return a NoVariable error.
        let first_length = match lengths.first() {
            Some(length) => *length,
            None => return Err(ExpressionError::NoVariable(NoVariableError::new())),
        };

        // Check that all the lengths are the same.
        if lengths.iter().any(|length| *length != first_length) {
            return Err(ExpressionError::LengthMismatch(LengthMismatchError::new(
                lengths,
            )));
        }

        // Now we know it's safe to do so, call evaluate on the expression tree for each element of
        // the variables vector.
        let result: Vec<f64> = variables
            .iter()
            .map(|values| self.evaluate(values))
            .collect();

        // Return the result.
        Ok(result)
    }
}
