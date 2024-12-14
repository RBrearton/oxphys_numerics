use super::expr::Expr;
use crate::traits::expression::Expression;
use cranelift_codegen::ir::InstBuilder;

/// # BinaryNode
/// A node that has exactly two child nodes.
pub enum BinaryNode {
    Add(Box<Expr>, Box<Expr>),      // Binary operation: addition.
    Multiply(Box<Expr>, Box<Expr>), // Binary operation: multiplication.
    Pow(Box<Expr>, Box<Expr>),      // Binary operation: exponentiation.
    Log(Box<Expr>, Box<Expr>),      // Binary operation: logarithm.
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
    ) -> cranelift_codegen::ir::Value {
        match self {
            BinaryNode::Add(left, right) => {
                let left = left.build_jit(builder);
                let right = right.build_jit(builder);
                builder.ins().fadd(left, right)
            }
            BinaryNode::Multiply(left, right) => {
                let left = left.build_jit(builder);
                let right = right.build_jit(builder);
                builder.ins().fmul(left, right)
            }
            _ => unimplemented!(),
        }
    }
}
