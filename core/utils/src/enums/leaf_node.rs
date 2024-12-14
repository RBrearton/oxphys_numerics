use crate::traits::expression::Expression;
use cranelift_codegen::ir::InstBuilder;

/// # LeafExpr
/// An enum that represents the different types of leaf expressions that can be used in the
/// expression tree.
///
/// Being a leaf node, this node has no children.
pub enum LeafNode {
    Constant(f64), // Leaf node: a constant value.
    Variable(usize),
}

impl Expression for LeafNode {
    fn evaluate(&self, variables: &Vec<f64>) -> f64 {
        match self {
            LeafNode::Constant(value) => *value,
            LeafNode::Variable(idx) => variables[*idx],
        }
    }

    fn build_jit(
        &self,
        builder: &mut cranelift_frontend::FunctionBuilder,
    ) -> cranelift_codegen::ir::Value {
        match self {
            LeafNode::Constant(value) => builder.ins().f64const(*value),
            LeafNode::Variable(idx) => {
                let entry_block = builder.create_block();
                builder.append_block_params_for_function_params(entry_block);
                builder.switch_to_block(entry_block);
                builder.seal_block(entry_block);

                builder.block_params(entry_block)[0]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let f = LeafNode::Constant(2.).compile().unwrap();
        assert_eq!(f(1.0), 2.0);
        assert_eq!(f(2.0), 2.0);
        assert_eq!(f(3.0), 2.0);
    }
}
