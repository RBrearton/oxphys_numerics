use super::{expr::Expr, initialized_expr::InitializedExpr, uninitialized_expr::UninitializedExpr};
use crate::traits::{
    expression::Expression, expression_compiler::ExpressionCompiler,
    expression_node::ExpressionNode,
};
use cranelift_codegen::ir::{InstBuilder, Value};

/// # BinaryNode
/// A node that has exactly two child nodes.
#[derive(Debug, Clone)]
pub enum BinaryNode {
    Add(Box<Expr>, Box<Expr>),      // Binary operation: addition.
    Subtract(Box<Expr>, Box<Expr>), // Binary operation: subtraction.
    Multiply(Box<Expr>, Box<Expr>), // Binary operation: multiplication.
    Frac(Box<Expr>, Box<Expr>),     // Binary operation: division.
    Pow(Box<Expr>, Box<Expr>),      // Binary operation: exponentiation.
    Log(Box<Expr>, Box<Expr>),      // Binary operation: logarithm.
}

impl ExpressionNode for BinaryNode {
    fn to_expr(&self, is_initialized: bool) -> Expr {
        match is_initialized {
            true => Expr::Initialized(InitializedExpr::Binary(self.clone())),
            false => Expr::Uninitialized(UninitializedExpr::Binary(self.clone())),
        }
    }
}

impl BinaryNode {
    /// # Left
    /// Get the left expression.
    fn left(&self) -> &Expr {
        match self {
            BinaryNode::Add(left, _) => left,
            BinaryNode::Subtract(left, _) => left,
            BinaryNode::Multiply(left, _) => left,
            BinaryNode::Frac(left, _) => left,
            BinaryNode::Pow(left, _) => left,
            BinaryNode::Log(left, _) => left,
        }
    }

    /// # Right
    /// Get the right expression.
    fn right(&self) -> &Expr {
        match self {
            BinaryNode::Add(_, right) => right,
            BinaryNode::Subtract(_, right) => right,
            BinaryNode::Multiply(_, right) => right,
            BinaryNode::Frac(_, right) => right,
            BinaryNode::Pow(_, right) => right,
            BinaryNode::Log(_, right) => right,
        }
    }

    fn expression_value(
        &self,
        builder: &mut cranelift_frontend::FunctionBuilder<'_>,
        left_value: Value,
        right_value: Value,
    ) -> Value {
        match self {
            BinaryNode::Add(_, _) => builder.ins().fadd(left_value, right_value),
            BinaryNode::Subtract(_, _) => builder.ins().fsub(left_value, right_value),
            BinaryNode::Multiply(_, _) => builder.ins().fmul(left_value, right_value),
            BinaryNode::Frac(_, _) => builder.ins().fdiv(left_value, right_value),
            _ => unimplemented!(),
        }
    }
}

impl ExpressionCompiler for BinaryNode {
    fn build_jit_nd(
        &self,
        builder: &mut cranelift_frontend::FunctionBuilder,
        parameters: &[Value],
    ) -> cranelift_codegen::ir::Value {
        // Start by building the left and right Values, then apply the binary operation.
        let left_value = self.left().build_jit_nd(builder, parameters);
        let right_value = self.right().build_jit_nd(builder, parameters);
        self.expression_value(builder, left_value, right_value)
    }

    fn build_jit_1d(
        &self,
        builder: &mut cranelift_frontend::FunctionBuilder,
        parameter: Value,
    ) -> Value {
        // Start by building the left and right Values, then apply the binary operation.
        let left_value = self.left().build_jit_1d(builder, parameter);
        let right_value = self.right().build_jit_1d(builder, parameter);
        self.expression_value(builder, left_value, right_value)
    }

    fn build_jit_2d(
        &self,
        builder: &mut cranelift_frontend::FunctionBuilder,
        param_0: Value,
        param_1: Value,
    ) -> Value {
        // Start by building the left and right Values, then apply the binary operation.
        let left_value = self.left().build_jit_2d(builder, param_0, param_1);
        let right_value = self.right().build_jit_2d(builder, param_0, param_1);
        self.expression_value(builder, left_value, right_value)
    }

    fn build_jit_3d(
        &self,
        builder: &mut cranelift_frontend::FunctionBuilder,
        param_0: Value,
        param_1: Value,
        param_2: Value,
    ) -> Value {
        // Start by building the left and right Values, then apply the binary operation.
        let left_value = self.left().build_jit_3d(builder, param_0, param_1, param_2);
        let right_value = self
            .right()
            .build_jit_3d(builder, param_0, param_1, param_2);
        self.expression_value(builder, left_value, right_value)
    }
}

impl Expression for BinaryNode {
    fn num_variables(&self) -> usize {
        self.left()
            .num_variables()
            .max(self.right().num_variables())
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::variable;

    use super::*;

    #[test]
    fn test_compiled_add() {
        let f = BinaryNode::Add(
            Box::new(variable("x".to_string()).to_expr()),
            Box::new(variable("y".to_string()).to_expr()),
        )
        .compile_nd()
        .unwrap();

        let values = vec![1.0, 2.0];
        assert_eq!(f(values.as_ptr(), values.len()), 3.0);
    }

    #[test]
    fn test_compiled_multiply() {
        let f = BinaryNode::Multiply(
            Box::new(variable("x".to_string()).to_expr()),
            Box::new(variable("y".to_string()).to_expr()),
        )
        .compile_nd()
        .unwrap();

        let values = vec![3.0, 4.0];
        assert_eq!(f(values.as_ptr(), values.len()), 12.0);
    }

    #[test]
    fn test_compiled_frac() {
        let f = BinaryNode::Frac(
            Box::new(variable("x".to_string()).to_expr()),
            Box::new(variable("y".to_string()).to_expr()),
        )
        .compile_nd()
        .unwrap();

        let values = vec![3.0, 4.0];
        assert_eq!(f(values.as_ptr(), values.len()), 0.75);
    }
}
