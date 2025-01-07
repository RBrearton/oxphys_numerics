use std::ops::{Add, Div, Mul, Neg, Sub};

use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

use crate::traits::{expression::Expression, expression_compiler::ExpressionCompiler};

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
        InitializedExpr::Binary(BinaryNode::Add(
            Box::new(self.to_expr()),
            Box::new(other.to_expr()),
        ))
    }
}

impl Sub for InitializedExpr {
    type Output = InitializedExpr;

    fn sub(self, other: InitializedExpr) -> InitializedExpr {
        InitializedExpr::Binary(BinaryNode::Subtract(
            Box::new(self.to_expr()),
            Box::new(other.to_expr()),
        ))
    }
}

impl Mul for InitializedExpr {
    type Output = InitializedExpr;

    fn mul(self, other: InitializedExpr) -> InitializedExpr {
        InitializedExpr::Binary(BinaryNode::Multiply(
            Box::new(self.to_expr()),
            Box::new(other.to_expr()),
        ))
    }
}

impl Div for InitializedExpr {
    type Output = InitializedExpr;

    fn div(self, other: InitializedExpr) -> InitializedExpr {
        InitializedExpr::Binary(BinaryNode::Frac(
            Box::new(self.to_expr()),
            Box::new(other.to_expr()),
        ))
    }
}

impl Neg for InitializedExpr {
    type Output = InitializedExpr;

    fn neg(self) -> InitializedExpr {
        InitializedExpr::Unary(UnaryNode::Negate(Box::new(self.to_expr())))
    }
}

impl InitializedExpr {
    /// # To expr
    /// Converts the UninitializedExpr to an Expr.
    pub fn to_expr(self) -> Expr {
        Expr::Initialized(self)
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
    fn num_variables(&self) -> usize {
        match self {
            InitializedExpr::Leaf(leaf) => leaf.num_variables(),
            InitializedExpr::Unary(unary) => unary.num_variables(),
            InitializedExpr::Binary(binary) => binary.num_variables(),
        }
    }
}
