use std::ops::{Add, Div, Mul, Neg, Sub};

use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

use crate::{
    errors::{
        expression_error::ExpressionError, length_mismatch_error::LengthMismatchError,
        no_variable_error::NoVariableError,
    },
    traits::{expression::Expression, expression_compiler::ExpressionCompiler},
};

use super::{
    binary_node::BinaryNode, expr::Expr, initialized_leaf::InitializedLeaf, unary_node::UnaryNode,
};

/// # InitializedExpr
/// An uninitialized expression. This represents a node in an `oxphys_numerics` expression tree, but
/// it can't yet be evaluated or compiled.
#[derive(Debug, Clone)]
pub enum InitializedExpr {
    Leaf(InitializedLeaf),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

impl Add for InitializedExpr {
    type Output = InitializedExpr;

    fn add(self, other: InitializedExpr) -> InitializedExpr {
        InitializedExpr::Binary(BinaryNode::Add(Box::new(self), Box::new(other)))
    }
}

impl Sub for InitializedExpr {
    type Output = InitializedExpr;

    fn sub(self, other: InitializedExpr) -> InitializedExpr {
        InitializedExpr::Binary(BinaryNode::Subtract(Box::new(self), Box::new(other)))
    }
}

impl Mul for InitializedExpr {
    type Output = InitializedExpr;

    fn mul(self, other: InitializedExpr) -> InitializedExpr {
        InitializedExpr::Binary(BinaryNode::Multiply(Box::new(self), Box::new(other)))
    }
}

impl Div for InitializedExpr {
    type Output = InitializedExpr;

    fn div(self, other: InitializedExpr) -> InitializedExpr {
        InitializedExpr::Binary(BinaryNode::Frac(Box::new(self), Box::new(other)))
    }
}

impl Neg for InitializedExpr {
    type Output = InitializedExpr;

    fn neg(self) -> InitializedExpr {
        InitializedExpr::Unary(UnaryNode::Negate(Box::new(self)))
    }
}

impl InitializedExpr {
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

impl ExpressionCompiler for InitializedExpr {
    fn build_jit_nd(&self, builder: &mut FunctionBuilder, parameters: &[Value]) -> Value {
        match self {
            InitializedExpr::Leaf(leaf) => leaf.build_jit_nd(builder, parameters),
            InitializedExpr::Unary(unary) => unary.build_jit_nd(builder, parameters),
            InitializedExpr::Binary(binary) => binary.build_jit_nd(builder, parameters),
        }
    }

    fn build_jit_1d(&self, builder: &mut FunctionBuilder, parameter: Value) -> Value {
        match self {
            InitializedExpr::Leaf(leaf) => leaf.build_jit_1d(builder, parameter),
            InitializedExpr::Unary(unary) => unary.build_jit_1d(builder, parameter),
            InitializedExpr::Binary(binary) => binary.build_jit_1d(builder, parameter),
        }
    }

    fn build_jit_2d(&self, builder: &mut FunctionBuilder, param_0: Value, param_1: Value) -> Value {
        match self {
            InitializedExpr::Leaf(leaf) => leaf.build_jit_2d(builder, param_0, param_1),
            InitializedExpr::Unary(unary) => unary.build_jit_2d(builder, param_0, param_1),
            InitializedExpr::Binary(binary) => binary.build_jit_2d(builder, param_0, param_1),
        }
    }

    fn build_jit_3d(
        &self,
        builder: &mut FunctionBuilder,
        param_0: Value,
        param_1: Value,
        param_2: Value,
    ) -> Value {
        match self {
            InitializedExpr::Leaf(leaf) => leaf.build_jit_3d(builder, param_0, param_1, param_2),
            InitializedExpr::Unary(unary) => unary.build_jit_3d(builder, param_0, param_1, param_2),
            InitializedExpr::Binary(binary) => {
                binary.build_jit_3d(builder, param_0, param_1, param_2)
            }
        }
    }
}

impl Expression for InitializedExpr {
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            InitializedExpr::Leaf(leaf) => leaf.evaluate(variables),
            InitializedExpr::Unary(unary) => unary.evaluate(variables),
            InitializedExpr::Binary(binary) => binary.evaluate(variables),
        }
    }

    fn num_variables(&self) -> usize {
        match self {
            InitializedExpr::Leaf(leaf) => leaf.num_variables(),
            InitializedExpr::Unary(unary) => unary.num_variables(),
            InitializedExpr::Binary(binary) => binary.num_variables(),
        }
    }
}
