use crate::traits::expression::Expression;
use crate::traits::expression_compiler::ExpressionCompiler;
use cranelift_codegen::ir::{types, MemFlags};
use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_frontend::FunctionBuilder;

/// # LeafExpr
/// An enum that represents the different types of leaf expressions that can be used in the
/// expression tree.
///
/// Being a leaf node, this node has no children.
#[derive(Debug, Clone, Copy)]
pub enum LeafNode {
    Constant(f64),   // Leaf node: a constant value.
    Variable(usize), // The usize is the index of the variable in the input vector.
}

impl ExpressionCompiler for LeafNode {
    fn build_jit_nd(&self, builder: &mut FunctionBuilder, parameters: &[Value]) -> Value {
        match self {
            LeafNode::Constant(value) => builder.ins().f64const(*value),
            LeafNode::Variable(idx) => {
                let args_ptr = parameters[0]; // *const f64

                // We want to load the i-th argument (0-based index).
                let i = *idx;
                let arg_offset = (i * 8) as i32; // Each f64 is 8 bytes

                // Load the i-th argument from the arguments pointer.
                builder
                    .ins()
                    .load(types::F64, MemFlags::new(), args_ptr, arg_offset)
            }
        }
    }

    fn build_jit_1d(&self, builder: &mut FunctionBuilder, parameter: Value) -> Value {
        match self {
            LeafNode::Constant(value) => builder.ins().f64const(*value),
            LeafNode::Variable(_) => parameter,
        }
    }

    fn build_jit_2d(&self, builder: &mut FunctionBuilder, param_0: Value, param_1: Value) -> Value {
        match self {
            LeafNode::Constant(value) => builder.ins().f64const(*value),
            LeafNode::Variable(idx) => match idx {
                0 => param_0,
                1 => param_1,
                x => panic!(
                    "Invalid variable index for 2D expression. Expected 0 or 1, got {}",
                    x
                ),
            },
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
            LeafNode::Constant(value) => builder.ins().f64const(*value),
            LeafNode::Variable(idx) => match idx {
                0 => param_0,
                1 => param_1,
                2 => param_2,
                x => panic!(
                    "Invalid variable index for 3D expression. Expected 0, 1, or 2, got {}",
                    x
                ),
            },
        }
    }
}

impl Expression for LeafNode {
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            LeafNode::Constant(value) => *value,
            LeafNode::Variable(idx) => variables[*idx],
        }
    }

    fn num_variables(&self) -> usize {
        match self {
            LeafNode::Constant(_) => 0,
            LeafNode::Variable(idx) => *idx + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_1_variable() {
        let f = LeafNode::Variable(0).compile_nd().unwrap();
        // Create an array of f64 values.
        let values_1 = vec![1.0];
        let values_2 = vec![2.0];
        let values_3 = vec![3.0];

        assert_eq!(f(values_1.as_ptr(), values_1.len()), 1.0);
        assert_eq!(f(values_2.as_ptr(), values_2.len()), 2.0);
        assert_eq!(f(values_3.as_ptr(), values_3.len()), 3.0);
    }

    #[test]
    fn test_expression_constant() {
        let f = LeafNode::Constant(2.0).compile_nd().unwrap();
        let values = vec![];
        assert_eq!(f(values.as_ptr(), values.len()), 2.0);
    }

    #[test]
    fn test_evaluate_variable() {
        // Set up the variables hashmap.
        let variables = vec![1.0, 2.0, 3.0];

        // f(x) = x
        let expr = LeafNode::Variable(1);
        assert_eq!(expr.evaluate(&variables), 2.0);
    }

    #[test]
    fn test_evaluate_constant() {
        // Set up the variables hashmap.
        let variables = vec![1.0, 2.0, 3.0];

        // f(x) = 2
        let expr = LeafNode::Constant(2.0);
        assert_eq!(expr.evaluate(&variables), 2.0);
    }
}
