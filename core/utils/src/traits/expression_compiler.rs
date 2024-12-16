use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

/// # Expression compiler
/// This trait defines all the methods that need to be implemented to make an it possible to
/// jit-compile an expression.
pub(crate) trait ExpressionCompiler {
    /// # Build jit ND
    /// Given a jit function builder, add this expression to the function builder. This is called
    /// when we build an expression that takes an ND input (i.e. an array of f64s).
    fn build_jit_nd(&self, builder: &mut FunctionBuilder, parameters: &[Value]) -> Value;
}
