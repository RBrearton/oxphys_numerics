use std::collections::HashMap;

use crate::{
    errors::{
        expression_error::ExpressionError, length_mismatch_error::LengthMismatchError,
        no_variable_error::NoVariableError,
    },
    traits::expression::Expression,
};

use super::{
    binary_node::BinaryNode, leaf_node::LeafNode, scalar_type::ScalarType, unary_node::UnaryNode,
};

/// Expression tree.
pub enum Expr {
    Leaf(LeafNode),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

impl Expression for Expr {
    fn evaluate(&self, variables: &HashMap<String, ScalarType>) -> ScalarType {
        match self {
            Expr::Leaf(leaf) => leaf.evaluate(variables),
            Expr::Unary(unary) => unary.evaluate(variables),
            Expr::Binary(binary) => binary.evaluate(variables),
        }
    }

    fn get_variables(&self) -> Vec<String> {
        match self {
            Expr::Leaf(leaf) => leaf.get_variables(),
            Expr::Unary(unary) => unary.get_variables(),
            Expr::Binary(binary) => binary.get_variables(),
        }
    }
}

impl Expr {
    /// # Evaluate vec
    /// Evaluate the expression tree on vectors of input variables.
    pub fn evaluate_vec(
        &self,
        variables: &HashMap<String, Vec<ScalarType>>,
    ) -> Result<Vec<ScalarType>, ExpressionError> {
        // Get the lengths of the vectors we were given as a new hashmap mapping variable names to
        // their lengths.
        let lengths: HashMap<String, usize> = variables
            .iter()
            .map(|(name, values)| (name.clone(), values.len()))
            .collect();

        // Check that all the vectors have the same length.
        let mut lengths_iter = lengths.values();

        // Get the first length. If there are no lengths, return a NoVariable error.
        let first_length = match lengths_iter.next() {
            Some(length) => *length,
            None => return Err(ExpressionError::NoVariable(NoVariableError::new())),
        };

        // Check that all the lengths are the same.
        if lengths_iter.any(|length| *length != first_length) {
            return Err(ExpressionError::LengthMismatch(LengthMismatchError::new(
                lengths,
            )));
        }

        // Pre-allocate the result vector.
        let mut result = Vec::with_capacity(first_length);

        // Iterate over these variables and evaluate the expression for each row.
        for i in 0..first_length {
            // Create a hashmap that maps variable names to their values at this index.
            let mut variables_at_index = HashMap::new();

            // Fill in the values at this index.
            for (name, values) in variables {
                // Get the value at this index.
                variables_at_index.insert(name.clone(), values[i]);
            }

            // Evaluate the expression at this index.
            result.push(self.evaluate(&variables_at_index));
        }

        // Return the result.
        Ok(result)
    }
}
