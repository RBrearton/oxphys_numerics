use crate::traits::expression::Expression;
use cranelift_codegen::ir::InstBuilder;
use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

use super::expr::Expr;

/// # UnaryNode
/// A node that has exactly one child node.
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

    fn build_jit(&self, builder: &mut FunctionBuilder) -> Value {
        // Start by building the inner expression.
        let input = self.inner().build_jit(builder);

        match self {
            UnaryNode::Negate(_) => builder.ins().fneg(input),
            UnaryNode::Sqrt(_) => builder.ins().sqrt(input),
            _ => unimplemented!(),
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
        // Set up the variables hashmap.
        let variables = vec![1.0, 2.0, 3.0];

        // f(x) = -x
        let expr = UnaryNode::Negate(Box::new(Expr::Leaf(LeafNode::Variable(1))));
        assert_eq!(expr.evaluate(&variables), -2.0);
    }
}
