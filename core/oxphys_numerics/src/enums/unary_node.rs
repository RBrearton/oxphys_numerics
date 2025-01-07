use crate::traits::expression::Expression;
use crate::traits::expression_compiler::ExpressionCompiler;
use crate::traits::expression_node::ExpressionNode;
use cranelift_codegen::ir::InstBuilder;
use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

use super::expr::Expr;
use super::initialized_expr::InitializedExpr;
use super::uninitialized_expr::UninitializedExpr;

/// # UnaryNode
/// A node that has exactly one child node.
#[derive(Debug, Clone)]
pub enum UnaryNode {
    Negate(Box<Expr>), // Unary operation: negation.
    Sqrt(Box<Expr>),   // Unary operation: square root.
    Sin(Box<Expr>),    // Unary operation: sine.
    Cos(Box<Expr>),    // Unary operation: cosine.
    Exp(Box<Expr>),    // Unary operation: exponentiate.
    Ln(Box<Expr>),     // Unary operation: natural logarithm.
}

impl UnaryNode {
    /// # Inner
    /// Get the inner expression.
    fn inner(&self) -> &Expr {
        match self {
            UnaryNode::Negate(inner) => inner,
            UnaryNode::Sqrt(inner) => inner,
            UnaryNode::Sin(inner) => inner,
            UnaryNode::Cos(inner) => inner,
            UnaryNode::Exp(inner) => inner,
            UnaryNode::Ln(inner) => inner,
        }
    }

    fn expression_value(&self, builder: &mut FunctionBuilder<'_>, input: Value) -> Value {
        match self {
            UnaryNode::Negate(_) => builder.ins().fneg(input),
            UnaryNode::Sqrt(_) => builder.ins().sqrt(input),
            _ => unimplemented!(),
        }
    }
}

impl ExpressionNode for UnaryNode {
    fn to_expr(&self, is_initialized: bool) -> Expr {
        match is_initialized {
            true => Expr::Initialized(InitializedExpr::Unary(self.clone())),
            false => Expr::Uninitialized(UninitializedExpr::Unary(self.clone())),
        }
    }
}

impl ExpressionCompiler for UnaryNode {
    fn build_jit_nd(&self, builder: &mut FunctionBuilder, parameters: &[Value]) -> Value {
        // Start by building the inner expression, then apply the unary operation.
        let input = self.inner().build_jit_nd(builder, parameters);
        self.expression_value(builder, input)
    }

    fn build_jit_1d(&self, builder: &mut FunctionBuilder, parameter: Value) -> Value {
        // Start by building the inner expression, then apply the unary operation.
        let input = self.inner().build_jit_1d(builder, parameter);
        self.expression_value(builder, input)
    }

    fn build_jit_2d(&self, builder: &mut FunctionBuilder, param_0: Value, param_1: Value) -> Value {
        // Start by building the inner expression, then apply the unary operation.
        let input = self.inner().build_jit_2d(builder, param_0, param_1);
        self.expression_value(builder, input)
    }

    fn build_jit_3d(
        &self,
        builder: &mut FunctionBuilder,
        param_0: Value,
        param_1: Value,
        param_2: Value,
    ) -> Value {
        // Start by building the inner expression, then apply the unary operation.
        let input = self
            .inner()
            .build_jit_3d(builder, param_0, param_1, param_2);
        self.expression_value(builder, input)
    }
}

impl Expression for UnaryNode {
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            UnaryNode::Negate(inner) => -inner.evaluate(variables),
            UnaryNode::Sqrt(inner) => inner.evaluate(variables).sqrt(),
            UnaryNode::Sin(inner) => inner.evaluate(variables).sin(),
            UnaryNode::Cos(inner) => inner.evaluate(variables).cos(),
            UnaryNode::Exp(inner) => inner.evaluate(variables).exp(),
            UnaryNode::Ln(inner) => inner.evaluate(variables).ln(),
        }
    }

    fn num_variables(&self) -> usize {
        self.inner().num_variables()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::leaf_node::LeafNode;

    #[test]
    fn test_evaluate() {
        // Set up the variables vector.
        let variables = vec![1.0, 2.0, 3.0];

        // f(x) = -x
        let expr = UnaryNode::Negate(Box::new(Expr::Initialized(LeafNode::Variable(1))));
        assert_eq!(expr.evaluate(&variables), -2.0);
    }

    #[test]
    fn test_compiled_negate() {
        // Set up the variables vector.
        let variables = vec![15.0];

        // f(x) = -x
        let expr = UnaryNode::Negate(Box::new(Expr::Leaf(LeafNode::Variable(0))));
        let f = expr.compile_nd().unwrap();

        assert_eq!(f(variables.as_ptr(), variables.len()), -15.0);
    }

    #[test]
    fn test_compiled_sqrt() {
        // Set up the variables vector.
        let variables = vec![16.0];

        // f(x) = sqrt(x)
        let expr = UnaryNode::Sqrt(Box::new(Expr::Leaf(LeafNode::Variable(0))));
        let f = expr.compile_nd().unwrap();

        assert_eq!(f(variables.as_ptr(), variables.len()), 4.0);
    }
}
