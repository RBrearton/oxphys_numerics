use super::expr::Expr;
use crate::traits::expression::Expression;
use cranelift_codegen::ir::{InstBuilder, Value};

/// # BinaryNode
/// A node that has exactly two child nodes.
pub enum BinaryNode {
    Add(Box<Expr>, Box<Expr>),      // Binary operation: addition.
    Multiply(Box<Expr>, Box<Expr>), // Binary operation: multiplication.
    Pow(Box<Expr>, Box<Expr>),      // Binary operation: exponentiation.
    Log(Box<Expr>, Box<Expr>),      // Binary operation: logarithm.
}

impl BinaryNode {
    /// # Left
    /// Get the left expression.
    fn left(&self) -> &Expr {
        match self {
            BinaryNode::Add(left, _) => left,
            BinaryNode::Multiply(left, _) => left,
            BinaryNode::Pow(left, _) => left,
            BinaryNode::Log(left, _) => left,
        }
    }

    /// # Right
    /// Get the right expression.
    fn right(&self) -> &Expr {
        match self {
            BinaryNode::Add(_, right) => right,
            BinaryNode::Multiply(_, right) => right,
            BinaryNode::Pow(_, right) => right,
            BinaryNode::Log(_, right) => right,
        }
    }
}

impl Expression for BinaryNode {
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            BinaryNode::Add(left, right) => left.evaluate(variables) + right.evaluate(variables),
            BinaryNode::Multiply(left, right) => {
                left.evaluate(variables) * right.evaluate(variables)
            }
            BinaryNode::Pow(base, exponent) => {
                base.evaluate(variables).powf(exponent.evaluate(variables))
            }
            BinaryNode::Log(base, inner) => inner.evaluate(variables).log(base.evaluate(variables)),
        }
    }

    fn build_jit(
        &self,
        builder: &mut cranelift_frontend::FunctionBuilder,
        parameters: &[Value],
    ) -> cranelift_codegen::ir::Value {
        match self {
            BinaryNode::Add(left, right) => {
                let left = left.build_jit(builder, parameters);
                let right = right.build_jit(builder, parameters);
                builder.ins().fadd(left, right)
            }
            BinaryNode::Multiply(left, right) => {
                let left = left.build_jit(builder, parameters);
                let right = right.build_jit(builder, parameters);
                builder.ins().fmul(left, right)
            }
            _ => unimplemented!(),
        }
    }

    fn num_variables(&self) -> usize {
        self.left()
            .num_variables()
            .max(self.right().num_variables())
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::leaf_node::LeafNode;

    use super::*;

    #[test]
    fn test_compiled_add() {
        let f = BinaryNode::Add(
            Box::new(Expr::Leaf(LeafNode::Variable(0))),
            Box::new(Expr::Leaf(LeafNode::Variable(1))),
        )
        .compile()
        .unwrap();

        let values = vec![1.0, 2.0];
        assert_eq!(f(values.as_ptr(), values.len()), 3.0);
    }

    #[test]
    fn test_compiled_multiply() {
        let f = BinaryNode::Multiply(
            Box::new(Expr::Leaf(LeafNode::Variable(0))),
            Box::new(Expr::Leaf(LeafNode::Variable(1))),
        )
        .compile()
        .unwrap();

        let values = vec![3.0, 4.0];
        assert_eq!(f(values.as_ptr(), values.len()), 12.0);
    }

    #[test]
    fn test_evaluated_add() {
        let f = BinaryNode::Add(
            Box::new(Expr::Leaf(LeafNode::Variable(0))),
            Box::new(Expr::Leaf(LeafNode::Variable(1))),
        );

        let values = vec![1.0, 2.0];
        assert_eq!(f.evaluate(&values), 3.0);
    }

    #[test]
    fn test_evaluated_multiply() {
        let func_1 = BinaryNode::Multiply(
            Box::new(Expr::Leaf(LeafNode::Variable(0))),
            Box::new(Expr::Leaf(LeafNode::Variable(1))),
        );
        let func_2 = BinaryNode::Multiply(
            Box::new(Expr::Leaf(LeafNode::Variable(0))),
            Box::new(Expr::Leaf(LeafNode::Constant(2.))),
        );

        let values_1 = vec![3.0, 4.0];
        let values_2 = vec![3.0];
        assert_eq!(func_1.evaluate(&values_1), 12.0);
        assert_eq!(func_2.evaluate(&values_2), 6.0);
    }
}
